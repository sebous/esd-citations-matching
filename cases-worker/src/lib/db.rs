use std::iter::once;

use rusqlite::{params, types::Null, Connection, Result};

#[derive(Debug)]
pub struct EsdCase {
    pub id: usize,
    pub code: String,
    pub short_name: String,
    pub full_name: Option<String>,
    pub date: String,
    pub related_cases: Vec<EsdRelatedCase>,
}

pub trait Code {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_>;
}

impl Code for EsdCase {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(once(&self.code).chain(self.related_cases.iter().map(|c| &c.code)))
    }
}
impl Code for Vec<EsdCase> {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.iter().flat_map(|case| case.get_codes()))
    }
}

#[derive(Debug)]
pub struct EsdRelatedCase {
    pub id: usize,
    pub code: String,
}

#[derive(Debug)]
pub struct Match {
    pub source_case: String,
    pub matched_case_id: usize,
    pub matched_value: String,
    pub m_type: String,
}

struct EsdJoinedCaseDbRow {
    pub id: usize,
    pub code: String,
    pub short_name: String,
    pub full_name: Option<String>,
    pub date: String,
    pub rel_id: Option<usize>,
    pub rel_code: Option<String>,
}

pub fn fetch_data(db_conn: &Connection) -> Result<Vec<EsdCase>> {
    let mut query = db_conn
        .prepare(
            "
        SELECT C.id, C.code, C.short_name, C.full_name, C.date, R.id, R.code
        FROM esdcases C
        LEFT OUTER JOIN esdrelatedcases R ON C.id=R.parent_case_id
        ORDER BY C.id
    ",
        )
        .unwrap();

    let cases = query
        .query_map([], |row| {
            Ok(EsdJoinedCaseDbRow {
                id: row.get(0).unwrap(),
                code: row.get(1).unwrap(),
                short_name: row.get(2).unwrap(),
                full_name: row.get(3).unwrap_or(None),
                date: row.get(4).unwrap(),
                rel_id: row.get(5).unwrap_or(None),
                rel_code: row.get(6).unwrap_or(None),
            })
        })?
        .map(|r| r.unwrap())
        .fold(vec![], |mut result: Vec<EsdCase>, row| {
            let last_item = result.iter().last();
            if last_item.is_some() && last_item.unwrap().id == row.id {
                result
                    .last_mut()
                    .unwrap()
                    .related_cases
                    .push(EsdRelatedCase {
                        id: row.rel_id.unwrap(),
                        code: row.rel_code.unwrap(),
                    });
                return result;
            }

            result.push(EsdCase {
                id: row.id,
                code: row.code,
                short_name: row.short_name,
                full_name: row.full_name,
                date: row.date,
                related_cases: vec![],
            });
            result
        });
    Ok(cases)
}

pub fn save_match(match_obj: Match, db_conn: &Connection) -> Result<()> {
    db_conn.execute(
        "
        INSERT INTO matches (source_case, matched_case_id, matched_value, type)
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![
            match_obj.source_case,
            match_obj.matched_case_id,
            match_obj.matched_value,
            match_obj.m_type,
        ],
    )?;

    Ok(())
}

pub fn clear_matches(db_conn: &Connection) -> Result<()> {
    db_conn.execute("DELETE FROM matches", [])?;
    Ok(())
}
