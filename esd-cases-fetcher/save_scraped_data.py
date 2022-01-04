import os
import json
from typing import List

from db import db
import re

f_name = os.path.join("scraper", "data.json")
with open(f_name, mode="r", encoding="utf-8") as f:
    scraped_data: List = json.load(f)

    # remove duplicates
    data = []
    codes_met = set()
    for row in scraped_data:
        if row["code"] not in codes_met:
            new_row = row
            match = re.search(r"C-\d{1,4}/\d{1,2}", row["code"])

            if match == None:
                continue
            new_row["code"] = match.group()

            data.append(new_row)
            codes_met.add(row["code"])

    db.EsdCases_Code.delete().execute()

    with db.db.atomic():
        db.EsdCases_Code.insert_many(data).execute()
