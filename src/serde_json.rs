use std::str::FromStr;
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
    // println!("{:?}", &output);
}
