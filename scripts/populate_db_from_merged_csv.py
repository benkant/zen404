import sqlite3
import csv
import os

# Define file paths relative to the project root (404zen/)
# Assumes the script is run from the 404zen directory, e.g., `python scripts/populate_db_from_merged_csv.py`
DB_FILE = "data/audio_narrative_database.sqlite"
MERGED_CSV_FILE = "data/processed/merged_album_list.csv"

# CSV column names
CSV_ARTIST_COL = "artist_name"
CSV_ALBUM_COL = "album_title"


def populate_artists_and_albums():
    """
    Populates the Artists and Albums tables in the SQLite database
    from the merged_album_list.csv file.
    It ensures that artists and albums are unique.
    """
    if not os.path.exists(MERGED_CSV_FILE):
        print(f"Error: Merged CSV file not found at {MERGED_CSV_FILE}")
        print("Please run the merge_album_lists.py script first.")
        return

    if not os.path.exists(DB_FILE):
        print(f"Error: Database file not found at {DB_FILE}")
        print("Please ensure the database schema has been created.")
        return

    conn = None
    try:
        conn = sqlite3.connect(DB_FILE)
        cursor = conn.cursor()

        print(f"Populating database from {MERGED_CSV_FILE}...")

        with open(MERGED_CSV_FILE, "r", encoding="utf-8") as f_in:
            reader = csv.DictReader(f_in)

            if (
                CSV_ARTIST_COL not in reader.fieldnames
                or CSV_ALBUM_COL not in reader.fieldnames
            ):
                print(
                    f"Error: CSV file {MERGED_CSV_FILE} is missing required columns '{CSV_ARTIST_COL}' or '{CSV_ALBUM_COL}'."
                )
                return

            artists_added = 0
            albums_added = 0
            artists_skipped = 0
            albums_skipped = 0

            for row_num, row in enumerate(reader, 1):
                artist_name = row.get(CSV_ARTIST_COL)
                album_title = row.get(CSV_ALBUM_COL)

                if not artist_name or not album_title:
                    print(f"Skipping row {row_num}: missing artist or album name.")
                    continue

                try:
                    # Add Artist if not exists
                    cursor.execute(
                        "INSERT OR IGNORE INTO Artists (name) VALUES (?)",
                        (artist_name,),
                    )
                    if cursor.rowcount > 0:
                        artists_added += 1
                    else:
                        artists_skipped += 1

                    # Get artist_id (either newly inserted or existing)
                    cursor.execute(
                        "SELECT artist_id FROM Artists WHERE name = ?", (artist_name,)
                    )
                    artist_result = cursor.fetchone()
                    if artist_result:
                        artist_id = artist_result[0]

                        # Add Album if not exists for this artist
                        # Using a composite key (title, artist_id) for album uniqueness check implicitly
                        # by trying to insert and ignoring if it violates a unique constraint (if one were defined on title+artist_id)
                        # or by checking first. Here, we check first to be explicit.

                        cursor.execute(
                            "SELECT album_id FROM Albums WHERE title = ? AND artist_id = ?",
                            (album_title, artist_id),
                        )
                        album_exists = cursor.fetchone()

                        if not album_exists:
                            cursor.execute(
                                "INSERT INTO Albums (title, artist_id) VALUES (?, ?)",
                                (album_title, artist_id),
                            )
                            if cursor.rowcount > 0:
                                albums_added += 1
                        else:
                            albums_skipped += 1
                    else:
                        print(
                            f"Error: Could not find or create artist_id for {artist_name}. Skipping album '{album_title}'."
                        )
                        albums_skipped += 1

                except sqlite3.IntegrityError as e:
                    # This might catch unique constraint violations if not handled by OR IGNORE or prior checks
                    print(
                        f"Skipping due to integrity error (likely duplicate): Artist '{artist_name}', Album '{album_title}'. Error: {e}"
                    )
                    if "Artists.name" in str(e):
                        artists_skipped += 1
                    elif "Albums" in str(e):  # A more generic catch for album issues
                        albums_skipped += 1
                except Exception as e:
                    print(
                        f"An unexpected error occurred for Artist '{artist_name}', Album '{album_title}': {e}"
                    )
                    albums_skipped += 1

            conn.commit()
            print("\nPopulation complete.")
            print(f"Artists added: {artists_added}")
            print(f"Artists skipped (already existed): {artists_skipped}")
            print(f"Albums added: {albums_added}")
            print(f"Albums skipped (already existed or error): {albums_skipped}")

    except sqlite3.Error as e:
        print(f"SQLite error: {e}")
    except FileNotFoundError:
        print(f"Error: File not found. Ensure {MERGED_CSV_FILE} exists.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
    finally:
        if conn:
            conn.close()


if __name__ == "__main__":
    populate_artists_and_albums()
