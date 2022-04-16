import collections
import csv
import re
from db import db
import util
from datetime import datetime


def save_esd_scraped_data():
    with open("esd_cases_source.csv", mode="r", encoding="utf-8") as f:
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

        # with db.db.atomic():
        #     db.EsdCases.insert_many(esd_cases).execute()

        esd_cases_db = db.EsdCases.select().dicts()
        esd_case_infos = []

        codes = []
        for row in reader:
            code_match = re.search(r"\d{1,4}[-/]\d{1,2}", row["case_id"])
            if code_match == None:
                print(row["case_id"], row)
            code = util.normalize_esd_code(code_match.group())
            codes.append(code)

        print([item for item, count in collections.Counter(
            codes).items() if count > 1].__len__())

        # fill EsdCaseInfos table
        # for row in reader:
        #     if row["ecli"] == "NA":
        #         continue
        #     base_case = next(
        #         (item for item in esd_cases_db if item["ecli"] == row["ecli"]), None)
        #     if base_case == None:
        #         continue
        #     case_id = base_case["id"]

        #     code_match = re.search(r"\d{1,4}[-/]\d{1,2}", row["case_id"])
        #     if code_match == None:
        #         print(row["case_id"], row)
        #     code = util.normalize_esd_code(code_match.group())

        #     esd_case_infos.append(
        #         {"case_id": case_id, "code": code,
        #             "info_text": row["case_info"]}
        #     )
        # with db.db.atomic():
        #     db.EsdCaseInfos.insert_many(esd_case_infos).execute()
