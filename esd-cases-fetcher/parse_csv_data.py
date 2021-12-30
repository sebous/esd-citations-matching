import csv
import re
from db import db


with open("04-14-test.csv", mode='r', encoding="utf-8-sig") as f:
    reader = csv.DictReader(f, delimiter=",")

    data = []
    for i, line in enumerate(reader):
        name = line["nazev"]
        match = re.search(r"C[-‑]\d{1,4}/\d{1,2}", name)

        if match is None:
            print(f"error --> no code found on line: {i}")

        code = match.group()

        data.append({"code": code, "full_name": name})

    with db.db.atomic():
        db.EsdCases.insert_many(data).execute()
