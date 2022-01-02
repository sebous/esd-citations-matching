import os
import sqlite3
from pathlib import Path
import peewee

db = peewee.SqliteDatabase("../db/db.sqlite")


class BaseModel(peewee.Model):
    class Meta:
        database = db


class EsdCases_Fulltext(BaseModel):
    text = peewee.CharField()
    date = peewee.DateField(null=True)


class EsdCases_Code(BaseModel):
    short_name = peewee.CharField(null=True)
    code = peewee.CharField(null=True)
    date = peewee.DateField(null=True)


class Matches(BaseModel):
    source_case = peewee.CharField()
    matched_case_fulltext = peewee.ForeignKeyField(
        EsdCases_Fulltext, backref="matches", null=True)
    matched_case_code = peewee.ForeignKeyField(
        EsdCases_Code, backref="matches", null=True)


models = [EsdCases_Fulltext, EsdCases_Code, Matches]


def init():
    db.connect()
    db.create_tables(models)
