use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContactInfo {
    mobile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternate_mobile: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AddressInfo {
    address: Option<String>,
    city: Option<String>,
    state: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customer {
    name: String,
    // #[serde(skip_serializing_if = "Option::is_none")]
    address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contact_info: Option<ContactInfo>,
    delivery_address: Option<Vec<AddressInfo>>,
}

pub fn get_customer(data: Customer) -> Result<Value> {
    // println!("{:?}", &data);
    let cust_name = data.name;
    let address = data.address.unwrap_or_default();
    let mut contact = None;
    if let Some(c) = data.contact_info {
        contact = c.mobile;
    }
    let out = serde_json::json!({
        "name": cust_name,
        "address": address,
        "contact_info": contact
    });
    println!("{:?}", &out);
    Ok(out)
}
