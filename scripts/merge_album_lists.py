import csv
import os
import re

# Define file paths relative to the project root (404zen/)
# Assumes the script is run from the 404zen directory, e.g., `python scripts/merge_album_lists.py`
CANON_FILE = "data/tracklists/canon_electronic_artists_albums.csv"
HECK_FILE = "data/tracklists/HeckTheDJ_albums.csv"
OUTPUT_DIR = "data/processed"
OUTPUT_FILE = os.path.join(OUTPUT_DIR, "merged_album_list.csv")

# Column names in input files
CANON_ARTIST_COL = "Artist Name"
CANON_ALBUM_COL = "Seminal Electronic Album"
HECK_ARTIST_COL = "Artist Name"
HECK_ALBUM_COL = "Best-Known Album"

# Output column names
OUTPUT_ARTIST_COL = "artist_name"
OUTPUT_ALBUM_COL = "album_title"

# Manual suggestions for artists who might be filtered out or have non-album entries.
# These are intended to be their "most awesome" or critically acclaimed studio albums,
# or highly influential compilations that serve a similar role.
# These will be added only if the artist isn't already present with a valid album from the CSV files.
MANUAL_ALBUM_SUGGESTIONS = {
    "Kiki": "Kaos",
    "Kiko": "Slave of My Mind",
    "Liquid Liquid": "Liquid Liquid",  # Seminal compilation of their EPs
    "Ray Mang": "Ray Mang",  # Debut studio album
    "T99": "Children of Chaos",
    "DJ Falcon": "Hello My Name Is DJ Falcon",  # Often used for compilations of his EPs
    "Maurice Fulton": "Life Is Water",
    "Linus Loves": "Stage Invader",
    "Max Berlin": "Elle Et Moi",
    "Jess & Crabbe": "The Big Booya",
    "Fred Falke": "Part IV",
    "Just Brothers": "Sliced Tomatoes",
    "SL2": "On A Ragga Tip",
    "Spiller": "Mighty Society",
    "Streamer": "Streamer",
    "Optimo (Espacio)": "How To Kill The DJ (Part Two)",
    "Danmass": "Formfreaks",
    "Jaydee": "Selected",
    "Kagami": "Starlight",
    "Natural Born Chillers": "Shotgun",
    "Raven Maize": "Maize Daze",
    # Add other artists and their acclaimed albums here if needed
    # Example: "Aphex Twin": "Selected Ambient Works Volume II" # If we wanted to ensure this specific one.
}


def load_and_parse_csv(filepath, artist_col_name, album_col_name):
    """Loads a CSV file and extracts artist and album data."""
    data = []
    try:
        with open(filepath, "r", encoding="utf-8") as f:
            reader = csv.DictReader(f)
            if (
                artist_col_name not in reader.fieldnames
                or album_col_name not in reader.fieldnames
            ):
                print(
                    f"Warning: Expected columns '{artist_col_name}' and/or '{album_col_name}' not found in {filepath}. Skipping this file."
                )
                return []
            for row in reader:
                artist = row.get(artist_col_name, "").strip()
                album = row.get(album_col_name, "").strip()
                if artist and album:  # Only add if both are non-empty after stripping
                    data.append({"artist_name": artist, "album_title": album})
    except FileNotFoundError:
        print(f"Warning: File not found {filepath}. Proceeding without it.")
    except Exception as e:
        print(f"Error reading {filepath}: {e}. Proceeding without it.")
    return data


def is_likely_album(album_title):
    """
    Checks if the album title is likely a full album and not a single, EP, N/A, etc.
    This function aims to be reasonably strict to improve data quality.
    """
    album_lower = album_title.lower()

    # Condition 1: Explicitly 'n/a', 'na', or starts with 'n/a '
    if album_lower == "n/a" or album_lower == "na" or album_lower.startswith("n/a "):
        return False

    # Condition 2: Specific undesirable exact titles (case-insensitive)
    undesirable_exact_titles = ["white label", "single", "ep"]  # 'na' covered above
    if album_lower in undesirable_exact_titles:
        return False

    # Condition 3: Album title ends with typical single/EP signifiers, possibly in parentheses or brackets
    # Covers: (single), [ep], (maxi-single), (remixes), etc.
    if re.search(
        r"\s*[\(\[]\s*(single|maxi-single|ep|maxi single|remixes|versions|edit|radio edit)\s*[\)\]]$",
        album_lower,
    ):
        return False

    # Condition 4: Album title contains phrases (often in parentheses) that indicate non-album content
    # Using (?i) for case-insensitivity within the pattern.
    non_album_phrases_pattern = r"(?i)\((dj tool|sample[ /]dj tool|mashup alias|spoken word|library music|dj intro/skit|obscure|project/single based|hip hop eps/singles|singles/influence|dub artist|single-focused|dj/single-focused|ep/single focused|dj alias|compilation only|promo|interview|soundtrack|ost)\)"
    if re.search(
        non_album_phrases_pattern, album_title
    ):  # Check original case title for this pattern
        return False

    # Condition 5: Album title ends with " ep" or " single" (without parentheses)
    # This helps catch cases like "My Song Title EP"
    if album_lower.endswith(" ep") or album_lower.endswith(" single"):
        return False

    # Condition 6: Album title is "Soundtrack" or "OST" (often generic)
    # This might be too broad if we want specific soundtracks, but for a general list, it's often noise.
    # The regex above handles (Soundtrack) and (OST). This handles standalone.
    if album_lower == "soundtrack" or album_lower == "ost":
        return False

    return True


