extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::collections::HashMap;
use crate::utils;
use crate::MLB;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub copyright: String,
    pub total_items: i64,
    pub total_events: i64,
    pub total_games: i64,
    pub total_games_in_progress: i64,
    pub dates: Vec<Date>
}

impl Root {
    pub fn to_records(&self) -> Vec<Vec<String>> {
        let mut recs: Vec<Vec<String>> = Vec::new();
        for d in self.dates.iter() {
            for g in d.games.iter() {
                recs.push(Game::to_record(g));
            }
        }
    return recs;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub date: String,
    pub total_items: i64,
    pub total_events: i64,
    pub total_games: i64,
    pub total_games_in_progress: i64,
    pub games: Vec<Game>,
    pub events: Vec<::serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_pk: i64,
    pub link: String,
    pub game_type: String,
    pub season: String,
    pub game_date: String,
    pub status: Status,
    pub teams: Teams,
    pub venue: General,
    pub content: Content,
    pub is_tie: Option<bool>,
    pub game_number: i64,
    pub public_facing: bool,
    pub double_header: String,
    pub gameday_type: Option<String>,
    pub tiebreaker: String,
    #[serde(rename = "calendarEventID")]
    pub calendar_event_id: String,
    pub season_display: String,
    pub day_night: String,
    pub description: Option<String>,
    pub scheduled_innings: Option<i64>,
    pub inning_break_length: i64,
    pub games_in_series: Option<i64>,
    pub series_game_number: Option<i64>,
    pub series_description: Option<String>,
    pub record_source: String,
    pub if_necessary: String,
    pub if_necessary_description: String,
    pub reschedule_date: Option<String>,
    pub rescheduled_from: Option<String>,
    pub resume_date: Option<String>,
    pub resumed_from: Option<String>,
}

impl Game {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = vec!(
            self.season.clone(),
            self.game_pk.to_string(), 
            self.game_date.clone(),
            self.game_type.clone(),
        );
        rec.append(&mut Teams::to_record(&self.teams));
        rec.append(&mut Status::to_record(&self.status));
        rec.append(&mut General::to_record(&self.venue));
        rec.append(&mut vec!(
            utils::lilmatcher_bool(self.is_tie),
            self.game_number.to_string(),
            self.public_facing.to_string(),
            self.double_header.clone(),
            utils::lilmatcher(self.gameday_type.clone()),
            self.tiebreaker.clone(),
            self.calendar_event_id.clone(),
            self.day_night.clone(),
            utils::lilmatcher(self.description.clone()).replace(",", ";"),
            utils::lilmatcher_i64(self.scheduled_innings),
            self.inning_break_length.to_string(),
            utils::lilmatcher_i64(self.games_in_series),
            utils::lilmatcher_i64(self.series_game_number),
            utils::lilmatcher(self.series_description.clone()),
            self.record_source.clone(),
            self.if_necessary.clone(),
            self.if_necessary_description.clone(),
        ));

        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub abstract_game_state: String,
    pub coded_game_state: String,
    pub detailed_state: String,
    pub status_code: String,
    pub abstract_game_code: String,
    #[serde(rename = "startTimeTBD")]
    pub start_time_tbd: Option<bool>,
    pub reason: Option<String>,
}

impl Status {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.coded_game_state.clone(),
            self.detailed_state.clone(),
            self.status_code.clone(),
            utils::lilmatcher_bool(self.start_time_tbd),
            utils::lilmatcher(self.reason.clone()),
        );
        return rec
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub away: ScheduleTeam,
    pub home: ScheduleTeam,
}

impl Teams {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = Vec::new();
        rec.append(&mut ScheduleTeam::to_record(&self.away));
        rec.append(&mut ScheduleTeam::to_record(&self.home));
        return rec;
    }
}


#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduleTeam {
    pub league_record: MLB::LeagueRecord,
    pub score: Option<i64>,
    pub team: General,
    pub is_winner: Option<bool>,
    pub split_squad: bool,
    pub series_number: Option<i64>,
}

impl ScheduleTeam {
    pub fn to_record(&self) -> Vec<String> {
        let mut rec: Vec<String> = Vec::new();
        rec.append(&mut General::to_record(&self.team));
        rec.append(&mut MLB::LeagueRecord::to_record(&self.league_record));
        rec.append(
            &mut vec!(
                utils::lilmatcher_i64(self.score),
                utils::lilmatcher_bool(self.is_winner),
                self.split_squad.to_string(),
                utils::lilmatcher_i64(self.series_number)
            ));
        return rec;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub link: String,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct General {
    pub id: i64,
    pub name: String,
    pub link: String,
}

impl General {
    pub fn to_record(&self) -> Vec<String> {
        let rec: Vec<String> = vec!(
            self.id.to_string(),
            self.name.to_string(),
        );
        // for Team ref link pre is /api/v1/teams/,
        // for Venue ref link pre is /api/v1/venues/,
        return rec
    }
}