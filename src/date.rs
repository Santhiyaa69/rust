use mongodb::bson::DateTime;
// use chrono::prelude::*;

// let exp = Utc::now()
//             .checked_add_signed(Duration::seconds(86400))
//             .expect("")
//             .timestamp();

// pub fn bsondatetime_demo() {
//     let bson_dt = DateTime::now(); //current datetime
//     let chrono_dt = bson_dt.to_chrono().format("%Y-%m-%d").to_string(); //e.g `2022-05-05` - datetime to string
//     println!("{}", chrono_dt.as_str()); //string to &str
// }

// let date = DateTime::parse_rfc3339_str("2022-05-03T13:36:23.843Z").unwrap(); // datetime string to datetime

// pub fn datetime() {
//     let utc_dt = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
//     let local_dt = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`
//     let utc = Utc::today();
//     println!("utc-today: {:?}", utc);
//     println!("utc-dt: {:?}", utc_dt);
//     println!("local-dt: {:?}", local_dt);
//     println!("diff: {:?}", local_dt.signed_duration_since(utc_dt));
// }

pub fn cal_days_bt_dates() {
    // let start_date = NaiveDate::parse_from_str("2021-05-05", "%F").unwrap();
    // let end_date = NaiveDate::parse_from_str("2022-05-07", "%F").unwrap();
    // let d = end_date.signed_duration_since(start_date);
    // println!("{}", &d);
    // println!("days: {}", d.num_days());
    let date = DateTime::parse_rfc3339_str("2022-10-28T13:36:23.843Z").unwrap(); // datetime string to datetime
    println!("date: {}", date);
}

// pub fn date_str_split_by_chars() {
//     let date_str = "20230401";

//     let out = date_str
//         .chars()
//         .collect::<Vec<char>>()
//         .chunks(4)
//         .map(|c| c.iter().collect::<String>())
//         .collect::<Vec<String>>();

//     let y = &out[0];
//     let md = &out[1]
//         .chars()
//         .collect::<Vec<char>>()
//         .chunks(2)
//         .map(|c| c.iter().collect::<String>())
//         .collect::<Vec<String>>()
//         .join("-");
//     let date_str = format!("{}-{}", y, md);
//     println!("date_str : {:?}", &date_str);

//     // let naive_date = NaiveDate::parse_from_str(date_str.as_str(), "%Y-%m-%d").unwrap();
//     // println!("naive_date is: {}", naive_date);

//     let dd = "202341";
//     let (y, m, d) = (&dd[0..=3], &dd[4..=5], &dd[6..=7]);
//     println!("y: {y}, m: {m}, d: {d}");
// }
