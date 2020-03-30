extern crate serde;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub event_id: i64,
    pub event_source: String,
    pub alt_ids: AltIds,
    pub scoreboard_available: bool,
    pub sport: String,
    pub latest_score: Option<LatestScore>,
    pub clock: Clock,
    pub sport_details: Option<SportDetails>,
    pub game_status: String,
    pub last_updated: String,
    pub display_visitor_team_first: bool,
    #[serde(default)]
    pub previous_periods_score: Vec<PreviousPeriodsScore>,
    pub current_period_score: Option<CurrentPeriodScore>,
    pub last_significant_key_event: Option<LastSignificantKeyEvent>,
    pub last_significant_key_event_index: Option<i64>,
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(cur_score) = &self.latest_score {
            write!(
                f,
                "{}, {}, {}, {}, {} {} {} {}",
                self.event_id,
                self.sport,
                self.game_status,
                self.last_updated,
                self.clock.period_number,
                self.clock.relative_game_time_in_secs,
                self.clock.is_ticking,
                cur_score
            )
        } else {
            write!(
                f,
                "{}, {}, {}, {}, {} {} {}",
                self.event_id,
                self.sport,
                self.game_status,
                self.last_updated,
                self.clock.period_number,
                self.clock.relative_game_time_in_secs,
                self.clock.is_ticking
            )
        }
    }
}

impl Root {
    fn gen_rec(&self) -> Vec<String>{
        if let Some(ls) = &self.latest_score {
            let rec = &[
                self.event_id.to_string(),
                self.sport.to_string(),
                self.game_status.to_string(),
                self.last_updated.to_string(),
                self.clock.period_number.to_string(),
                self.clock.relative_game_time_in_secs.to_string(),
                self.clock.is_ticking.to_string(),
                ls.visitor.to_string(),
                ls.home.to_string(),
            ];
            return rec;
        } else {
            let rec = &[
                self.event_id.to_string(),
                self.sport.to_string(),
                self.game_status.to_string(),
                self.last_updated.to_string(),
                self.clock.period_number.to_string(),
                self.clock.relative_game_time_in_secs.to_string(),
                self.clock.is_ticking.to_string(),
                "".to_string(),
                "".to_string(),
            ];
            return rec;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AltIds {
    #[serde(rename = "BGS")]
    pub bgs: i64,
    pub feed_code: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LatestScore {
    pub home: String,
    pub visitor: String,
}

impl fmt::Display for LatestScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.visitor, self.home)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Clock {
    pub period: String,
    pub period_number: i64,
    pub game_time: String,
    pub relative_game_time_in_secs: i64,
    pub direction: String,
    pub number_of_periods: i64,
    pub is_ticking: bool,
    #[serde(default)]
    pub children: Vec<Children>,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}, {}",
            self.period_number, self.relative_game_time_in_secs, self.is_ticking
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Children {
    pub period: String,
    pub period_number: i64,
    pub game_time: String,
    pub relative_game_time_in_secs: i64,
    pub direction: String,
    pub number_of_periods: i64,
    pub is_ticking: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SportDetails {
    pub numbers: Option<Numbers>,
    pub soccer: Option<Soccer>,
    pub tennis: Option<Tennis>,
    pub football: Option<Football>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Numbers {
    pub draws: Vec<Draw>,
    pub previous_score: Option<PreviousScore>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Draw {
    pub number: i64,
    pub round: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviousScore {
    pub total: i64,
    pub draws: Vec<Draw2>,
    pub parity: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Draw2 {
    pub number: i64,
    pub round: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Soccer {
    pub yellow_cards: Option<YellowCards>,
    pub red_cards: Option<RedCards>,
    pub corners: Option<Corners>,
    pub has_overtime: bool,
    pub penalties: Option<Penalties>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct YellowCards {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RedCards {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Corners {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalties {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tennis {
    pub sets: Sets,
    pub server: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sets {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Football {
    pub possession: String,
    pub show_secondary_info: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviousPeriodsScore {
    pub home: i64,
    pub visitor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CurrentPeriodScore {
    pub home: i64,
    pub visitor: i64,
}

impl fmt::Display for CurrentPeriodScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}, {:?}", self.visitor, self.home)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastSignificantKeyEvent {
    pub event_type: String,
    pub params: Params,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Params {
    pub home_score: Option<String>,
    pub visitor_score: Option<String>,
    pub competitor_name: Option<String>,
    pub game_time: Option<String>,
    pub goal_ordinal: Option<String>,
    pub period: Option<String>,
    pub game: Option<String>,
    pub loser_game_score: Option<String>,
    pub period_ordinal: Option<String>,
    pub frame: Option<String>,
    pub number_of_runs: Option<String>,
    pub competitor_abbreviation: Option<String>,
    pub card_ordinal: Option<String>,
    pub loser_set_game_score: Option<String>,
    pub set: Option<String>,
    pub winner_set_game_score: Option<String>,
}
