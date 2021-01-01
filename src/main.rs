use chrono::prelude::*;
use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct Event {
    name: String,
    dateWithoutTime: String,
    createdAt: String,
    referenceId: String,
    time: String,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EventList {
    events: Vec<Event>,
}

impl EventList {
    async fn get() -> Result<Self, ExitFailure> {
        let url = env::var("EVENTS_URL").unwrap();
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url).await?.json::<EventList>().await?;
        Ok(resp)
    }
}

fn format_current_date(local_date: String) -> String {
    let mut parts_vec = Vec::new();

    let parts = local_date.split("-");
    for (position, part) in parts.enumerate() {
        if position == 0 || position == 1 {
            parts_vec.push(part);
        }
        if position == 2 {
            let split = part.split(" ");
            let vec = split.collect::<Vec<&str>>();

            parts_vec.push(vec[0]);
        }
    }

    format!("{}.{}.{}", parts_vec[2], parts_vec[1], parts_vec[0])
}

fn get_utc_hour(utc_now: &str) -> String {
    let mut parts_vec = Vec::new();

    let parts = utc_now.split(":");
    for (position, part) in parts.enumerate() {
        if position == 0 {
            parts_vec.push(part);
        }
    }
    let mut time_parts_vec = Vec::new();

    let parts = parts_vec[0].split(" ");
    for (position, part) in parts.enumerate() {
        if position == 1 {
            time_parts_vec.push(part);
        }
    }

    time_parts_vec[0].to_string()
}

fn get_utc_minutes(utc_now: &str) -> String {
    let mut parts_vec = Vec::new();
    let parts = utc_now.split(":");

    for (position, part) in parts.enumerate() {
        if position == 1 {
            parts_vec.push(part);
        }
    }

    String::from(parts_vec[0])
}

fn suffix_toggle<'a>(len: usize) -> &'a str {
    return if len == 1 { "е" } else { "я" };
}

fn prepend_zero(s: String) -> String {
    format!("0{}", s)
}

fn should_show_index<'a>(len: usize, pos: usize) -> String {
    return if len == 1 {
        String::from("")
    } else {
        format!("{}{}", (pos + 1).to_string(), ".")
    };
}

fn in_time_range(utc_now: &str, fixed_hour: String, minutes_min: i32, minutes_max: i32) -> bool {
    let hour = get_utc_hour(&utc_now);
    let minutes = get_utc_minutes(&utc_now);

    let minutes_int: i32 = minutes.trim().parse().unwrap();
    if minutes_int > minutes_max || minutes_int < minutes_min {
        return false;
    }

    if hour == fixed_hour || hour == prepend_zero(fixed_hour) {
        true
    } else {
        false
    }
}

