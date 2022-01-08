import csv
import os
import re
from datetime import datetime

from db import db

db.init()

dir_name = "csv_source"


def extract_codes(input: str):
    output: list[str] = []
    input_list = list(re.finditer(
        r"(?<![a-zA-Z])\d{1,4}(?:-\d{1,4})?(?:/{1,2}\d{2})?", input))
    # print(input, input_list)

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
    # if len(output) > 1:
    #     print(input, output)
    return output


# extract_codes("219-228, 230-235, 237, 238 a 240-242/80.' PAD2")

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
            code_part = name.split("#")[-1]

            extract_codes(code_part)

# code_matches = re.findall(
#     r"\d{1,4}[\/\-\-]\d{1,2}", code_part)

# if len(code_matches) > 1:
#     print(code_part)

#     date_str = line["Datum dokumentu"]

#     date = datetime.strptime(date_str, "%Y-%m-%d")

#     if name == "":
#         print(
#             f"error --> empty Name field, line: {i + 2}, filename: {f_name}, skipping...")
#         continue

#     data.append({"text": name, "date": date})

# with db.db.atomic():
#     db.EsdCases_Fulltext.insert_many(data).execute()
