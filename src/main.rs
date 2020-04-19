extern crate serde_derive;
extern crate csv;
extern crate serde;

use std::collections::HashMap;

mod utils;
mod lines;
mod scores;
mod MLB;

fn main() {
    let lines_fn = "./data/lines.csv";
    let scores_fn = "./data/scores.csv";

    if let Some(lines_data) = utils::get_lines() {
        utils::lines_to_csv(lines_data, lines_fn.to_string());
    }
    
    if let Some(scores_data) = utils::get_scores() {
        utils::scores_to_csv(scores_data, scores_fn.to_string());
    }

}
