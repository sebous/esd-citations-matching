import os
import sqlite3
from pathlib import Path


def init(db_file):
    conn = sqlite3.connect(db_file)
    c = conn.cursor()

    # create tables etc
    query = __load_query("./queries/init_db.sql")
    c.execute(query)

    return c


def __load_query(path):
    curr_path = Path(__file__).parent
    with open(os.path.join(curr_path, path), 'r') as file:
        return file.read()
