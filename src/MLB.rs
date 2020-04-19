extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;

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
    pub official_scorer: NamedRef,
    pub primary_datacaster: NamedRef,
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
    pub name: String,
    pub link: String,
    pub season: i64,
    pub venue: General,
    pub team_code: String,
    pub file_code: String,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub league: General,
    pub division: General,
    pub sport: General,
    pub short_name: String,
    pub record: General,
    pub spring_league: RefWAbbreviation,
    pub all_star_status: String,
    pub active: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub id: i64,
    pub name: String,
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NamedRef {
    pub id: i64,
    pub full_name: String,
    pub link: String,
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub games_played: i64,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefWAbbreviation {
    pub id: i64,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
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
    pub primary_number: String,
    pub birth_date: String,
    pub current_age: i64,
    pub birth_city: String,
    pub birth_country: String,
    pub height: String,
    pub weight: i64,
    pub active: bool,
    pub primary_position: Position,
    pub use_name: String,
    pub middle_name: String,
    pub boxscore_name: String,
    pub nick_name: String,
    pub gender: String,
    pub name_matrilineal: String,
    pub is_player: bool,
    pub is_verified: bool,
    pub mlb_debut_date: String,
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
    pub id: i64,
    pub name: String,
    pub link: String,
    pub location: Location,
    pub time_zone: TimeZone,
    pub field_info: FieldInfo,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub city: String,
    pub state: String,
    pub state_abbrev: String,
    pub default_coordinates: LatLongCoords,
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
    pub offset: i64,
    pub tz: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub capacity: i64,
    pub turf_type: String,
    pub roof_type: String,
    pub left_line: i64,
    pub left: i64,
    pub left_center: i64,
    pub center: i64,
    pub right_center: i64,
    pub right_line: i64,
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
    pub current_play: Play,
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
    pub balls: i64,
    pub strikes: i64,
    pub outs: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Matchup {
    pub batter: NamedRef,
    pub bat_side: CodeDesc,
    pub pitcher: NamedRef,
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
    pub pitcher: String,
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
    pub id: i64,
    pub link: String,
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

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PitchData {
    pub strike_zone_top: f64,
    pub strike_zone_bottom: f64,
    pub coordinates: XYCoords,
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
pub struct CoordsXy {
    pub coord_x: f64,
    pub coord_y: f64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HitData {
    pub trajectory: String,
    pub hardness: String,
    pub location: String,
    pub coordinates: CoordsXy,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPlay {
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
    pub pitcher: NamedRef,
    pub batter: NamedRef,
    pub coordinates: XYCoords,
    #[serde(rename = "type")]
    pub type_field: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    pub note: String,
    pub current_inning: i64,
    pub current_inning_ordinal: String,
    pub inning_state: String,
    pub inning_half: String,
    pub is_top_inning: bool,
    pub scheduled_innings: i64,
    pub innings: Vec<Inning>,
    pub teams: TeamLinescores,
    pub defense: Defense,
    pub offense: Offense,
    pub balls: i64,
    pub strikes: i64,
    pub outs: i64,
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
    pub runs: i64,
    pub hits: i64,
    pub errors: i64,
    pub left_on_base: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defense {
    pub pitcher: NamedRef,
    pub catcher: NamedRef,
    pub first: NamedRef,
    pub second: NamedRef,
    pub third: NamedRef,
    pub shortstop: NamedRef,
    pub left: NamedRef,
    pub center: NamedRef,
    pub right: NamedRef,
    pub batter: NamedRef,
    pub on_deck: NamedRef,
    pub in_hole: NamedRef,
    pub team: General,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Offense {
    pub batter: NamedRef,
    pub on_deck: NamedRef,
    pub in_hole: NamedRef,
    pub pitcher: NamedRef,
    pub team: General,
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
    pub batting: Batting,
    pub pitching: Pitching,
    pub fielding: Fielding,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxTeam {
    pub team: BoxTeamRef,
    pub team_stats: StatsBlock,
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
    pub id: i64,
    pub name: String,
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
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub abbreviation: String,
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
    pub winner: NamedRef,
    pub loser: NamedRef,
    pub save: NamedRef,
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
    pub games_played: i64,
    pub fly_outs: i64,
    pub ground_outs: i64,
    pub runs: i64,
    pub doubles: i64,
    pub triples: i64,
    pub home_runs: i64,
    pub strike_outs: i64,
    pub base_on_balls: i64,
    pub intentional_walks: i64,
    pub hits: i64,
    pub hit_by_pitch: i64,
    pub avg: String,
    pub at_bats: i64,
    pub obp: String,
    pub slg: String,
    pub ops: String,
    pub caught_stealing: i64,
    pub stolen_bases: i64,
    pub stolen_base_percentage: String,
    pub ground_into_double_play: i64,
    pub ground_into_triple_play: i64,
    pub plate_appearances: i64,
    pub total_bases: i64,
    pub rbi: i64,
    pub left_on_base: i64,
    pub sac_bunts: i64,
    pub sac_flies: i64,
    pub babip: String,
    pub catchers_interference: i64,
    pub pickoffs: i64,
    pub at_bats_per_home_run: String,
}

//pitching
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pitching {
    pub games_played: i64,
    pub games_started: i64,
    pub ground_outs: i64,
    pub air_outs: i64,
    pub runs: i64,
    pub doubles: i64,
    pub triples: i64,
    pub home_runs: i64,
    pub strike_outs: i64,
    pub base_on_balls: i64,
    pub intentional_walks: i64,
    pub hits: i64,
    pub hit_by_pitch: i64,
    pub at_bats: i64,
    pub obp: String,
    pub caught_stealing: i64,
    pub stolen_bases: i64,
    pub stolen_base_percentage: String,
    pub era: String,
    pub innings_pitched: String,
    pub wins: i64,
    pub losses: i64,
    pub saves: i64,
    pub save_opportunities: i64,
    pub holds: i64,
    pub blown_saves: i64,
    pub earned_runs: i64,
    pub whip: String,
    pub outs: i64,
    pub games_pitched: i64,
    pub complete_games: i64,
    pub shutouts: i64,
    pub hit_batsmen: i64,
    pub balks: i64,
    pub wild_pitches: i64,
    pub pickoffs: i64,
    pub ground_outs_to_airouts: String,
    pub rbi: i64,
    pub win_percentage: String,
    pub games_finished: i64,
    pub strikeout_walk_ratio: String,
    pub strikeouts_per9_inn: String,
    pub walks_per9_inn: String,
    pub hits_per9_inn: String,
    pub runs_scored_per9: String,
    pub home_runs_per9: String,
    pub inherited_runners: i64,
    pub inherited_runners_scored: i64,
    pub catchers_interference: i64,
    pub sac_bunts: i64,
    pub sac_flies: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fielding {
    pub assists: i64,
    pub put_outs: i64,
    pub errors: i64,
    pub chances: i64,
    pub caught_stealing: i64,
    pub passed_ball: i64,
    pub stolen_bases: i64,
    pub stolen_base_percentage: String,
    pub pickoffs: i64,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoxscorePlayer {
    pub person: NamedRef,
    pub jersey_number: String,
    pub position: Position,
    pub stats: StatsBlock,
    pub status: CodeDesc,
    pub parent_team_id: i64,
    pub season_stats: StatsBlock,
    pub game_status: PlayerGameStatus,
}