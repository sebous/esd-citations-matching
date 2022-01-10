from enum import unique
import peewee

db = peewee.SqliteDatabase("../db/db.sqlite")


class BaseModel(peewee.Model):
    class Meta:
        database = db


class EsdCases(BaseModel):
    id = peewee.AutoField()
    code = peewee.CharField(unique=True)
    short_name = peewee.CharField()
    full_name = peewee.CharField(null=True)
    date = peewee.DateField(null=True)


class EsdRelatedCases(BaseModel):
    id = peewee.AutoField()
    parent_case = peewee.ForeignKeyField(EsdCases, backref="related_cases")
    code = peewee.CharField(unique=True)


class Matches(BaseModel):
    id = peewee.AutoField()
    source_case = peewee.CharField()
    matched_case = peewee.ForeignKeyField(EsdCases, backref="matches")
    matched_value = peewee.CharField()
    type = peewee.CharField()


models = [EsdCases, EsdRelatedCases, Matches]


def init():
    db.connect()
    db.create_tables(models)
