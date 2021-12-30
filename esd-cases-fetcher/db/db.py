import os
import sqlite3
from pathlib import Path
import peewee

db = peewee.SqliteDatabase("../db/db.sqlite")


class EsdCases(peewee.Model):
    full_name = peewee.CharField(null=True)
    short_name = peewee.CharField(null=True)
    code = peewee.CharField(null=True)

    class Meta:
        database = db


models = [EsdCases]


def init():
    db.connect()
    db.create_tables(models)
