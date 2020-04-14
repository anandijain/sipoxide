extern crate serde_derive;
extern crate csv;
extern crate serde;

mod utils;
mod lines;
mod scores;


fn main() {
    let lines_fn = "./data/lines.csv";
    let scores_fn = "./data/scores.csv";
    if let Ok(lines_data) = utils::get_lines() {
        utils::lines_to_csv(lines_data, lines_fn.to_string());
        // let lm = lines_hashmap(&lines_data);
        // for (id, ev) in &lm{
        //     println!("{}: {} {}", id, ev.sport, ev.description);
        // }
    }
    if let Ok(scores_data) = utils::get_scores() {
        utils::scores_to_csv(scores_data, scores_fn.to_string());
        // let sm = scores_hashmap(&scores_data);
        // for (id, ev) in &sm{
        //     println!("{}: {} {}", id, ev.sport, ev.game_status);
        // }
    }

}
