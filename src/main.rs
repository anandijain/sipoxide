extern crate csv;
extern crate futures;

extern crate reqwest;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate tokio;
extern crate regex;

use regex::Regex;
use futures::stream::StreamExt;
use std::error::Error;

mod lines;
mod mlb_schedule;
mod scores;
mod MLB;
mod utils;
use std::future;

fn main() -> Result<(), Box<dyn Error>> {
    // let lines_fn = "./data/lines.csv";
    // let scores_fn = "./data/scores.csv";

    // if let Some(lines_data) = utils::get_lines() {
    //     utils::lines_to_csv(lines_data, lines_fn.to_string());
    // }
    // if let Some(scores_data) = utils::get_scores() {
    //     utils::scores_to_csv(scores_data, scores_fn.to_string());
    // }
    // wtr.write_record(vec![
    //     "play_start".to_string(),
    //     "play_end".to_string(),
    //     "inning".to_string(),
    //     "balls".to_string(),
    //     "strikes".to_string(),
    //     "outs".to_string(),
    //     "a_pts".to_string(),
    //     "h_pts".to_string(),
    //     "batter".to_string(),
    //     "pitcher".to_string(),
    // ]);
    // let last_pk = "492427".to_string();
    // let pks: Vec<String> = utils::readlines("./data/pks.txt").iter().rev();
    // let index = pks
    //     .iter()
    //     .position(|r| r.to_string() == last_pk)
    //     .unwrap();
    // let todo_symbs = &pks[index..pks.len()];
    // for pk in pks.iter() {
    let mut pks: Vec<String> = utils::readlines("./data/pks.txt");
    pks.reverse();

    println!("{:#?}", pks);
    async_urls(pks);
    // utils::mlb_seasons(2000, 2018);
    // utils::mlb_game_plays(pk);
    // utils::mlb_gamedata(pk);

    Ok(())
}

#[tokio::main]
async fn async_urls(pks: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    // let client = reqwest::Client::builder().build()
    // let mut wtr = csv::Writer::from_path(file_name.clone())?;
    // wtr.write_record(&mut bloomberg::PRICE_HEADER.iter());

    let fetches = futures::stream::iter(pks.into_iter().map(|pk| async move {
        match reqwest::get(format!(
                "https://statsapi.mlb.com/api/v1.1/game/{}/feed/live",
                pk.clone().to_string()
            ).as_str()).await {
            Ok(resp) => match resp.json::<MLB::Root>().await {
                Ok(root) => {
                    // let pk = pk_from_url(url.clone());

                    // for r in root.iter() {
                        
                    let file_name = format!("./data/mlb_game_plays/mlb_{}.csv", pk.clone());
                    let recs: Vec<csv::StringRecord> = MLB::Root::to_records(&root)
                        .into_iter()
                        .map(|x| csv::StringRecord::from(x))
                        .collect();
                        
                    // let blank = MLB::Play::to_records(&root);
                    // for pe in blank.iter() {
                    //     let rec = csv::StringRecord::from(pe.as_ref());
                    //     recs.push(rec);
                    // }

                    println!("{}: # records {}", pk, recs.len());
                    println!("RESPONSE: {:#?}", recs);
                    utils::writerecs(
                        file_name.to_string(),
                        &MLB::MLB_LIVE_PLAY_HEADER.to_vec(),
                        recs,
                    );
                    // }
                }
                Err(s) => println!("ERROR reading {} {:#?}", pk.to_string(), s),
            },
            Err(_) => println!("ERROR downloading"),
        }
    }))
    .buffer_unordered(16)
    .collect::<Vec<()>>();
    fetches.await;
    Ok(())
}

// pub fn pk_from_url(s: String) -> String {
//     let re = Regex::new(r"").unwrap();

// }

// #[tokio::main]
// pub fn mlb_game(game_pk: String) -> Result<(), csv::Error> {
//     let mut wtr = csv::Writer::from_path(format!("./data/MLB/mlb_{}.csv", game_pk.to_string()))?;
//     wtr.write_record(MLB::MLB_LIVE_PLAY_HEADER.iter());

//     if let Ok(body) = utils::getter(url) {
//         let root: MLB::Root = serde_json::from_str(&body.to_string()).unwrap();
//         // let recs: Vec<csv::StringRecord> = root
//         //     .live_data
//         //     .plays
//         //     .all_plays
//         //     .into_iter()
//         //     .map(|x| csv::StringRecord::from(MLB::Play::to_record(&x)))
//         //     .collect();
//         let mut recs: Vec<csv::StringRecord> = Vec::new();
//         for p in root.live_data.plays.all_plays.iter() {
//             let blank = MLB::Play::to_records(&p);
//             for pe in blank.iter() {
//                 let rec = csv::StringRecord::from(pe.as_ref());
//                 recs.push(rec);
//             }
//         }
//         // let recs: Vec<csv::StringRecord> = root
//         // .live_data
//         // .plays
//         // .all_plays
//         // .into_iter()
//         // .map(|x| csv::StringRecord::from(MLB::Play::to_record(&x)))
//         // .collect();

//         println!("{:#?}", recs);

//         for r in recs.iter() {
//             wtr.write_record(r);
//         }
//         wtr.flush();
//     }
//     Ok(())
// }
