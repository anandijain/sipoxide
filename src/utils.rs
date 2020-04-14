extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::{
    error::Error,
    fs,
};

use crate::lines;
use crate::scores;

const LINES_HEADER: &[&'static str; 11] = &[
        "oc_id",
        "id",
        "sport",
        "event_desc",
        "dg_desc",
        "mkt_desc",
        "oc_desc",
        "oc_status",
        "oc_type_field",
        "price",
        "hc",
    ];

pub fn test_json_to_csv() {
    json_scores_file_to_csv(
        "./data/scores.json".to_string(),
        "./data/scores.csv".to_string(),
    );
    // json_scores_file_to_csv("./data/root.json".to_string(), "./data/root.csv".to_string());
}

pub fn json_scores_file_to_csv(read_fn: String, write_fn: String) -> Result<(), Box<dyn Error>> {
    let json_data = fs::read_to_string(read_fn)
        .expect("Something went wrong reading the file")
        .to_string();
    let mut wtr = csv::Writer::from_path(write_fn)?;

    let ds: Vec<scores::Root> = serde_json::from_str(&json_data).unwrap();
    for elt in ds.iter() {
        let rec = scores::Root::gen_rec(elt);
        // println!("{:#?}", rec);
        wtr.write_record(rec)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn scores_to_csv(scores: Vec<scores::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;
    wtr.write_record(&[
        "id",
        "sport",
        "status",
        "last_mod",
        "period",
        "secs",
        "is_ticking",
        "a_pts",
        "h_pts",
    ])?;

    for elt in scores.iter() {
        let rec = scores::Root::gen_rec(elt);
        // println!("{:#?}", rec);
        wtr.write_record(rec)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn lines_to_csv(lines: Vec<lines::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;

    wtr.write_record(LINES_HEADER)?;
    
    for s in lines.iter() {
        // s is a Root
        let recs = lines::Root::to_records(s);
        for r in recs.iter(){
            wtr.write_record(r)?;
        }
    }
    wtr.flush()?;
    Ok(())
}

#[tokio::main]
pub async fn get_lines() -> Result<Vec<lines::Root>, reqwest::Error> {
    let lines_url = "https://www.bovada.lv/services/sports/event/v2/events/A/description/";
    let res = reqwest::get(lines_url).await?;
    println!("{} status: {}", lines_url, res.status());
    let body = res.text().await?;
    let lines: Vec<lines::Root> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(lines)
}

#[tokio::main]
pub async fn get_scores() -> Result<Vec<scores::Root>, reqwest::Error> {
    let scores_url = "https://services.bovada.lv/services/sports/results/api/v1/scores/";
    let res = reqwest::get(scores_url).await?;
    println!("{} status: {}", scores_url, res.status());
    let body = res.text().await?;
    let scores: Vec<scores::Root> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(scores)
}

// pub fn lines_hashmap<'a>(ls: &'a Vec<lines::Root>) -> HashMap<String, &'a lines::Event> {
//     let mut lines_map = HashMap::new();
//     for elt in ls.iter() {
//         for event in elt.events.iter() {
//             let id = &event.id;
//             lines_map.insert(id.to_string(), event);
//             // println!("{}: {}", id.to_string(), event.description.to_string());
//         }
//     }
//     lines_map
// }
// pub fn scores_hashmap<'a>(scores_vec: &'a Vec<scores::Root>) -> HashMap<String, &'a scores::Root> {
//     let mut scores_map = HashMap::new();
//     for elt in scores_vec.iter() {
//         let id = &elt.event_id;
//         scores_map.insert(id.to_string(), elt);
//         }
//     scores_map
// }