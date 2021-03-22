extern crate chrono;
extern crate bigdecimal;

use serde::*;
use chrono::*;
use bigdecimal::BigDecimal;

pub fn time_to_json(t: NaiveDateTime) -> String {
	DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
}

mod json_time {
	use super::*;
	use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};

	pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
		time_to_json(time.clone()).serialize(serializer)
	}

	pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
		let time: String = Deserialize::deserialize(deserializer)?;
		Ok(DateTime::parse_from_rfc3339(&time).map_err(D::Error::custom)?.naive_utc())
	}
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
    #[serde(with = "json_time")]
    pub start_time: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Plan {
    pub id: i32,
    pub site_id: String,
    pub stage_id: String,
    pub oper_id: String,
    pub resource_id: String,
    pub product_id: String,
    pub plan_qty: BigDecimal,
    #[serde(with = "json_time")]
    pub start_time: NaiveDateTime,
    #[serde(with = "json_time")]
    pub end_time: NaiveDateTime,
}