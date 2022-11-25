use chrono::NaiveDate;
use mongodb::bson::DateTime;
use serde::{Serialize,Deserialize};
// Serialize and deserialize this struct or enum with the given name instead of its Rust name.
// #[serde(rename(serialize = "ser_name"))]
// #[serde(rename(deserialize = "de_name"))]
// #[serde(rename(serialize = "ser_name", deserialize = "de_name"))]

// #[serde(rename_all = "...")] //Rename all fields according to given case convention
// #[serde(default)] //When deserializing, any missing fields should be filled in from the struct's implementation of Default. Only allowed on structs.
// #[serde(skip)] //Never serialize or deserialize this variant.
// #[serde(skip_serializing)] //Never serialize this variant.
// #[serde(skip_serializing)] //Never deserialize this variant.

pub fn run() {
    let mut output = Vec::new();

    let mut branches = vec!["DEMO", "DUMMY"];
    branches.push("DP ROAD BRANCH");
    let br = serde_json::to_string(&branches).unwrap();
    println!("{:?}", &br);
    let enc = serde_urlencoded::to_string([("branches", br)]).unwrap();
    println!("{:?}", &enc);

    let out = serde_json::json!({
        "title": "Accounts",
        "organization": "test".to_string(),
        "branch": branches,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        },
    });
    output.push(out);
    println!("{:?}", &output);
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn default_as_true() -> bool {
    true
}

#[derive(Debug, Clone)]
pub struct CurrentDate(pub DateTime);

impl Default for CurrentDate {
    fn default() -> Self {
        Self(DateTime::now())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "CurrentDate")]
struct CurrentDateRef(DateTime);


#[derive(Debug,Serialize,Deserialize)]
struct User {
    id: String,
    username: Vec<String>,
    #[serde(default = "default_as_true")]
    is_active: bool,
    #[serde(with = "CurrentDateRef")]
    created_at: CurrentDate
}

pub fn example() {
    let point = Point{
        x:  1,
        y: 2,
    };
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {:?}", &serialized);
    let deserialized:Point= serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", &deserialized);

    let user = User{
        id: "1".to_string(),
        username: vec!["Thiya".to_string(),"riya".to_string()],
        is_active: true,
        created_at: CurrentDate::default(),
    };
    println!("user = {:?}", &user);
}