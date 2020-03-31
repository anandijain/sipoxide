#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate csv;
extern crate serde;
use rocket_contrib::json::Json;
use std::error::Error;
use std::fs;

mod lines;
mod scores;

fn test_json_to_csv() {
    json_scores_file_to_csv(
        "./data/scores.json".to_string(),
        "./data/scores.csv".to_string(),
    );
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
        // println!("{:#?}", rec);
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

fn lines_to_csv(lines: Vec<lines::Root>, write_fn: String) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(write_fn)?;

    wtr.write_record(&[
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
                                &oc.status,
                                &oc.type_field,
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
                                &oc.status,
                                &oc.type_field,
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
async fn lines() -> Result<Vec<lines::Root>, reqwest::Error> {
    let lines_url = "https://www.bovada.lv/services/sports/event/v2/events/A/description/";
    let lines_fn = "./data/root.csv";
    let res = reqwest::get(lines_url).await?;
    println!("{} status: {}", lines_url, res.status());
    let body = res.text().await?;
    let lines: Vec<lines::Root> = serde_json::from_str(&body.to_string()).unwrap();
    // if let Err(err) = lines_to_csv(lines, lines_fn.to_string()) {
    //     println!("{}", err);
    // }

     Ok(lines)
}

#[tokio::main]
async fn scores() -> Result<Vec<scores::Root>, reqwest::Error> {
    let scores_url = "https://services.bovada.lv/services/sports/results/api/v1/scores/";
    let scores_fn = "./data/scores.csv";
    let res = reqwest::get(scores_url).await?;
    println!("{} status: {}", scores_url, res.status());
    let body = res.text().await?;
    let scores: Vec<scores::Root> = serde_json::from_str(&body.to_string()).unwrap();
    Ok(scores)
}

fn main() {
    let l = lines();
    // let s = scores();
    // for elt in l.iter() {
    //     println!("{:#?}", elt)
    // }
    rkt_main()
}

#[get("/")]
fn index () -> &'static str {
    "hello"
}

#[get("/scores")]
fn route_scores() -> Json<Vec<scores::Root>>  {
    // let fn = "./data/scores.csv";
    // let file = File::open(fn)?;
    // let mut rdr = csv::Reader::from_reader(file);
    let s = scores(); 
    for elt in &s {
        println!("{:?}", elt);
    }
    match s {
        Ok(s) => Json(s),
        _ => Json(vec![])
    }
}

#[get("/lines")]
fn route_lines() -> Json<Vec<lines::Root>>  {
    // let fn = "./data/scores.csv";
    // let file = File::open(fn)?;
    // let mut rdr = csv::Reader::from_reader(file);
    let s = lines(); 
    // for elt in &s {
    //     println!("{:?}", elt);
    // }
    match s {
        Ok(s) => Json(s),
        _ => Json(vec![])
    }
}
fn rkt_main() {
    rocket::ignite().mount("/", routes![index, route_scores, route_lines]).launch();
}

