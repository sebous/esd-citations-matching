import peewee

db = peewee.SqliteDatabase("../db/db.sqlite")


class BaseModel(peewee.Model):
    class Meta:
        database = db


class EsdCases(BaseModel):
    code = peewee.CharField(unique=True)
    short_name = peewee.CharField()
    full_name = peewee.CharField(null=True)
    date = peewee.DateField(null=True)


class EsdRelatedCases(BaseModel):
    code = peewee.CharField(unique=True)
    parent_case = peewee.ForeignKeyField(EsdCases, backref="related_cases")


class Matches(BaseModel):
    source_case = peewee.CharField()
    matched_case = peewee.ForeignKeyField(EsdCases, backref="matches")
    matched_value = peewee.CharField()
    type = peewee.CharField()


models = [EsdCases, EsdRelatedCases, Matches]


def init():
    db.connect()
    db.create_tables(models)
