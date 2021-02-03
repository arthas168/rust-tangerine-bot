use crate::event_list;
use crate::helpers;

pub fn new(events: Vec<event_list::Event>, local: String) -> String {
    let current_date = helpers::format_current_date(local.to_string());
    let message_prefix = r#"{"text":""#;
    let message_suffix = r#""}"#;

    print!("{:?}", events);

    let mut message = String::from("Добро утро! ☀ 💕");
    message = format!(
        "{}{} Днес ({}) има {} събити{}:\\n",
        message_prefix,
        message,
        current_date,
        events.len(),
        helpers::suffix_toggle(events.len())
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
            helpers::should_show_index(events.len(), position),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_for_array_with_multiple_elements() {
        let event_1: event_list::Event = event_list::Event {
            name: String::from("test"),
            dateWithoutTime: String::from("02.01.2020"),
            createdAt: String::from("02.01.2020"),
            referenceId: String::from("23432432"),
            time: String::from("14:30"),
            date: String::from("02.01.2020 14:30"),
        };

        let event_2: event_list::Event = event_list::Event {
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
        let message = new(mock_events, mock_local_time);
        let expected_message = r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) има 2 събития:\n     1.test в 14:30ч.\n     2.test2 в 14:30ч.\nХубав и успешен ден! ☕\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }

    #[test]
    fn test_message_for_array_with_one_element() {
        let event_1: event_list::Event = event_list::Event {
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
        let message = new(mock_events, mock_local_time);
        let expected_message = r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) има 1 събитие:\n     test just one в 14:30ч.\nХубав и успешен ден! ☕\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }

    #[test]
    fn test_message_for_empty_array() {
        let mock_events: Vec<event_list::Event> = Vec::new();

        let mock_local_time = String::from("2020-12-14 17:23:15.049409695 +02:00");
        let message = new(mock_events, mock_local_time);
        let expected_message =
            r#"{"text":"Добро утро! ☀ 💕 Днес (14.12.2020) няма събития. 🏖️\n️🍊 🦀"}"#;
        assert_eq!(message, expected_message);
    }
}
