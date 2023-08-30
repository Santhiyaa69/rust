use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub date: String,
    pub particulars: Option<String>,
    pub voucher_no: String,
    pub description: String,
    pub voucher_type: String,
    pub approved_by: String,
    pub amount: f64,
}

pub fn run() {
    
      let t1 = Transaction{
        date: "01-12-2022".to_string(),
        particulars: Some("cashfff".to_string()),
        voucher_no: "".to_string(),
        description: "".to_string(),
        voucher_type: "JOURNAL".to_string(),
        approved_by: "divya divi".to_string(),
        amount: 5.0,
    };
    let t2 = Transaction{
        date: "01-12-2022".to_string(),
        particulars: Some("cashfff".to_string()),
        voucher_no: "".to_string(),
        description: "".to_string(),
        voucher_type: "JOURNAL".to_string(),
        approved_by: "divya divi".to_string(),
        amount: 5.0,
    };
    let t3 = Transaction{
        date: "02-12-2022".to_string(),
        particulars: Some("test".to_string()),
        voucher_no: "".to_string(),
        description: "".to_string(),
        voucher_type: "SALE".to_string(),
        approved_by: "divya divi".to_string(),
        amount: 15.0,
    };
    let txns = vec![t1,t2,t3];
    
}