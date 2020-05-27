extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use crate::utils;
/*
https://statsapi.mlb.com/api/v1.1/game/607243/feed/live
https://statsapi.mlb.com/api/v1/schedule?sportId=1&startDate=2020-04-18&endDate=2020-04-18

https://cuts.diamond.mlb.com/FORGE/2020/2020-02/21/b666df28-a14073cb-4cf3f821-csvm-diamondx64-asset_1280x720_59_4000K.mp4
https://www.mlb.com/data-service/en/search?advancedCriteria=%2Fvideos%3Ftags.slug%3Dclassic&limit=100
https://www.mlb.com/data-service/en/videos/tatis-jr-s-15-run-inning
https://www.mlb.com/news-forge/article-data/cole-tucker-has-to-name-son-rhys
*/
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub copyright: String,
    pub game_pk: i64,
    pub link: String,
    pub meta_data: MetaData,
    pub game_data: GameData,
    pub live_data: LiveData,
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new(); 
        for p in self.live_data.plays.all_plays.iter() {
            // for play_rec in Play::to_record(p).iter() {
            let rec: Vec<String> =  Play::to_record(p);
            println!("{:#?}", rec);
        }
        return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaData {
    pub wait: i64,
    pub time_stamp: String,
    pub game_events: Vec<String>,
    pub logical_events: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub game: Game,
    pub datetime: Datetime,
    pub status: Status,
    pub teams: Teams,
    pub players: GameDataPlayers,
    pub venue: Venue,
    pub weather: Weather,
    pub review: Review,
    pub flags: Flags,
    pub alerts: Vec<::serde_json::Value>,
    pub probable_pitchers: ::serde_json::Value,
    pub official_scorer: Option<NamedRef>,
    pub primary_datacaster: Option<NamedRef>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub pk: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub double_header: String,
    pub id: String,
    pub gameday_type: String,
    pub tiebreaker: String,
    pub game_number: i64,
    #[serde(rename = "calendarEventID")]
    pub calendar_event_id: String,
    pub season: String,
    pub season_display: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datetime {
    pub date_time: String,
    pub original_date: String,
    pub day_night: String,
    pub time: String,
    pub ampm: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub abstract_game_state: String,
    pub coded_game_state: String,
    pub detailed_state: String,
    pub status_code: String,
    pub abstract_game_code: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub away: Team,
    pub home: Team,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: i64,
    pub name: Option<String>,
    pub link: String,
    pub season: i64,
    pub venue: General,
    pub team_code: String,
    pub file_code: String,
    pub abbreviation: Option<String>,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub league: General,
    pub division: Option<General>,
    pub sport: General,
    pub short_name: String,
    pub record: General,
    pub spring_league: Option<RefWAbbreviation>,
    pub all_star_status: String,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: Option<String>,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRef {
    pub id: Option<i64>,
    pub full_name: Option<String>,
    pub link: Option<String>,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub games_played: Option<i64>,
    pub wild_card_games_back: String,
    pub league_games_back: String,
    pub spring_league_games_back: String,
    pub sport_games_back: String,
    pub division_games_back: String,
    pub conference_games_back: String,
    pub league_record: LeagueRecord,
    pub records: ::serde_json::Value,
    pub division_leader: bool,
    pub wins: i64,
    pub losses: i64,
    pub winning_percentage: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeagueRecord {
    pub wins: i64,
    pub losses: i64,
    pub pct: String,
}

impl LeagueRecord {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(   
            self.wins.to_string(),
            self.losses.to_string(),
            self.pct.to_string(),
        );
        return rec;
    }
}
    

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefWAbbreviation {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: String,
    pub abbreviation: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDataPlayers {
    #[serde(flatten)]
    players: HashMap<String, GameDataPlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDataPlayer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub primary_number: Option<String>,
    pub birth_date: Option<String>,
    pub current_age: Option<i64>,
    pub birth_city: Option<String>,
    pub birth_country: Option<String>,
    pub height: Option<String>,
    pub weight: i64,
    pub active: bool,
    pub primary_position: Position,
    pub use_name: String,
    pub middle_name: Option<String>,
    pub boxscore_name: String,
    pub nick_name: Option<String>,
    pub gender: String,
    pub name_matrilineal: Option<String>,
    pub is_player: bool,
    pub is_verified: bool,
    pub mlb_debut_date: Option<String>,
    pub bat_side: CodeDesc,
    pub pitch_hand: CodeDesc,
    pub name_first_last: String,
    pub name_slug: String,
    pub first_last_name: String,
    pub last_first_name: String,
    pub last_init_name: String,
    pub init_last_name: String,
    #[serde(rename = "fullFMLName")]
    pub full_fmlname: String,
    #[serde(rename = "fullLFMName")]
    pub full_lfmname: String,
    pub strike_zone_top: f64,
    pub strike_zone_bottom: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: String,
    pub location: Location,
    pub time_zone: TimeZone,
    pub field_info: FieldInfo,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub city: String,
    pub state: Option<String>,
    pub state_abbrev: String,
    pub default_coordinates: Option<LatLongCoords>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatLongCoords {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub id: String,
    pub offset: Option<i64>,
    pub tz: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub capacity: i64,
    pub turf_type: String,
    pub roof_type: String,
    pub left_line: Option<i64>,
    pub left: Option<i64>,
    pub left_center: Option<i64>,
    pub center: Option<i64>,
    pub right_center: Option<i64>,
    pub right_line: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub condition: String,
    pub temp: String,
    pub wind: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    pub has_challenges: bool,
    pub away: Challenges,
    pub home: Challenges,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenges {
    pub used: i64,
    pub remaining: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Flags {
    pub no_hitter: bool,
    pub perfect_game: bool,
    pub away_team_no_hitter: bool,
    pub away_team_perfect_game: bool,
    pub home_team_no_hitter: bool,
    pub home_team_perfect_game: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveData {
    pub plays: Plays,
    pub linescore: Linescore,
    pub boxscore: Boxscore,
    pub decisions: Decisions,
    pub leaders: Leaders,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plays {
    pub all_plays: Vec<Play>,
    pub current_play: Option<Play>,
    pub scoring_plays: Vec<i64>,
    pub plays_by_inning: Vec<PlaysByInning>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Play {
    pub result: PlayResult,
    pub about: About,
    pub count: Count,
    pub matchup: Matchup,
    pub pitch_index: Vec<i64>,
    pub action_index: Vec<i64>,
    pub runner_index: Vec<i64>,
    pub runners: Vec<Runner>,
    pub play_events: Vec<PlayEvent>,
    pub at_bat_index: i64,
    pub play_end_time: String,
}




impl Play {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for play in self.play_events.iter() {
            let mut rec: Vec<String> = Play::to_record(self);
            rec.append(&mut PlayEvent::to_record(play));
            recs.push(rec);
        }
        println!("{:#?}", recs);
        return recs;
    }

    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec!(
            self.about.start_time.to_string(),
            self.about.end_time.to_string(),
            self.about.inning.to_string(),
            utils::lilmatcher_i64(self.count.balls).to_string(),
            utils::lilmatcher_i64(self.count.strikes).to_string(),
            utils::lilmatcher_i64(self.count.outs).to_string(),
            self.result.away_score.to_string(),
            self.result.home_score.to_string(),
            // utils::lilmatcher(self.matchup.batter.full_name.clone()).to_string(),
            // utils::lilmatcher(self.matchup.pitcher.full_name.clone()).to_string(),
            self.at_bat_index.to_string(),
        );
        return rec;
    }

}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayResult {
    #[serde(rename = "type")]
    pub type_field: String,
    pub event: String,
    pub event_type: String,
    pub description: String,
    pub rbi: i64,
    pub away_score: i64,
    pub home_score: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct About {
    pub at_bat_index: i64,
    pub half_inning: String,
    pub is_top_inning: bool,
    pub inning: i64,
    pub start_time: String,
    pub end_time: String,
    pub is_complete: bool,
    pub is_scoring_play: bool,
    pub has_review: bool,
    pub has_out: bool,
    pub captivating_index: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Count {
    pub balls: Option<i64>,
    pub strikes: Option<i64>,
    pub outs: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matchup {
    pub batter: NamedRef,
    pub bat_side: CodeDesc,
    pub pitcher: Option<NamedRef>,
    pub pitch_hand: CodeDesc,
    pub post_on_first: Option<NamedRef>,
    pub batter_hot_cold_zones: Vec<::serde_json::Value>,
    pub pitcher_hot_cold_zones: Vec<::serde_json::Value>,
    pub splits: Splits,
    pub post_on_second: Option<NamedRef>,
    pub post_on_third: Option<NamedRef>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Splits {
    pub batter: String,
    pub pitcher: Option<String>,
    pub men_on_base: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Runner {
    pub movement: Movement,
    pub details: RunnerDetails,
    #[serde(default)]
    pub credits: Vec<Credit>,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Credit {
    pub player: QuickRef,
    pub position: Position,
    pub credit: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Movement {
    pub start: Option<String>,
    pub end: Option<String>,
    pub out_base: Option<String>,
    pub is_out: Option<bool>,
    pub out_number: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickRef {
    // ResponsiblePitcher Player
    pub id: Option<i64>,
    pub link: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayEvent {
    pub details: PlayEventDetails,
    pub count: Count,
    pub index: i64,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub is_pitch: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub player: Option<QuickRef>,
    pub play_id: Option<String>,
    pub action_play_id: Option<String>,
    pub pitch_data: Option<PitchData>,
    pub pitch_number: Option<i64>,
    pub hit_data: Option<HitData>,
    pub position: Option<Position>,
    pub batting_order: Option<String>,
}

impl PlayEvent {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec!(
            self.index.to_string(),
            self.is_pitch.to_string(),
            utils::lilmatcher_string(self.play_id.clone()),
            utils::lilmatcher_i64(self.pitch_number.clone()),
            utils::lilmatcher_string(self.action_play_id.clone()),
            utils::lilmatcher_string(self.batting_order.clone()),
        );
        if let Some(pitch) = &self.pitch_data {
            rec.append(&mut PitchCoords::to_record(&pitch.coordinates));
        } else {
            rec.append(&mut vec!(
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ));
        }
        if let Some(hit) = &self.hit_data {
            rec.append(&mut HitData::to_record(&hit));
        } else {
            rec.append(&mut vec!(
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ));
        }
        if let Some(pos) = &self.position {
            rec.append(&mut Position::to_record(&pos));
        } else {
            rec.append(&mut vec!(
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ));
        }
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PitchData {
    pub strike_zone_top: f64,
    pub strike_zone_bottom: f64,
    pub coordinates: PitchCoords,
    pub breaks: ::serde_json::Value,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct XYCoords {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PitchCoords {
    pub a_y: Option<f64>,
    pub a_z: Option<f64>,
    pub pfx_x: Option<f64>,
    pub pfx_z: Option<f64>,
    pub p_x: Option<f64>,
    pub p_z: Option<f64>,
    pub v_x0: Option<f64>,
    pub v_y0: Option<f64>,
    pub v_z0: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub x0: Option<f64>,
    pub y0: Option<f64>,
    pub z0: Option<f64>,
    pub a_x: Option<f64>,
}

impl PitchCoords {
    pub fn to_record(&self) ->Vec<String> {
        let rec: Vec<String> = vec!(
            utils::lilmatcher_f64(self.a_y),
            utils::lilmatcher_f64(self.a_z),
            utils::lilmatcher_f64(self.pfx_x),
            utils::lilmatcher_f64(self.pfx_z),
            utils::lilmatcher_f64(self.p_x),
            utils::lilmatcher_f64(self.p_z),
            utils::lilmatcher_f64(self.v_x0),
            utils::lilmatcher_f64(self.v_y0),
            utils::lilmatcher_f64(self.v_z0),
            utils::lilmatcher_f64(self.x),
            utils::lilmatcher_f64(self.y),
            utils::lilmatcher_f64(self.x0),
            utils::lilmatcher_f64(self.y0),
            utils::lilmatcher_f64(self.z0),
            utils::lilmatcher_f64(self.a_x)
    );
    return rec;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoordsXy {
    pub coord_x: f64,
    pub coord_y: f64,
}

impl CoordsXy {
    pub fn to_record(&self) ->Vec<String> {
        let rec: Vec<String> = vec!(
            self.coord_x.to_string(),
            self.coord_y.to_string(),
    );
    return rec;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitData {
    pub trajectory: String,
    pub hardness: String,
    pub location: Option<String>,
    pub coordinates: CoordsXy,
}

impl HitData {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec!(
            self.trajectory.to_string(),
            self.hardness.to_string(),
            utils::lilmatcher(self.location.clone()),
        );
        rec.append(&mut CoordsXy::to_record(&self.coordinates));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayEventDetails {
    pub description: String,
    pub event: Option<String>,
    pub event_type: Option<String>,
    pub away_score: Option<i64>,
    pub home_score: Option<i64>,
    pub is_scoring_play: Option<bool>,
    pub has_review: bool,
    pub code: Option<String>,
    pub from_catcher: Option<bool>,
    pub call: Option<CodeDesc>,
    pub ball_color: Option<String>,
    pub is_in_play: Option<bool>,
    pub is_strike: Option<bool>,
    pub is_ball: Option<bool>,
    pub runner_going: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunnerDetails {
    pub event: String,
    pub event_type: String,
    pub movement_reason: Option<String>,
    pub runner: NamedRef,
    pub responsible_pitcher: Option<QuickRef>,
    pub is_scoring_event: bool,
    pub rbi: bool,
    pub earned: bool,
    pub team_unearned: bool,
    pub play_index: i64,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaysByInning {
    pub start_index: i64,
    pub end_index: i64,
    pub top: Vec<i64>,
    pub bottom: Vec<i64>,
    pub hits: Hits,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hits {
    pub away: Vec<TeamHits>,
    pub home: Vec<TeamHits>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamHits {
    pub team: BoxTeamRef,
    pub inning: i64,
    pub pitcher: Option<NamedRef>,
    pub batter: NamedRef,
    pub coordinates: XYCoords,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    pub note: Option<String>,
    pub current_inning: Option<i64>,
    pub current_inning_ordinal: Option<String>,
    pub inning_state: Option<String>,
    pub inning_half: Option<String>,
    pub is_top_inning: Option<bool>,
    pub scheduled_innings: Option<i64>,
    pub innings: Option<Vec<Inning>>,
    pub teams: Option<TeamLinescores>,
    pub defense: Option<Defense>,
    pub offense: Option<Offense>,
    pub balls: Option<i64>,
    pub strikes: Option<i64>,
    pub outs: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inning {
    pub num: i64,
    pub ordinal_num: String,
    pub home: InningScore,
    pub away: InningScore,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InningScore {
    pub runs: Option<i64>,
    pub hits: i64,
    pub errors: i64,
    pub left_on_base: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamLinescores {
    pub home: InningStatus,
    pub away: InningStatus,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InningStatus {
    pub runs: Option<i64>,
    pub hits: Option<i64>,
    pub errors: Option<i64>,
    pub left_on_base: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defense {
    pub pitcher: Option<NamedRef>,
    pub catcher: Option<NamedRef>,
    pub first: Option<NamedRef>,
    pub second: Option<NamedRef>,
    pub third: Option<NamedRef>,
    pub shortstop: Option<NamedRef>,
    pub left: Option<NamedRef>,
    pub center: Option<NamedRef>,
    pub right: Option<NamedRef>,
    pub batter: Option<NamedRef>,
    pub on_deck: Option<NamedRef>,
    pub in_hole: Option<NamedRef>,
    pub team: Option<General>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offense {
    pub batter: Option<NamedRef>,
    pub on_deck: Option<NamedRef>,
    pub in_hole: Option<NamedRef>,
    pub pitcher: Option<NamedRef>,
    pub team: Option<General>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Boxscore {
    pub teams: BoxscoreTeams,
    pub officials: Vec<Official>,
    pub info: Vec<Note>,
    pub pitching_notes: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxscoreTeams {
    pub away: BoxTeam,
    pub home: BoxTeam,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatsBlock {
    pub batting: Option<Batting>,
    pub pitching: Option<Pitching>,
    pub fielding: Option<Fielding>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxTeam {
    pub team: BoxTeamRef,
    pub team_stats: Option<StatsBlock>,
    pub players: BoxscorePlayers,
    pub batters: Vec<i64>,
    pub pitchers: Vec<i64>,
    pub bench: Vec<i64>,
    pub bullpen: Vec<i64>,
    pub batting_order: Vec<i64>,
    pub info: Vec<Info>,
    pub note: Vec<Note>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxTeamRef {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub link: String,
    pub spring_league: Option<RefWAbbreviation>,
    pub all_star_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxscorePlayers {
    #[serde(flatten)]
    pub players: HashMap<String, BoxscorePlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGameStatus {
    pub is_current_batter: bool,
    pub is_current_pitcher: bool,
    pub is_on_bench: bool,
    pub is_substitute: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub field_list: Vec<Note>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub label: Option<String>,
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDesc {
    pub code: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub code: String,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub abbreviation: String,
}

impl Position {
    pub fn to_record(&self) ->Vec<String> {
        let rec: Vec<String> = vec![
            self.code.to_string(),
            utils::lilmatcher(self.name.clone()),
            self.type_field.to_string(),
            self.abbreviation.to_string(),
        ];
    return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Official {
    pub official: NamedRef,
    pub official_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Decisions {
    pub winner: Option<NamedRef>,
    pub loser: Option<NamedRef>,
    pub save: Option<NamedRef>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Leaders {
    pub hit_distance: ::serde_json::Value,
    pub hit_speed: ::serde_json::Value,
    pub pitch_speed: ::serde_json::Value,
}

// batting
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Batting {
    pub games_played: Option<i64>,
    pub fly_outs: Option<i64>,
    pub ground_outs: Option<i64>,
    pub runs: Option<i64>,
    pub doubles: Option<i64>,
    pub triples: Option<i64>,
    pub home_runs: Option<i64>,
    pub strike_outs: Option<i64>,
    pub base_on_balls: Option<i64>,
    pub intentional_walks: Option<i64>,
    pub hits: Option<i64>,
    pub hit_by_pitch: Option<i64>,
    pub avg: Option<String>,
    pub at_bats: Option<i64>,
    pub obp: Option<String>,
    pub slg: Option<String>,
    pub ops: Option<String>,
    pub caught_stealing: Option<i64>,
    pub stolen_bases: Option<i64>,
    pub stolen_base_percentage: Option<String>,
    pub ground_into_double_play: Option<i64>,
    pub ground_into_triple_play: Option<i64>,
    pub plate_appearances: Option<i64>,
    pub total_bases: Option<i64>,
    pub rbi: Option<i64>,
    pub left_on_base: Option<i64>,
    pub sac_bunts: Option<i64>,
    pub sac_flies: Option<i64>,
    pub babip: Option<String>,
    pub catchers_interference: Option<i64>,
    pub pickoffs: Option<i64>,
    pub at_bats_per_home_run: Option<String>,
}

//pitching
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pitching {
    pub games_played: Option<i64>,
    pub games_started: Option<i64>,
    pub ground_outs: Option<i64>,
    pub air_outs: Option<i64>,
    pub runs: Option<i64>,
    pub doubles: Option<i64>,
    pub triples: Option<i64>,
    pub home_runs: Option<i64>,
    pub strike_outs: Option<i64>,
    pub base_on_balls: Option<i64>,
    pub intentional_walks: Option<i64>,
    pub hits: Option<i64>,
    pub hit_by_pitch: Option<i64>,
    pub at_bats: Option<i64>,
    pub obp: Option<String>,
    pub caught_stealing: Option<i64>,
    pub stolen_bases: Option<i64>,
    pub stolen_base_percentage: Option<String>,
    pub era: Option<String>,
    pub innings_pitched: Option<String>,
    pub wins: Option<i64>,
    pub losses: Option<i64>,
    pub saves: Option<i64>,
    pub save_opportunities: Option<i64>,
    pub holds: Option<i64>,
    pub blown_saves: Option<i64>,
    pub earned_runs: Option<i64>,
    pub whip: Option<String>,
    pub outs: Option<i64>,
    pub games_pitched: Option<i64>,
    pub complete_games: Option<i64>,
    pub shutouts: Option<i64>,
    pub hit_batsmen: Option<i64>,
    pub balks: Option<i64>,
    pub wild_pitches: Option<i64>,
    pub pickoffs: Option<i64>,
    pub ground_outs_to_airouts: Option<String>,
    pub rbi: Option<i64>,
    pub win_percentage: Option<String>,
    pub games_finished: Option<i64>,
    pub strikeout_walk_ratio: Option<String>,
    pub strikeouts_per9_inn: Option<String>,
    pub walks_per9_inn: Option<String>,
    pub hits_per9_inn: Option<String>,
    pub runs_scored_per9: Option<String>,
    pub home_runs_per9: Option<String>,
    pub inherited_runners: Option<i64>,
    pub inherited_runners_scored: Option<i64>,
    pub catchers_interference: Option<i64>,
    pub sac_bunts: Option<i64>,
    pub sac_flies: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fielding {
    pub assists: Option<i64>,
    pub put_outs: Option<i64>,
    pub errors: Option<i64>,
    pub chances: Option<i64>,
    pub caught_stealing: Option<i64>,
    pub passed_ball: Option<i64>,
    pub stolen_bases: Option<i64>,
    pub stolen_base_percentage: Option<String>,
    pub pickoffs: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxscorePlayer {
    pub person: NamedRef,
    pub jersey_number: Option<String>,
    pub position: Position,
    pub stats: Option<StatsBlock>,
    pub status: CodeDesc,
    pub parent_team_id: Option<i64>,
    pub season_stats: Option<StatsBlock>,
    pub game_status: PlayerGameStatus,
}


pub const MLB_LIVE_PLAY_HEADER: [&'static str; 39] = [
"start_time",
"end_time",
"inning",
"balls",
"strikes",
"outs",
"away_score",
"home_score",
// "batter",
// "pitcher",
"at_bat_index",
"index",
"is_pitch",
"play_id",
"pitch_number",
"action_play_id",
"batting_order",
"a_y",
"a_z",
"pfx_x",
"pfx_z",
"p_x",
"p_z",
"v_x0",
"v_y0",
"v_z0",
"x",
"y",
"x0",
"y0",
"z0",
"a_x",
"trajectory",
"hardness",
"location",
"hit_x",
"hit_y",
"pos_code",
"pos_name",
"pos_type_field",
"pos_abbreviation",
];