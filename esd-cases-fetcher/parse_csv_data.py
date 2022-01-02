import csv
import re
import os
from datetime import datetime

from db import db

db.init()

dir_name = "csv_source"

for filename in os.listdir(dir_name):
    f_name = os.path.join(dir_name, filename)
    if not os.path.isfile(f_name):
        continue

    with open(f_name, mode="r", encoding="utf-8-sig") as f:
        reader = csv.DictReader(
            f, delimiter=',')

        data = []
        for i, line in enumerate(reader):
            name = line["Název"]
            date_str = line["Datum dokumentu"]

            date = datetime.strptime(date_str, "%Y-%m-%d")

            if name == "":
                print(
                    f"error --> empty Name field, line: {i + 2}, filename: {f_name}, skipping...")
                continue

            data.append({"text": name, "date": date})

        with db.db.atomic():
            db.EsdCases_Fulltext.insert_many(data).execute()