fn compose_message(events: Vec<Event>, local: String) -> String {
    let current_date = format_current_date(local.to_string());
    let message_prefix = r#"{"text":""#;
    let message_suffix = r#""}"#;

    let mut message = String::from("Добро утро! ☀ 💕");
    message = format!(
        "{}{} Днес ({}) има {} събити{}:\\n",
        message_prefix,
        message,
        current_date,
        events.len(),
        suffix_toggle(events.len())
    );

    for (position, event) in events.iter().enumerate() {
        let name = event.name.as_str();
        let time = event.time.as_str();
        let at = " в ";
        let hr_suffix = "ч.";
        let tab = "     ";
        message = format!(
            "{}{}{}{}{}{}{}\\n",
            message,
            tab,
            should_show_index(events.len(), position),
            name,
            at,
            time,
            hr_suffix
        );
    }

    message = format!(
        "{}{}\\n{}{}",
        message.as_str(),
        "Хубав и успешен ден! ☕",
        "️🍊 🦀",
        message_suffix
    );

    if events.len() == 0 {
        message = format!(
            "{}Добро утро! ☀ 💕 Днес ({}) няма събития. 🏖️\\n️🍊 🦀{}",
            message_prefix, current_date, message_suffix,
        );
    }

    message
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let response = EventList::get().await?;
    let message = compose_message(response.events, Local::now().to_string());
    let access_token = format!(r#"{}"#, env::var("ACCESS_TOKEN").unwrap());
    let messaging_type = format!(r#"{}"#, "UPDATE");
    let recipient_object = format!(r#"{}"#, env::var("RECIPIENT_OBJECT").unwrap());

    let client = reqwest::Client::new();
    let url = "https://graph.facebook.com/v9.0/me/messages";
    let params = [
        ("access_token", &access_token),
        ("messaging_type", &messaging_type),
        ("recipient", &recipient_object),
        ("message", &message),
    ];


    let utc_now = chrono::offset::Utc::now().to_string();
    if in_time_range(utc_now.as_str(), String::from("6"), 25, 35) {
        client.post(url).form(&params).send().await;
    } else {
        panic!("Not in valid time range!");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_current_date() {
        let mock_local_date = String::from("2020-12-09 15:59:34.294613366 +02:00");
        let formatted = format_current_date(mock_local_date);

        assert_eq!(formatted, "09.12.2020");
    }

    #[test]
    fn test_get_utc_hour() {
        let mock_utc_now = "2021-01-01 14:03:37.577180674 UTC";
        let hour = get_utc_hour(mock_utc_now);

        assert_eq!(hour, "14");
    }

    #[test]
    fn test_get_utc_minutes() {
        let mock_utc_now = "2021-01-01 14:03:37.577180674 UTC";
        let minutes = get_utc_minutes(mock_utc_now);

        assert_eq!(minutes, "03");
    }

    #[test]
    fn test_suffix_toggle() {
        let mock_len = 0;
        assert_ne!(suffix_toggle(mock_len), "е");

        let mock_len = 1;
        assert_eq!(suffix_toggle(mock_len), "е");

        let mock_len = 100;
        assert_eq!(suffix_toggle(mock_len), "я");
    }

    #[test]
    fn test_should_show_index() {
        let mock_len = 4;
        assert_eq!(should_show_index(mock_len, 3), "4.");

        let mock_len = 1;
        assert_eq!(should_show_index(mock_len, 0), "");

        let mock_len = 4;
        assert_ne!(should_show_index(mock_len, 3), "3.");
    }

    #[test]
    fn test_prepend_zero() {
        let mock_s = String::from("6");
        assert_eq!(prepend_zero(mock_s), "06");

        let mock_s = String::from("07");
        assert_eq!(prepend_zero(mock_s), "007");
    }

    #[test]
    fn test_in_time_range() {
        let mock_utc_now = String::from("2021-01-01 14:03:37.577180674 UTC");
        let minutes = get_utc_minutes(mock_utc_now.as_str());

        let minutes_int: i32 = minutes.trim().parse().unwrap();
        assert_eq!(minutes_int, 3);

        let is_valid_time = in_time_range(&mock_utc_now, String::from("14"), 1, 5);
        assert_eq!(is_valid_time, true);

        let is_valid_time = in_time_range(&mock_utc_now, String::from("14"), 5, 10);
        assert_ne!(is_valid_time, true);

        let is_valid_time = in_time_range(&mock_utc_now, String::from("00"), 1, 5);
        assert_ne!(is_valid_time, true);

        let is_valid_time = in_time_range(&mock_utc_now, String::from("14"), 1, 2);
        assert_ne!(is_valid_time, true);
    }

    #[test]
    fn test_message_for_array_with_multiple_elements() {
        let event_1: Event = Event {
            name: String::from("test"),
            dateWithoutTime: String::from("02.01.2020"),
            createdAt: String::from("02.01.2020"),
            referenceId: String::from("23432432"),
            time: String::from("14:30"),
            date: String::from("02.01.2020 14:30"),
        };

        let event_2: Event = Event {
            name: String::from("test2"),
            dateWithoutTime: String::from("02.01.2020"),
            createdAt: String::from("02.01.2020"),
            referenceId: String::from("23432432"),
            time: String::from("14:30"),
            date: String::from("02.01.2020 14:30"),
        };

        let mut mock_events = Vec::new();
        mock_events.push(event_1);
        mock_events.push(event_2);

        let mock_local_time = String::from("2020-12-14 17:23:15.049409695 +02:00");
        let message = compose_message(mock_events, mock_local_time);
        let expected_message = r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) има 2 събития:\n     1.test в 14:30ч.\n     2.test2 в 14:30ч.\nХубав и успешен ден! ☕\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }

    #[test]
    fn test_message_for_array_with_one_element() {
        let event_1: Event = Event {
            name: String::from("test just one"),
            dateWithoutTime: String::from("02.01.2020"),
            createdAt: String::from("02.01.2020"),
            referenceId: String::from("23432432"),
            time: String::from("14:30"),
            date: String::from("02.01.2020 14:30"),
        };

        let mut mock_events = Vec::new();
        mock_events.push(event_1);

        let mock_local_time = String::from("2020-12-14 17:23:15.049409695 +02:00");
        let message = compose_message(mock_events, mock_local_time);
        let expected_message = r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) има 1 събитие:\n     test just one в 14:30ч.\nХубав и успешен ден! ☕\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }

    #[test]
    fn test_message_for_empty_array() {
        let mut mock_events: Vec<Event> = Vec::new();

        let mock_local_time = String::from("2020-12-14 17:23:15.049409695 +02:00");
        let message = compose_message(mock_events, mock_local_time);
        let expected_message =
            r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) няма събития. 🏖️\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }
}
