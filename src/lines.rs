extern crate csv;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Record {
    oc_id: String,
    id: String,
    sport: String,
    event_desc: String,
    dg_desc: String,
    mkt_desc: String,
    oc_desc: String,
    price: String,
    hc: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: Vec<Path>,
    pub events: Vec<Event>,
}

impl Root {
    pub fn to_records(&self) -> Vec<csv::StringRecord> {
        let mut recs: Vec<csv::StringRecord> = Vec::new();
        for e in self.events.iter() {
            for dg in e.display_groups.iter() {
                for m in dg.markets.iter() {
                    for oc in m.outcomes.iter() {
                        let hc = match &oc.price.handicap {
                            Some(hc) => hc,
                            None => "",
                        };

                        let rec = &[
                            &oc.id,
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
                        recs.push(csv::StringRecord::from(rec.to_vec()));
                    }
                }
            }
        }
        return recs;
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.events)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub id: String,
    pub link: Option<String>,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub sport_code: Option<String>,
    pub order: i128,
    pub leaf: bool,
    pub current: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub link: Option<String>,
    pub status: String,
    pub sport: String,
    pub start_time: Option<i128>,
    pub live: bool,
    pub away_team_first: Option<bool>,
    pub deny_same_game: Option<String>,
    pub teaser_allowed: Option<bool>,
    pub competition_id: Option<String>,
    pub notes: String,
    pub num_markets: Option<u64>,
    pub last_modified: Option<i128>,
    pub competitors: Vec<Competitor>,
    // pub display_groups: Option<Vec<DisplayGroup>>,
    pub display_groups: Vec<DisplayGroup>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.display_groups)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Competitor {
    pub id: String,
    pub name: String,
    pub home: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayGroup {
    pub id: String,
    pub description: String,
    pub default_type: Option<bool>,
    pub alternate_type: Option<bool>,
    pub markets: Vec<Market>,
    pub order: i128,
}

impl fmt::Display for DisplayGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DG_START{} {} {} {:#?}",
            self.id,
            self.description,
            self.order,
            self.markets.iter()
        )
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    pub id: String,
    pub description_key: Option<String>,
    pub description: String,
    pub key: String,
    pub market_type_id: Option<String>,
    pub status: String,
    pub single_only: Option<bool>,
    pub notes: String,
    pub period: Period,
    pub outcomes: Vec<Outcome>,
    pub sort_type: Option<String>,
}

impl fmt::Display for Market {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.outcomes)
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub id: String,
    pub description: String,
    pub abbreviation: String,
    pub live: bool,
    pub main: bool,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Outcome {
    pub id: String,
    pub description: String,
    pub status: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub price: Price,
    pub competitor_id: Option<String>,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.price.to_string())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Price {
    pub id: String,
    pub american: String,
    pub decimal: String,
    pub fractional: String,
    pub malay: String,
    pub indonesian: String,
    pub hongkong: String,
    pub handicap: Option<String>,
    pub handicap2: Option<String>,
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decimal.to_string())
    }
}
