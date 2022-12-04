use std::error::Error;

use crate::rules::Match;

pub fn write_matches_to_file(matches: &Vec<Match>) -> Result<(), Box<dyn Error>> {
    let mut writer = csv::Writer::from_path("data/matches_output.csv")?;

    for m in matches {
        writer.serialize(m)?;
    }

    writer.flush()?;
    Ok(())
}
