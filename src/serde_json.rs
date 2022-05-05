pub fn run() {
    let mut output = Vec::new();

    let mut branches = vec!["DEMO", "DUMMY"];
    branches.push("DP ROAD BRANCH");

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
