use std::iter::once;

use rusqlite::{params, Connection, Result, Transaction};

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
                related_cases: row
                    .rel_id
                    .and_then(|id| row.rel_code.map(|c| vec![EsdRelatedCase { id, code: c }]))
                    .unwrap_or(vec![]),
            });
            result
        });
    Ok(cases)
}

fn do_matches_batch(batch: &[Match], tx: &mut Transaction) -> Result<()> {
    for m in batch {
        tx.execute(
            "
        INSERT INTO matches (source_case, matched_case_id, matched_value, type)
        VALUES (?1, ?2, ?3, ?4)
        ",
            params![m.source_case, m.matched_case_id, m.matched_value, m.m_type,],
        )?;
    }
    Ok(())
}

pub fn save_matches(matches: &Vec<Match>, mut db_conn: Connection) -> Result<(), rusqlite::Error> {
    for chunk in matches.chunks(1000) {
        let mut tx = db_conn.transaction()?;
        do_matches_batch(chunk, &mut tx)?;
        tx.commit()?;
    }
    Ok(())
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
#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::lib::db::{Code, EsdCase, EsdRelatedCase};

    #[test]
    fn test() {
        let case = EsdCase {
            id: 1,
            code: "C-1/01".to_string(),
            date: String::new(),
            full_name: None,
            short_name: String::new(),
            related_cases: vec![
                EsdRelatedCase {
                    id: 2,
                    code: String::from("C-2/01"),
                },
                EsdRelatedCase {
                    id: 3,
                    code: "C-3/01".to_string(),
                },
            ],
        };
        assert_eq!(
            case.get_codes().map(|s| s.as_str()).collect_vec(),
            vec!["C-1/01", "C-2/01", "C-3/01"]
        );
    }
}
