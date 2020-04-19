extern crate csv;
extern crate serde;
extern crate serde_derive;

use std::collections::HashMap;

mod MLB;
mod lines;
mod scores;
mod utils;

fn main() -> Result<(), csv::Error> {
    // let lines_fn = "./data/lines.csv";
    // let scores_fn = "./data/scores.csv";

    // if let Some(lines_data) = utils::get_lines() {
    //     utils::lines_to_csv(lines_data, lines_fn.to_string());
    // }
    // if let Some(scores_data) = utils::get_scores() {
    //     utils::scores_to_csv(scores_data, scores_fn.to_string());
    // }
    let mut wtr = csv::Writer::from_path("./data/mlb_test_game.csv")?;
    wtr.write_record(vec![
        "play_start".to_string(),
        "play_end".to_string(),
        "inning".to_string(),
        "balls".to_string(),
        "strikes".to_string(),
        "outs".to_string(),
        "a_pts".to_string(),
        "h_pts".to_string(),
        "batter".to_string(),
        "pitcher".to_string(),
    ]);
    if let Ok(body) =
        utils::getter("https://statsapi.mlb.com/api/v1.1/game/607243/feed/live".to_string())
    {
        let root: MLB::Root = serde_json::from_str(&body.to_string()).unwrap();
        let recs: Vec<csv::StringRecord> = MLB::Root::to_records(&root)
            .into_iter()
            .map(|x| csv::StringRecord::from(x))
            .collect();

        for r in recs.iter() {
            wtr.write_record(r);
            wtr.flush();
        }
    }
    Ok(())
}
