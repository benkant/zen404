import sqlite3
import csv
import json
import os
import re

# Define file paths relative to the project root (404zen/)
# Assumes the script is run from the 404zen directory, e.g., `python scripts/populate_narrative_tracks.py`
DB_FILE = "data/audio_narrative_database.sqlite"
MALE_NARRATIVE_CSV = "data/male_narrative_selection.csv"
FEMALE_NARRATIVE_JSON = "data/female_narrative_selection.json"

# CSV column names for male narrative
CSV_MALE_TRACK_ID_COL = "track_id"
CSV_MALE_ARTIST_COL = "artist"
CSV_MALE_TRACK_NAME_COL = "track_name"
CSV_MALE_SOUND_COL = "sound"


def get_or_create_artist(cursor, artist_name):
    """Adds artist if not exists, returns artist_id."""
    cursor.execute("INSERT OR IGNORE INTO Artists (name) VALUES (?)", (artist_name,))
    if cursor.rowcount > 0:
        # print(f"Added new artist: {artist_name}")
        pass
    cursor.execute("SELECT artist_id FROM Artists WHERE name = ?", (artist_name,))
    result = cursor.fetchone()
    if result:
        return result[0]
    else:
        # This should not happen if INSERT OR IGNORE worked or artist already existed
        raise Exception(f"Could not find or create artist_id for {artist_name}")


def track_exists(cursor, title, artist_id):
    """Checks if a track with the given title and artist_id already exists."""
    cursor.execute(
        "SELECT track_id FROM Tracks WHERE title = ? AND artist_id = ?",
        (title, artist_id),
    )
    return cursor.fetchone() is not None


def get_max_female_project_track_id_numeric(cursor):
    """Gets the maximum numeric part of existing Fxxx project_track_ids."""
    cursor.execute(
        "SELECT project_track_id FROM Tracks WHERE project_track_id LIKE 'F%'"
    )
    max_num = 0
    for row in cursor.fetchall():
        if row[0] and re.match(r"F\d+", row[0]):
            try:
                num = int(row[0][1:])
                if num > max_num:
                    max_num = num
            except ValueError:
                continue  # Should not happen with LIKE 'F%' and regex match
    return max_num


def populate_tracks_from_narrative_selections():
    """
    Populates the Artists and Tracks tables from the male and female
    narrative selection files, handling duplicates.
    """
    if not os.path.exists(DB_FILE):
        print(f"Error: Database file not found at {DB_FILE}")
        return

    conn = None
    try:
        conn = sqlite3.connect(DB_FILE)
        cursor = conn.cursor()

        artists_added_count = 0
        tracks_added_count = 0
        tracks_skipped_count = 0

        # --- Process Male Narrative Selections (CSV) ---
        print(f"\nProcessing Male Narrative Selections from {MALE_NARRATIVE_CSV}...")
        if not os.path.exists(MALE_NARRATIVE_CSV):
            print(f"Warning: Male narrative CSV file not found at {MALE_NARRATIVE_CSV}")
        else:
            with open(MALE_NARRATIVE_CSV, "r", encoding="utf-8") as f_in:
                reader = csv.DictReader(f_in)
                for row in reader:
                    artist_name = row.get(CSV_MALE_ARTIST_COL, "").strip()
                    track_title = row.get(CSV_MALE_TRACK_NAME_COL, "").strip()
                    project_track_id = row.get(CSV_MALE_TRACK_ID_COL, "").strip()
                    sound_description = row.get(CSV_MALE_SOUND_COL, "").strip()

                    if not artist_name or not track_title:
                        print(
                            f"Skipping row due to missing artist or track title: {row}"
                        )
                        continue

                    try:
                        initial_artist_count = cursor.execute(
                            "SELECT COUNT(*) FROM Artists"
                        ).fetchone()[0]
                        artist_id = get_or_create_artist(cursor, artist_name)
                        if (
                            cursor.execute("SELECT COUNT(*) FROM Artists").fetchone()[0]
                            > initial_artist_count
                        ):
                            artists_added_count += 1

                        if not track_exists(cursor, track_title, artist_id):
                            cursor.execute(
                                """
                                INSERT INTO Tracks (title, artist_id, project_track_id, narrative_sound_description)
                                VALUES (?, ?, ?, ?)
                            """,
                                (
                                    track_title,
                                    artist_id,
                                    project_track_id,
                                    sound_description,
                                ),
                            )
                            tracks_added_count += 1
                        else:
                            # print(f"Skipping existing male track: '{track_title}' by {artist_name}")
                            tracks_skipped_count += 1
                    except Exception as e:
                        print(
                            f"Error processing male track '{track_title}' by {artist_name}: {e}"
                        )

        # --- Process Female Narrative Selections (JSON) ---
        print(
            f"\nProcessing Female Narrative Selections from {FEMALE_NARRATIVE_JSON}..."
        )
        female_track_id_counter = get_max_female_project_track_id_numeric(cursor) + 1

        if not os.path.exists(FEMALE_NARRATIVE_JSON):
            print(
                f"Warning: Female narrative JSON file not found at {FEMALE_NARRATIVE_JSON}"
            )
        else:
            with open(FEMALE_NARRATIVE_JSON, "r", encoding="utf-8") as f_in:
                female_tracks_data = json.load(f_in)
                for item_string in female_tracks_data:
                    parts = item_string.split(",", 1)  # Split only on the first comma
                    if len(parts) == 2:
                        track_title = parts[0].strip()
                        artist_name = parts[1].strip()
                    else:
                        print(f"Could not parse female track entry: {item_string}")
                        continue

                    if not artist_name or not track_title:
                        print(
                            f"Skipping entry due to missing artist or track title after parsing: {item_string}"
                        )
                        continue

                    project_track_id = f"F{female_track_id_counter:03d}"

                    try:
                        initial_artist_count = cursor.execute(
                            "SELECT COUNT(*) FROM Artists"
                        ).fetchone()[0]
                        artist_id = get_or_create_artist(cursor, artist_name)
                        if (
                            cursor.execute("SELECT COUNT(*) FROM Artists").fetchone()[0]
                            > initial_artist_count
                        ):
                            artists_added_count += 1

                        if not track_exists(cursor, track_title, artist_id):
                            cursor.execute(
                                """
                                INSERT INTO Tracks (title, artist_id, project_track_id)
                                VALUES (?, ?, ?)
                            """,
                                (track_title, artist_id, project_track_id),
                            )
                            tracks_added_count += 1
                            female_track_id_counter += 1
                        else:
                            # print(f"Skipping existing female track: '{track_title}' by {artist_name}")
                            tracks_skipped_count += 1
                    except Exception as e:
                        print(
                            f"Error processing female track '{track_title}' by {artist_name}: {e}"
                        )

        conn.commit()
        print("\n--- Population Summary ---")
        print(f"New artists added: {artists_added_count}")
        print(f"New tracks added: {tracks_added_count}")
        print(f"Tracks skipped (already existed or error): {tracks_skipped_count}")

    except sqlite3.Error as e:
        print(f"SQLite error: {e}")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
    finally:
        if conn:
            conn.close()


if __name__ == "__main__":
    populate_tracks_from_narrative_selections()
