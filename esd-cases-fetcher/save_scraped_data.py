import os
import json
from typing import List

from db import db
import re

db.init()

f_name = os.path.join("scraper", "data.json")
with open(f_name, mode="r", encoding="utf-8") as f:
    scraped_data: List = json.load(f)

    # remove duplicates
    data = []
    codes_met = set()
    for row in scraped_data:
        match = re.search(r"C-\d{1,4}/\d{1,2}", row["code"])
        if match == None:
            continue
        code = match.group()

        if code not in codes_met:
            new_row = row
            new_row["code"] = code

            data.append(new_row)
            codes_met.add(code)

    db.EsdCases_Code.delete().execute()

    with db.db.atomic():
        db.EsdCases_Code.insert_many(data).execute()
