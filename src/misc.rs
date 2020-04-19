
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


/*


https://statsapi.mlb.com/api/v1/teams?sportIds=1&hydrate=venue(timezone,location)
https://statsapi.mlb.com/api/v1/schedule?sportId=1&startDate=2010-04-18&endDate=2020-04-18&gameType=E&&gameType=S&&gameType=R&&gameType=F&&gameType=D&&gameType=L&&gameType=W&&gameType=A&hydrate=team(leaders(showOnPreview(leaderCategories=[homeRuns,runsBattedIn,battingAverage],statGroup=[pitching,hitting]))),linescore(matchup,runners),flags,liveLookin,review,broadcasts(all),decisions,person,probablePitcher,stats,homeRuns,previousPlay,game(content(media(featured,epg),summary),tickets),seriesStatus(useOverride=true)&language=en&leagueId=103&&leagueId=104
https://statsapi.mlb.com/api/v1/schedule?sportId=1&startDate=2020-02-16&endDate=2020-04-18&leagueId=103&&leagueId=104&hydrate=venue(timezone,location)
https://statsapi.mlb.com/api/v1.1/game/607243/feed/live

https://stats-prod.nba.com/wp-json/statscms/v1/rotowire/player/?team=bucks&limit=1000
https://stats.nba.com/js/data/widgets/nba_stat_alerts.json
https://data.nba.com/data/10s/v2015/json/mobile_teams/nba/2019/scores/00_todays_scores.json
https://service-apis-us-east-1.prod.nbad.io/onair-agg/v1/schedule/rundown
http://www.nfl.com/ajax/news?partnerId=around-the-league&maxResults=1000
https://gitlab.com/dword4/nhlapi

https://layserbeam-cached.bleacherreport.com/djay/content?url=https://bleacherreport.com/articles/2886509-2020-nfl-free-agents-which-available-players-can-still-be-key-contributors
https://layserbeam-cached.bleacherreport.com/social/tracks/9d71ede936b6adccf451f289413e9df2da3ce5cfb7b94d238cb6334764bd3c33/comments?limit=1000&sort_by=reaction_count
https://api-app.espn.com/v1/watch/clients/watchespn-web/providers
https://secure.espn.com/core/nfl/?xhr=1
*/