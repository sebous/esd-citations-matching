use itertools::Itertools;
use rusqlite::{params, Connection, Result, Transaction};

pub trait Code {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_>;
}

impl Code for EsdCase {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.case_infos.iter().map(|x| &x.code))
    }
}
impl Code for Vec<EsdCase> {
    fn get_codes(&self) -> Box<dyn Iterator<Item = &String> + '_> {
        Box::new(self.iter().flat_map(|case| case.get_codes()))
    }
}

#[derive(Debug)]
pub struct EsdCase {
    pub id: usize,
    pub ecli: String,
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub case_infos: Vec<EsdCaseInfo>,
}

#[derive(Debug)]
pub struct EsdCaseInfo {
    pub id: usize,
    pub code: String,
}

#[derive(Debug)]
pub struct Match {
    pub source_case_id: usize,
    pub matched_case_id: usize,
    pub matched_value: String,
    pub m_type: String,
}

#[derive(Debug)]
pub struct SourceCase {
    pub id: usize,
    pub file_name: String,
}

struct EsdJoinedCaseDbRow {
    pub id: usize,
    pub ecli: String,
    pub short_name: Option<String>,
    pub full_name: Option<String>,
    pub info_id: Option<usize>,
    pub info_code: Option<String>,
}

pub fn fetch_data(db_conn: &Connection) -> Result<(Vec<EsdCase>, Vec<SourceCase>)> {
    let mut esd_query = db_conn
        .prepare(
            "
            SELECT C.id, C.ecli, C.short_name, C.full_name, I.id, I.code
            FROM esdcases C
            LEFT OUTER JOIN esdcaseinfos I ON C.id=I.case_id
            ORDER BY C.id
    ",
        )
        .unwrap();

    let esd_cases = esd_query
        .query_map([], |row| {
            Ok(EsdJoinedCaseDbRow {
                id: row.get(0).unwrap(),
                ecli: row.get(1).unwrap(),
                short_name: row.get(2).unwrap_or(None),
                full_name: row.get(3).unwrap_or(None),
                info_id: row.get(4).unwrap(),
                info_code: row.get(5).unwrap(),
            })
        })?
        .map(|r| r.unwrap())
        .fold(vec![], |mut result: Vec<EsdCase>, row| {
            // join creates a duplicated rows, append case infos to last EsdCase
            let last_item = result.iter().last();
            if last_item.is_some()
                && last_item.unwrap().id == row.id
                && row.info_id.is_some()
                && row.info_code.is_some()
            {
                result.last_mut().unwrap().case_infos.push(EsdCaseInfo {
                    id: row.info_id.unwrap(),
                    code: row.info_code.unwrap(),
                });
                return result;
            }

            result.push(EsdCase {
                id: row.id,
                ecli: row.ecli,
                short_name: row.short_name,
                full_name: row.full_name,
                case_infos: match row.info_id {
                    Some(info_id) => vec![EsdCaseInfo {
                        id: info_id,
                        code: row.info_code.unwrap(),
                    }],
                    None => vec![],
                },
            });
            result
        });

    let mut source_cases_query = db_conn
        .prepare(
            "
            SELECT id, file_name
            FROM sourcecases
            ORDER BY id
        ",
        )
        .unwrap();
    let source_cases = source_cases_query
        .query_map([], |row| {
            Ok(SourceCase {
                id: row.get(0).unwrap(),
                file_name: row.get(1).unwrap(),
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();

    Ok((esd_cases, source_cases))
}

fn save_matches_batch(batch: &[Match], tx: &mut Transaction) -> Result<()> {
    for m in batch {
        tx.execute(
            "
        INSERT INTO matches (source_case_id, matched_case_id, matched_value, type)
        VALUES (?1, ?2, ?3, ?4)
        ",
            params![
                m.source_case_id,
                m.matched_case_id,
                m.matched_value,
                m.m_type,
            ],
        )?;
    }
    Ok(())
}

pub fn save_matches(matches: &Vec<Match>, mut db_conn: Connection) -> Result<(), rusqlite::Error> {
    for chunk in matches.chunks(1000) {
        let mut tx = db_conn.transaction()?;
        save_matches_batch(chunk, &mut tx)?;
        tx.commit()?;
    }
    Ok(())
}

pub fn save_match(match_obj: Match, db_conn: &Connection) -> Result<()> {
    db_conn.execute(
        "
        INSERT INTO matches (source_case_id, matched_case_id, matched_value, type)
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![
            match_obj.source_case_id,
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

    use crate::lib::db::{Code, EsdCase, EsdCaseInfo};

    #[test]
    fn test() {
        let case = EsdCase {
            id: 1,
            ecli: String::new(),
            full_name: None,
            short_name: None,
            case_infos: vec![
                EsdCaseInfo {
                    id: 2,
                    code: String::from("C-2/01"),
                },
                EsdCaseInfo {
                    id: 3,
                    code: String::from("C-3/01"),
                },
            ],
        };
        assert_eq!(
            case.get_codes().map(|s| s.as_str()).collect_vec(),
            vec!["C-2/01", "C-3/01"]
        );
    }
}
