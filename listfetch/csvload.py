from pydantic import BaseModel
from typing import List
import csv


class Track(BaseModel):
    show_url: str
    is_mashup: bool
    mashup_name: str
    artist: str
    track_title: str


def load_tracks_from_csv(file_path: str) -> List[Track]:
    with open(file_path, newline="", encoding="utf-8") as csvfile:
        reader = csv.DictReader(csvfile)
        return [
            Track(
                **{
                    **row,
                    "is_mashup": str(row.get("is_mashup", "false")).lower() == "true",
                }
            )
            for row in reader
        ]


# Usage
tracks = load_tracks_from_csv("2manybootlegs_tracks.csv")
