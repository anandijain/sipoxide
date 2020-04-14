extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
use std::{
    error::Error,
    fs,
    collections::HashMap,
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

const SCORES_HEADER: &[&'static str; 9] = &[
        "id",
        "sport",
        "status",
        "last_mod",
        "period",
        "secs",
        "is_ticking",
        "a_pts",
        "h_pts",
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
        let rec = scores::Root::to_record(elt);
        // println!("{:#?}", rec);
        wtr.write_record(&rec)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn scores_to_csv(scores: Vec<scores::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;
    wtr.write_record(SCORES_HEADER)?;

    for elt in scores.iter() {
        let rec = scores::Root::to_record(elt);
        wtr.write_record(&rec)?;
        // println!("{:#?}", rec);
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
pub async fn getter(url: String) -> Result<String, reqwest::Error> {
    let res = reqwest::get(&url).await?;
    println!("{} status: {}", url, res.status());
    let body = res.text().await?;
    Ok(body.to_string())
}

pub fn get_scores() -> Option<Vec<scores::Root>> {
    let scores_url = "https://services.bovada.lv/services/sports/results/api/v1/scores/";
    if let Ok(body) = getter(scores_url.to_string()) {
        let scores: Vec<scores::Root> = serde_json::from_str(&body.to_string()).unwrap();
        return Some(scores)
    }
    return None
}

pub fn get_lines() -> Option<Vec<lines::Root>> {
    let lines_url = "https://www.bovada.lv/services/sports/event/v2/events/A/description/";
    if let Ok(body) = getter(lines_url.to_string()) {
        let lines: Vec<lines::Root> = serde_json::from_str(&body.to_string()).unwrap();
        return Some(lines)
    }
    return None
}

// pub fn cmp(t1, t2: HashMap<String, Vec<String>) -> Vec<String> {
    // at the end of cmp, t1 can be entirely deallocd, 
    // t2 must live on, as ret is list of ids, which are new in t2

// }

// pub fn lines_hashmap() -> Option<HashMap<String, &'static csv::StringRecord>> {
//     if let Some(lines_vec) = get_lines(){
//         let mut lines_map : HashMap<String, &'static csv::StringRecord> = HashMap::new();
//         for l in lines_vec.iter(){
//             let recs = lines::Root::to_records(l);
//             for r in recs.iter(){
//                 if let Some(id) = r.get(0) {
//                     lines_map.insert(id.to_string(), &r);
//                 }
//             }
//         }
//         return Some(lines_map)
//     }
//     return None
// }
// pub fn scores_hashmap<'a>(scores_vec: &'a Vec<scores::Root>) -> HashMap<String, &'a scores::Root> {
//     let mut scores_map = HashMap::new();
//     for elt in scores_vec.iter() {
//         let id = &elt.event_id;
//         scores_map.insert(id.to_string(), elt);
//         }
//     scores_map
// }