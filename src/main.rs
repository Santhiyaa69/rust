// use password_encrypt::{hash_password, verify_password};
// use serde_json::from_str;

use chrono::Utc;

// mod print;
// mod variables;
//  mod data_types;
// mod strings;
// mod tuples;
// mod array;
mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_reference;
// mod structs;
// mod results;
// mod enums;
// mod match_operator;
// mod options;
// mod iter;
// pub mod results;
mod date;
// mod password_encrypt;
// mod serde_json;
// mod string_matches;
// mod thread;
// mod structs_ex;
// mod traits;
// mod hashmap;
// mod hash;
// mod student_info;
// use student_info::*;
// mod rocket_rs;
// use rocket_rs::demo;
mod file;

#[allow(dead_code)]

// struct Student {
//       id: usize,
//       class: usize,
//       mark_details: Details
// }
fn main() {
    //       let stud1 =  Details {
    //             name:"Riya".to_string(),
    //             m1: 90,
    //             m2: 85,
    //             m3: 76
    //       };
    //       let stud2 = Details {
    //             name: "Thiya".to_string(),
    //             m1: 95,
    //             m2: 79,
    //             m3: 65
    //       };
    //       let s1 = Student {
    //             id: 1,
    //             class: 1,
    //             mark_details: Details::new("Riya".to_string(),87),
    //       };
    //       let s2 = Student {
    //             id: 2,
    //             class: 2,
    //             mark_details: Details::new("Thiya".to_string(),75),
    //       };
    // println!("{:#?}",s1.mark_details);
    // println!("{:#?}",s2.mark_details);
    // println!("Total Marks:{}",stud1.total());
    // println!("Total Marks:{}",stud2.total());
    // println!("Highest Mark in m2:{:?}",stud1.compare(stud2));

    let _data = r#"{
        "name": "Thiya",
        "address": "5, KTC Nagar,Tuty",
        "contactInfo": {
            "mobile": "6712034761",
            "alternateMobile": "9812345098"
        },
        "deliveryAddress": [
            {
                "address": "10, TTC Nagar",
                "city": "Tuty"
            }
        ]
    }"#;
    // let dt = serde_json::from_str(&data).unwrap();

    // print::print();
    // variables::vars();
    // data_types::types();
    // strings::strings();
    // tuples::tuples();
    // array::arrays();
    vectors::vectors();
    // conditionals::conditions();
    // loops::loops();
    // functions::run();
    // structs::run();
    // results::run();
    // enums::run();
    // match_operator::run();
    // options::run();
    // iter::run();
    // hashmap::run();
    // hash::run();
    date::bsondatetime_demo();
    // date::datetime();
    // date::cal_days_bt_dates();
    // serde_json::run();
    // string_matches::run();
    // thread::run();
    // structs_ex::get_customer(dt).unwrap();
    // hash_password("aplus");
    // verify_password(
    //     "$scrypt$ln=15,r=8,p=1$d5CP0THDyqmVQBMM8NcdIA$RNGmmsNnTTXJiMgJAKYdzvBb2rxJ+2rLQn2DdNT5BCo",
    //     "12345",
    // );
    file::run();
    file::open();

    // #[rocket::main]
    // async fn main() -> Result<(), rocket::Error> {
    //     let _rocket = rocket::build()
    //         .mount("/hello", demo::routes())
    //         .launch()
    //         .await?;

    //     Ok(())
    // }

    let a = Utc::now();
    println!("output = {:?}", &a);
}
