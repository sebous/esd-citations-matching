from db import db
from parse_csv_data import parse_csv_data
from save_scraped_data import save_scraped_data

db.init()

save_scraped_data()
parse_csv_data()
