import os
import csv

from db import db


def run():
    with open(os.path.join("csv_source", "source_cases.csv"), mode="r", encoding="utf-8") as f:
        reader = csv.DictReader(f, delimiter=",")

        source_cases = []

        for row in reader:
            source_cases.append({
                "code": row["code"], "date": row["date"], "file_name": row["filename"], "court": row["court"]})

        with db.db.atomic():
            db.SourceCases.insert_many(source_cases).execute()
