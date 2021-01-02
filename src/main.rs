use chrono::prelude::*;
use exitfailure::ExitFailure;
use fb_messenger::send_message;
use std::env;

mod compose_message;
mod event_list;
mod helpers;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let response = event_list::EventList::get().await?;
    let message = compose_message::new(response.events, Local::now().to_string());

    let access_token = format!(r#"{}"#, env::var("ACCESS_TOKEN").unwrap());
    let messaging_type = format!(r#"{}"#, "UPDATE");
    let recipient_object = format!(r#"{}"#, env::var("RECIPIENT_OBJECT").unwrap());

    let utc_now = chrono::offset::Utc::now().to_string();

    if helpers::in_time_range(utc_now.as_str(), String::from("6"), 25, 35) {
        send_message(message, access_token, messaging_type, recipient_object).await;
    } else {
        panic!("Not in valid time range! 🦀");
    }

    Ok(())
}
