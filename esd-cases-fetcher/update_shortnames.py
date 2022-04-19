import os
import json
from typing import List

from db import db
import re

# this gets short_name from scraped data ion scraper/data.json


def run():
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

        cases_select = (
            db.EsdCases
            .select(db.EsdCases.id, db.EsdCaseInfos.case_id, db.EsdCaseInfos.code)
            .join(db.EsdCaseInfos)
            .dicts()
        )

        for case_row in cases_select:
            item = next(
                (x for x in data if x["code"] == case_row["code"]), None)
            if item == None:
                case_row["short_name"] = None
                continue
            case_row["short_name"] = item["short_name"]
            case_row = db.EsdCases(case_row)

        # data to update needs to be db models for bulk_update to work
        update_data = [db.EsdCases(
            id=x["id"], short_name=x["short_name"]) for x in cases_select]

        with db.db.atomic():
            db.EsdCases.bulk_update(update_data, fields=[
                db.EsdCases.short_name], batch_size=100)
