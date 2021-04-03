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
#[allow(non_snake_case)]
pub struct Plan {
    pub id: i32,
    #[column_name = "site_id"]
    pub siteId: String,
    #[column_name = "stage_id"]
    pub stageId: String,
    #[column_name = "oper_id"]
    pub operId: String,
    #[column_name = "resource_id"]
    pub resourceId: String,
    #[column_name = "product_id"]
    pub productId: String,
    #[column_name = "plan_qty"]
    pub planQty: BigDecimal,
    #[column_name = "start_time"]
    #[serde(with = "json_time")]
    pub startTime: NaiveDateTime,
    #[column_name = "end_time"]
    #[serde(with = "json_time")]
    pub endTime: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct User {
    pub email: String,
    pub password: String,
}
