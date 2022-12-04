use itertools::Itertools;
use serde::Deserialize;
use std::error::Error;

pub struct EsdCase {
    pub ecli: String,
    pub code: String,
    pub codes_list: Vec<String>,
    pub date: String,
}

#[derive(Deserialize, Debug)]
struct EsdCsvLine {
    cjeu_proceeding_id: String,
    ecli: String,
    /// yyyy-MM-DD
    proceeding_date: String,
    cjeu_case_id_list: String,
}

pub fn load_target_cases() -> Result<Vec<EsdCase>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("data/esd_input.csv")?;
    let mut data = vec![];

    for line in reader.deserialize() {
        let line: EsdCsvLine = line?;
        let related_codes = line
            .cjeu_case_id_list
            .split(", ")
            .map(|x| x.to_owned())
            .collect_vec();

        data.push(EsdCase {
            ecli: line.ecli,
            code: line.cjeu_proceeding_id,
            date: line.proceeding_date,
            codes_list: related_codes,
        })
    }

    Ok(data)
}

pub struct Case {
    pub text: String,
    pub id: String,
}
#[derive(Deserialize)]
struct UsCsvLine {
    rozhodnuti_texts: String,
    spisova_znacka: String,
}

pub fn load_us_cases() -> Result<Vec<Case>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path("data/us_input.csv")?;
    let mut data = vec![];

    for line in reader.deserialize() {
        let line: UsCsvLine = line?;
        data.push(Case {
            text: line.rozhodnuti_texts,
            id: line.spisova_znacka,
        });
    }

    Ok(data)
}
