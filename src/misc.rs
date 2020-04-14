
#[get("/")]
fn index() -> &'static str {
    "hello"
}

#[get("/scores")]
fn route_scores() -> Json<Vec<scores::Root>> {
    let s = get_scores();
    match s {
        Ok(s) => Json(s),
        _ => Json(vec![]),
    }
}

#[get("/scorescsv")]
fn route_scores_csv() -> String {
    let d = fs::read_to_string("./data/scores.csv").expect("error in reading to string");
    return d;
}

#[get("/lines")]
fn route_lines() -> Json<Vec<lines::Root>> {
    let s = get_lines();
    match s {
        Ok(s) => Json(s),
        _ => Json(vec![]),
    }
}

#[get("/linescsv")]
fn route_lines_csv() -> String {
    let d = fs::read_to_string("./data/lines.csv").expect("error in reading to string");
    return d;
}

#[get("/joined_csv")]
fn route_joined_csv() -> String {
    let d = fs::read_to_string("./data/bov_joined.csv").expect("error in reading to string");
    return d;
}

fn rkt_main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                route_scores,
                route_scores_csv,
                route_lines,
                route_lines_csv,
                route_joined_csv
            ],
        )
        .launch();
}
