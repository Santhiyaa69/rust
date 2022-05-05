use bson::DateTime;
use chrono::prelude::*;

pub fn bsondatetime_demo() {
    let bson_dt = DateTime::now(); //current datetime
    let chrono_dt = bson_dt.to_chrono().format("%Y-%m-%d").to_string(); //e.g `2022-05-05` - datetime to string
    println!("{}", chrono_dt.as_str()); //string to &str
}

// let date = DateTime::parse_rfc3339_str("2022-05-03T13:36:23.843Z").unwrap();

pub fn datetime() {
    let utc_dt = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local_dt = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
    let utc = Utc::today();
    println!("utc-today: {:?}", utc);
    println!("utc-dt: {:?}", utc_dt);
    println!("local-dt: {:?}", local_dt);
    println!("diff: {:?}", local_dt.signed_duration_since(utc_dt));
}

pub fn cal_days_bt_dates() {
    let start_date = NaiveDate::parse_from_str("2021-05-05", "%F").unwrap();
    let end_date = NaiveDate::parse_from_str("2022-05-07", "%F").unwrap();
    let d = end_date.signed_duration_since(start_date);
    println!("{}", &d);
    println!("days: {}", d.num_days());
}
