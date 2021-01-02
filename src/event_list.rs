use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub name: String,
    pub dateWithoutTime: String,
    pub createdAt: String,
    pub referenceId: String,
    pub time: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventList {
    pub events: Vec<Event>,
}

impl EventList {
    pub async fn get() -> Result<Self, ExitFailure> {
        let url = env::var("EVENTS_URL").unwrap();
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<EventList>().await?;
        Ok(resp)
    }
}
