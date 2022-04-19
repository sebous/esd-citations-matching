from db import db
import esd_scraped_data_saver
import update_shortnames
import source_cases_data_saver

db.clear()
db.init()

esd_scraped_data_saver.run()
update_shortnames.run()
source_cases_data_saver.run()
