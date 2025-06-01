import httpx
from bs4 import BeautifulSoup
from bs4.element import Tag
import asyncio
import csv

# List of URLs to scrape
URLS = [
    "https://www.2manybootlegs.com/radio-shows/hank-the-dj/hank-the-dj-1/",
    "https://www.2manybootlegs.com/radio-shows/hank-the-dj/hank-the-dj-2/",
    # Add more URLs as needed
]


async def fetch_html(client, url):
    try:
        response = await client.get(url)
        response.raise_for_status()
        return response.text
    except httpx.HTTPError as e:
        print(f"Error fetching {url}: {e}")
        return ""


def parse_tracklist(html_content, show_url):
    soup = BeautifulSoup(html_content, "html.parser")
    tracklist_div = soup.find("div", class_="tracklist")
    if not isinstance(tracklist_div, Tag):  # Ensure it's a Tag
        print(f"Tracklist 'div' is not a Tag or not found in {show_url}")
        return []
    # tracklist_div is now known to be a Tag for the type checker

    tracks = []
    rows = tracklist_div.find_all("tr")  # This call is on a Tag
    for row in rows:
        if not isinstance(row, Tag):  # Ensure row is a Tag
            continue
        # row is now known to be a Tag by the type checker

        td = row.find("td", attrs={"colspan": "2"})  # Correct colspan and call on Tag
        if not isinstance(td, Tag):  # Ensure td is a Tag
            continue
        # td is now known to be a Tag by the type checker

        # Check for mashup
        # These calls are now on a Tag, so 'class_' should be fine, and methods are known.
        mash_title = td.find("span", class_="mashtitle")
        mash_tracks = td.find_all("span", class_="mashtrack")
        if mash_title and mash_tracks:
            mashup_name = mash_title.get_text(strip=True)
            for mt in mash_tracks:
                text = mt.get_text(strip=True)
                artist, title = split_artist_and_title(text)
                tracks.append(
                    {
                        "show_url": show_url,
                        "is_mashup": True,
                        "mashup_name": mashup_name,
                        "artist": artist,
                        "track_title": title,
                    }
                )
        else:
            text = td.get_text(strip=True)
            if "–" in text:
                artist, title = map(str.strip, text.split("–", 1))
            elif "-" in text:
                artist, title = map(str.strip, text.split("-", 1))
            else:
                artist, title = "", text
            tracks.append(
                {
                    "show_url": show_url,
                    "is_mashup": False,
                    "mashup_name": "",
                    "artist": artist,
                    "track_title": title,
                }
            )
    return tracks


async def main():
    all_tracks = []
    async with httpx.AsyncClient() as client:
        tasks = [fetch_html(client, url) for url in URLS]
        html_pages = await asyncio.gather(*tasks)
        for html, url in zip(html_pages, URLS):
            if html:
                tracks = parse_tracklist(html, url)
                all_tracks.extend(tracks)

    # Write to CSV
    with open("2manybootlegs_tracks.csv", "w", newline="", encoding="utf-8") as csvfile:
        fieldnames = ["show_url", "is_mashup", "mashup_name", "artist", "track_title"]
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        for track in all_tracks:
            writer.writerow(track)

    print(f"Extracted {len(all_tracks)} tracks.")


# Run the script
if __name__ == "__main__":
    asyncio.run(main())