def clean_album_data():
    """
    Merges album lists from two CSV files, incorporates manual suggestions,
    cleans the data, and saves the sorted, unique list to a new CSV file.
    """
    os.makedirs(OUTPUT_DIR, exist_ok=True)

    all_albums_raw = []

    canon_data = load_and_parse_csv(CANON_FILE, CANON_ARTIST_COL, CANON_ALBUM_COL)
    all_albums_raw.extend(canon_data)

    heck_data = load_and_parse_csv(HECK_FILE, HECK_ARTIST_COL, HECK_ALBUM_COL)
    all_albums_raw.extend(heck_data)

    # Use a dictionary to store unique albums, preserving the first encountered casing.
    # Key: (lowercase_artist_name, lowercase_album_title)
    # Value: {'artist_name': OriginalArtist, 'album_title': OriginalAlbum}
    unique_albums = {}

    for item in all_albums_raw:
        artist_name = item["artist_name"]
        album_title = item["album_title"]

        if not artist_name or not album_title:  # Should be caught by load_and_parse_csv
            continue

        if not is_likely_album(album_title):
            # print(f"Filtered from file: Artist: '{artist_name}', Album: '{album_title}'") # For debugging
            continue

        artist_key = artist_name.lower()
        album_key = album_title.lower()

        if (artist_key, album_key) not in unique_albums:
            unique_albums[(artist_key, album_key)] = {
                OUTPUT_ARTIST_COL: artist_name,
                OUTPUT_ALBUM_COL: album_title,
            }

    # Incorporate manual suggestions
    # Get set of lowercase artist names already processed from files AND having a valid album
    artists_with_valid_album_from_files_lowercase = {
        key[0] for key in unique_albums.keys()
    }

    for suggested_artist, suggested_album in MANUAL_ALBUM_SUGGESTIONS.items():
        suggested_artist_key = suggested_artist.lower()

        # Add suggestion only if the artist is NOT already in unique_albums
        # (meaning they either weren't in the files or their file entry was filtered out)
        if suggested_artist_key not in artists_with_valid_album_from_files_lowercase:
            if is_likely_album(suggested_album):  # Validate the suggested album too
                album_key = suggested_album.lower()
                # Ensure this specific artist-album pair isn't somehow already there
                if (suggested_artist_key, album_key) not in unique_albums:
                    unique_albums[(suggested_artist_key, album_key)] = {
                        OUTPUT_ARTIST_COL: suggested_artist,  # Use original casing from suggestion
                        OUTPUT_ALBUM_COL: suggested_album,
                    }
                    # print(f"Added manual suggestion: Artist: '{suggested_artist}', Album: '{suggested_album}'") # For debugging
            # else: # For debugging
            # print(f"Manual suggestion filtered: Artist: '{suggested_artist}', Album: '{suggested_album}'")

    if not unique_albums:
        print(
            "No valid album data loaded from input files or manual suggestions. Output CSV will contain headers only."
        )
        with open(OUTPUT_FILE, "w", newline="", encoding="utf-8") as f_out:
            writer = csv.DictWriter(
                f_out,
                fieldnames=[OUTPUT_ARTIST_COL, OUTPUT_ALBUM_COL],
                quoting=csv.QUOTE_ALL,
            )
            writer.writeheader()
        print(f"Empty merged list saved to {OUTPUT_FILE}")
        return

    cleaned_albums_list = list(unique_albums.values())

    # Sort data (case-insensitive for sorting keys, preserving original case in output)
    cleaned_albums_list.sort(
        key=lambda x: (x[OUTPUT_ARTIST_COL].lower(), x[OUTPUT_ALBUM_COL].lower())
    )

    with open(OUTPUT_FILE, "w", newline="", encoding="utf-8") as f_out:
        writer = csv.DictWriter(
            f_out,
            fieldnames=[OUTPUT_ARTIST_COL, OUTPUT_ALBUM_COL],
            quoting=csv.QUOTE_ALL,
        )
        writer.writeheader()
        writer.writerows(cleaned_albums_list)

    print(f"Merged and cleaned album list saved to {OUTPUT_FILE}")
    print(f"Total unique albums processed: {len(cleaned_albums_list)}")


if __name__ == "__main__":
    clean_album_data()
