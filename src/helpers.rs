pub fn format_current_date(local_date: String) -> String {
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

pub fn get_utc_hour(utc_now: &str) -> String {
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

pub fn get_utc_minutes(utc_now: &str) -> String {
    let mut parts_vec = Vec::new();
    let parts = utc_now.split(":");

    for (position, part) in parts.enumerate() {
        if position == 1 {
            parts_vec.push(part);
        }
    }

    String::from(parts_vec[0])
}

pub fn suffix_toggle<'a>(len: usize) -> &'a str {
    return if len == 1 { "е" } else { "я" };
}
pub fn prepend_zero(s: String) -> String {
    format!("0{}", s)
}

pub fn should_show_index<'a>(len: usize, pos: usize) -> String {
    return if len == 1 {
        String::from("")
    } else {
        format!("{}{}", (pos + 1).to_string(), ".")
    };
}

pub fn in_time_range(
    utc_now: &str,
    fixed_hour: String,
    minutes_min: i32,
    minutes_max: i32,
) -> bool {
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
}
