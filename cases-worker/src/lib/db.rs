use itertools::Itertools;
use rusqlite::{params, types::Null, Connection, Result};

#[derive(Debug)]
pub struct EsdCaseCode {
    pub id: usize,
    pub short_name: String,
    pub code: String,
    pub date: String,
}
impl EsdCaseCode {
    pub const TABLE_NAME: &'static str = "esdcases_code";
}

#[derive(Debug)]
pub struct EsdCaseFulltext {
    pub id: usize,
    pub text: String,
    pub date: String,
}
impl EsdCaseFulltext {
    pub const TABLE_NAME: &'static str = "esdcases_fulltext";
}

#[derive(Debug)]
pub struct Match {
    pub source_case: String,
    pub matched_case_id: Option<usize>,
    pub matched_case_table: Option<String>,
    pub matched_value: Option<String>,
    pub m_type: String,
}

pub type EsdCasesData = (Vec<EsdCaseCode>, Vec<EsdCaseFulltext>);

pub fn fetch_data(db_conn: &Connection) -> Result<EsdCasesData> {
    let mut fulltext_query = db_conn.prepare(
        "
        SELECT id, text, date
        FROM esdcases_fulltext
        ",
    )?;

    let mut codes_query = db_conn.prepare(
        "
        SELECT id, short_name, code, date
        FROM esdcases_code
        ",
    )?;

    let fulltext_cases = fulltext_query.query_map([], |row| {
        Ok(EsdCaseFulltext {
            id: row.get(0)?,
            text: row.get(1)?,
            date: row.get(2)?,
        })
    })?;
    let codes_cases = codes_query.query_map([], |row| {
        Ok(EsdCaseCode {
            id: row.get(0)?,
            short_name: row.get(1)?,
            code: row.get(2)?,
            date: row.get(3)?,
        })
    })?;

    Ok((
        codes_cases.map(|r| r.unwrap()).collect_vec(),
        fulltext_cases.map(|r| r.unwrap()).collect_vec(),
    ))
}

pub fn save_match(match_obj: Match, db_conn: &Connection) -> Result<()> {
    db_conn.execute(
        "
        INSERT INTO matches (source_case, matched_case_id, matched_case_table, matched_value, type)
        VALUES (?1, ?2, ?3, ?4, ?5)
        ",
        params![
            match_obj.source_case,
            match_obj.matched_case_id,
            match_obj.matched_case_table,
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
