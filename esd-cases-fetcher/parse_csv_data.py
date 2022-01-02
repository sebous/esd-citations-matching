import csv
import re
from db import db
import os

db.init()

dir_name = "csv_source"
for filename in os.listdir(dir_name):
    f_name = os.path.join(dir_name, filename)
    if not os.path.isfile(f_name):
        continue
    
    with open(f_name, mode="r", encoding="utf-8-sig") as f:
        reader = csv.DictReader(f, delimiter=",")

        data = []
        for i, line in enumerate(reader):
            name = line["Název"]
            matches = re.findall(r"\d{1,4}[/\--]\d{1,2}[ \u202F\u00A0,.)]", name)

            # print(matches)

            if len(matches) == 0:
                print(f"error --> no code found on line: {i}, filename: {f_name}")
                continue

            for match in matches:
                data.append({"code": f"C-{match[:-1]}", "full_name": name})

        with db.db.atomic():
            db.EsdCases.insert_many(data).execute()
