import csv
from operator import itemgetter
import re
import os
from typing import Dict
from db import db
import util
from datetime import datetime
from itertools import groupby


def map_row_code(row: Dict):
    code_match = re.search(r"\d{1,4}[-/]\d{1,2}", row["case_id"])
    row["case_id"] = util.normalize_esd_code(code_match.group())
    return row

# fn gets EsdCases and EsdCaseInfos from source csv
# short_name and full_name are missing in this datasource


def run():
    with open(os.path.join("csv_source", "esd_cases_source.csv"), mode="r", encoding="utf-8") as f:
        reader = csv.DictReader(f, delimiter=",")
        reader = list(reader)

        esd_cases = []
        ecli_met = set()

        # fill EsdCases table
        for row in reader:
            row_date = ''
            if row["date"] == "NA":
                row_date = None
            else:
                row_date = datetime.strptime(row["date"], '%Y-%m-%d').date()

            if row["ecli"] == "NA":
                continue
            if row["ecli"] not in ecli_met:
                ecli_met.add(row["ecli"])
                esd_cases.append(
                    {"ecli": row["ecli"], "date": row_date, "jr": row["jr"] if row["jr"] != "NA" else None, "cf": row["cf"] if row["cf"] != "NA" else None})

        with db.db.atomic():
            db.EsdCases.insert_many(esd_cases).execute()

        esd_cases_db = db.EsdCases.select().dicts()
        esd_case_infos = []

        rows_mapped = [map_row_code(item) for item in reader]
        rows_mapped = sorted(rows_mapped, key=itemgetter("case_id"))
        rows_mapped = groupby(rows_mapped, key=itemgetter("case_id"))

        rows_filtered = []

        for _, grouped_rows in rows_mapped:
            grouped_rows = list(grouped_rows)

            if len(grouped_rows) < 2:
                rows_filtered.append(grouped_rows[0])
                continue

            added = False
            for r in grouped_rows:
                if r["case_info"].startswith("Judgment"):
                    rows_filtered.append(r)
                    added = True
                    break

            if added == False:
                sorted_r = sorted(grouped_rows, key=lambda x: x["date"])
                rows_filtered.append(sorted_r[0])

        # fill EsdCaseInfos table
        for row in rows_filtered:
            if row["ecli"] == "NA":
                continue
            base_case = next(
                (item for item in esd_cases_db if item["ecli"] == row["ecli"]), None)
            if base_case == None:
                continue
            case_id = base_case["id"]

            case_info = {"case_id": case_id, "code": row["case_id"],
                         "info_text": row["case_info"]}
            esd_case_infos.append(case_info)

        with db.db.atomic():
            db.EsdCaseInfos.insert_many(esd_case_infos).execute()
