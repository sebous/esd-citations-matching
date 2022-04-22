from enum import unique
import peewee

db = peewee.SqliteDatabase("../db/db.sqlite")


class BaseModel(peewee.Model):
    class Meta:
        database = db


class SourceCases(BaseModel):
    id = peewee.AutoField()
    code = peewee.CharField()
    date = peewee.DateField(null=True)
    file_name = peewee.CharField(unique=True)
    court = peewee.CharField(null=True)


class EsdCases(BaseModel):
    id = peewee.AutoField()
    ecli = peewee.CharField(unique=True)
    short_name = peewee.CharField(null=True)
    full_name = peewee.CharField(null=True)
    date = peewee.DateField(null=True)
    jr = peewee.CharField(null=True)
    cf = peewee.CharField(null=True)


class EsdCaseInfos(BaseModel):
    id = peewee.AutoField()
    case_id = peewee.ForeignKeyField(EsdCases, backref="cases")
    code = peewee.CharField(unique=True)
    info_text = peewee.CharField(null=True)


class Matches(BaseModel):
    id = peewee.AutoField()
    source_case_id = peewee.ForeignKeyField(SourceCases, backref="matches")
    matched_case_id = peewee.ForeignKeyField(EsdCases, backref="matches")
    matched_value = peewee.CharField()
    type = peewee.CharField()


models = [EsdCases, EsdCaseInfos, Matches, SourceCases]


def init():
    db.create_tables(models)


def clear():
    db.drop_tables(models)
