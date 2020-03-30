#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate serde;
use std::error::Error;
use std::fs;

mod bov;
mod scores;

fn test_json_to_csv() {
    json_scores_file_to_csv("./data/scores.json".to_string(), "./data/scores.csv".to_string());
    // json_scores_file_to_csv("./data/root.json".to_string(), "./data/root.csv".to_string());
}

fn json_scores_file_to_csv(read_fn: String, write_fn: String) -> Result<(), Box<dyn Error>> {
    let json_data = fs::read_to_string(read_fn)
        .expect("Something went wrong reading the file")
        .to_string();
    let mut wtr = csv::Writer::from_path(write_fn)?;

    let ds: Vec<scores::Root> = serde_json::from_str(&json_data).unwrap();
    for elt in ds.iter() {
            let rec = scores::Root::gen_rec(elt);
            println!("{:#?}", rec);
            wtr.write_record(rec)?;
    }
    wtr.flush()?;
    Ok(())
}

fn scores_to_csv(scores: Vec<scores::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {
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
        "h_pts"
    ])?;

    for elt in scores.iter() {
            let rec = scores::Root::gen_rec(elt);
            println!("{:#?}", rec);
            wtr.write_record(rec)?;
        }
    wtr.flush()?;
    Ok(())
}

fn lines_to_csv(lines: Vec<bov::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {

    let mut wtr = csv::Writer::from_path(write_fn)?;

    wtr.write_record(&[
        "id",
        "sport",
        "event_desc",
        "dg_desc",
        "mkt_desc",
        "oc_desc",
        "price",
        "hc",
    ])?;

    for s in lines.iter() {
        // s is a Root
        for e in s.events.iter() {
            for dg in e.display_groups.iter() {
                for m in dg.markets.iter() {
                    for oc in m.outcomes.iter() {
                        if let Some(hc) = &oc.price.handicap {
                            let rec = &[
                                &e.id,
                                &e.sport,
                                &e.description,
                                &dg.description,
                                &m.description,
                                &oc.description,
                                &oc.price.decimal,
                                &hc.to_string(),
                            ];
                            wtr.write_record(rec)?;
                            println!("{:#?}", rec);
                        } else {
                            let rec = &[
                                &e.id,
                                &e.sport,
                                &e.description,
                                &dg.description,
                                &m.description,
                                &oc.description,
                                &oc.price.decimal,
                                &"".to_string(),
                            ];
                            wtr.write_record(rec)?;
                            println!("{:#?}", rec);
                        }
                    }
                }
            }
        }
    }
    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let lines_url = "https://www.bovada.lv/services/sports/event/v2/events/A/description/";
    let scores_url = "https://services.bovada.lv/services/sports/results/api/v1/scores/";

    let lines_fn = "./data/root.csv";
    let scores_fn = "./data/scores.csv";

    let res = reqwest::get(scores_url).await?;
    println!("{} status: {}", scores_url, res.status());
    let body = res.text().await?;
    let scores: Vec<scores::Root> = serde_json::from_str(&body.to_string()).unwrap();
    if let Err(err) = scores_to_csv(scores, scores_fn.to_string()) {
        println!("{}", err);
    }

    let res = reqwest::get(lines_url).await?;
    println!("{} status: {}", lines_url, res.status());
    let body = res.text().await?;
    let lines: Vec<bov::Root> = serde_json::from_str(&body.to_string()).unwrap();
    if let Err(err) = lines_to_csv(lines, lines_fn.to_string()) {
        println!("{}", err);
    }

    Ok(())
}



