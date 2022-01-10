import csv
import os
import re

from peewee import DatabaseError

from db import db

db.init()

dir_name = "csv_source"


def extract_codes(input: str):
    output: list[str] = []
    input_list = list(re.finditer(
        r"(?<![a-zA-Z])\d{1,4}(?:-\d{1,4})?(?:/{1,2}\d{2})?", input))

    dash_as_fwdslash = False

    for i, match in enumerate(reversed(input_list)):
        last = output[-1].split("/") if len(output) > 0 else None

        # 1-10/10
        if "/" in match.group() and "-" in match.group():
            num, year = match.group().split("/")
            start, end = num.split("-")
            for n in range(int(start), int(end) + 1):
                output.append(f"{n}/{year}")
        # 10/10
        elif "/" in match.group():
            if match.end() == len(input) or i == 0:
                output.append(match.group())
            elif "až" in input[match.end() + 1:list(reversed(input_list))[i-1].start()]:
                for num in range(int(match.group().split("/")[0]), int(last[0])):
                    output.append(f"{num}/{last[1]}")
            else:
                output.append(match.group())
        # 10-12
        elif "-" in match.group():
            # 1) [dash as fwdslash] 10-12 -> 10/12
            # 2) [dash as range] 10-12 -> 10,11,12

            match_replaced = match.group().replace("-", "/")

            # 1
            if len(output) == 0 or dash_as_fwdslash:
                dash_as_fwdslash = True
                if match.end() == len(input) or i == 0:
                    output.append(match_replaced)
                elif "až" in input[match.end() + 1:list(reversed(input_list))[i-1].start()]:
                    for num in range(int(match_replaced.split("/")[0]), int(last[0])):
                        output.append(f"{num}/{last[1]}")
                else:
                    output.append(match_replaced)
            # 2
            else:
                start, end = match.group().split("-")
                for num in range(int(start), int(end) + 1):
                    output.append(f"{num}/{last[1]}")
        # 10
        else:
            if "až" in input[match.end() + 1:list(reversed(input_list))[i-1].start()]:
                for num in range(int(match.group()), int(last[0])):
                    output.append(f"{num}/{last[1]}")
            else:
                output.append(f"{match.group()}/{last[1]}")

    output.sort(key=lambda str: int(str.split("/")[0]))
    output = [f"C-{code}" for code in output]

    # print(output)
    if len(output) == 0:
        print(input, output)
    return output


# missing_codes = []
db.EsdRelatedCases.delete().execute()


for filename in os.listdir(dir_name):
    f_name = os.path.join(dir_name, filename)
    if not os.path.isfile(f_name):
        continue

    with open(f_name, mode="r", encoding="utf-8-sig") as f:
        reader = csv.DictReader(
            f, delimiter=',')

        related_cases_data = []
        for i, line in enumerate(reader):
            name_parts = line["Název"].split("#")
            code_part = name_parts[-1]
            if line["Název"] == "" or code_part == "":
                continue
            full_name = name_parts[1]

            codes = extract_codes(code_part)

            try:
                query: "list[db.EsdCases]" = db.EsdCases.select(db.EsdCases.code, db.EsdCases.id).where(
                    db.EsdCases.code.in_(codes))

                # skip if code missing
                if len(query) == 0:
                    continue

                # one code match
                if len(query) == 1:
                    row = query[0]

                    # update full_name in main table
                    record: db.EsdCases = db.EsdCases.get_by_id(query[0].id)
                    if record.full_name == None or record.full_name == "":
                        record.full_name = full_name
                        record.save()

                    # insert related cases into related table
                    related_codes = [c for c in codes if record.code != c]
                    if len(related_codes) == 0:
                        continue

                    for code in related_codes:
                        related_cases_data.append(
                            {"parent_case_id": record.id, "code": code})

                # multiple codes match, related cases will be joined to the first code found (they are all related anyway)
                if len(query) > 1:
                    related_codes = []
                    parent_case_id = None
                    for esd_case in query:
                        case: db.EsdCases = db.EsdCases.get_or_none(
                            db.EsdCases.code == esd_case.code)
                        if case == None:
                            related_codes.append(esd_case.code)
                        else:
                            case.full_name = esd_case.full_name
                            case.save()
                            if parent_case_id == None:
                                parent_case_id = case.id

                        for code in related_codes:
                            related_cases_data.append(
                                {"parent_case_id": parent_case_id, "code": code})

            except DatabaseError as err:
                print(f"{err}, codes: {codes}")

        try:
            with db.db.atomic():
                db.EsdRelatedCases.insert_many(
                    related_cases_data).on_conflict_ignore().execute()
                related_cases_data = []
        except DatabaseError as err:
            print(err)
