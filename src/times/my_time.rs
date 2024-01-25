use axum::Json;
use chrono::{DateTime, Local, NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use serde_json::{json, Value};

pub fn get_local_time() {
    let current_local: DateTime<Local> = Local::now();
    println!("current_local: {}", current_local);

    let time_format = get_time_format("%Y-%m-%d %H:%M:%S");
    println!("time format: {}", time_format);

    let t_to_t = timestamp_to_time("1524820690", "%Y-%m-%d %H:%M:%S");
    println!("timestamp to time: {}", t_to_t);

    let time_str_to_dt = date_str_to_datetime("2020-04-12 22:10:57", "%Y-%m-%d %H:%M:%S");
    println!("date str to time: {}", time_str_to_dt);

    let day = 365;
    let day_30_added = add_day_from_now(day);
    println!("{} day added: {}", day, day_30_added);

    let add_tz = add_tz(
        Local::now()
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str(),
        "Asia/Bangkok",
        "%Y-%m-%d %H:%M:%S",
    );
    println!("date add timezone: {}", add_tz);
}

fn get_time_format(format: &str) -> String {
    let local_time: DateTime<Local> = Local::now();
    let with_format = local_time.format(format);
    return format!("{}", with_format);
}

fn timestamp_to_time(dt: &str, format: &str) -> String {
    let timestamp = dt.parse::<i64>().unwrap();
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    let new_date = datetime.format(format);
    return format!("{}", new_date);
}

fn date_str_to_datetime(date_str: &str, format: &str) -> String {
    let naive_datetime = NaiveDateTime::parse_from_str(date_str, format).unwrap();
    return format!("{}", naive_datetime);
}

fn add_day_from_now(num_days: i64) -> String {
    let today = Utc::now();
    let thirty_days_later = today + chrono::Duration::days(num_days);

    return format!("{}", thirty_days_later.format("%Y-%m-%d"));
}

fn add_tz(dt: &str, tz: &str, format: &str) -> String {
    let local = NaiveDateTime::parse_from_str(dt, format).unwrap();
    let tz: Tz = tz.parse().unwrap();
    let utc_datetime = local.and_local_timezone(tz).unwrap();

    return format!("{}", utc_datetime.format(format));
}
