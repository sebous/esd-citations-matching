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
    matched_case_id = peewee.IntegerField()
    matched_case_table = peewee.CharField()
    type = peewee.CharField()


models = [EsdCases_Fulltext, EsdCases_Code, Matches]


def init():
    db.connect()
    db.create_tables(models)
