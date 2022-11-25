use serde_repr::{Deserialize_repr,Serialize_repr};

#[derive(Debug)]
enum Gender {
    Male,
    Female,
}
// enum Payment {
//     Cash,
//     CreditCard,
//     DebitCard,
// }
#[derive(Debug)]
enum Payment {
    Cash(i32),
    CreditCard(String, usize),
    DebitCard(DebitData),
    Crypto { account_num: i32, amount: String },
}

#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    gender: Gender,
}
#[derive(Debug)]
struct DebitData {
    card_number: i32,
    amount: usize,
}

pub fn run() {
    // let male = Gender::Male;
    // let female = Gender::Female;
    // println!("{:?}",male);
    // println!("{:?}",female);

    //struct and enum
    let _p1 = Person {
        name: "sai".to_string(),
        gender: Gender::Female,
    };
    let _p2 = Person {
        name: "sakthi".to_string(),
        gender: Gender::Male,
    };
    // println!("{:?}", &p1);
    // println!("{:?}", &p2);
    // let v = vec![p1,p2];
    // println!("{:?}", &v);

    //match and enum
    //     let payment_method = Payment::Cash;
    //    match payment_method {
    //        Payment::Cash => println!("paying with cash {}",500),
    //        _ => println!("other payment method")
    //    }

    // match statement and enum with data type
    let cash = Payment::Cash(500);
    pay_process(cash);

    let credit_card = Payment::CreditCard("thiya".to_string(), 8000);
    pay_process(credit_card);

    let debit_card = Payment::DebitCard(DebitData {
        card_number: 8976533,
        amount: 15000,
    });
    pay_process(debit_card);

    let cryp = Payment::Crypto {
        account_num: 78955578,
        amount: "TwentyFiveThousand".to_string(),
    };
    pay_process(cryp);
}

fn pay_process(payment_method: Payment) {
    match payment_method {
        Payment::Cash(amt) => println!("paying with cash {}", &amt),
        Payment::CreditCard(a, b) => println!("payment from {} amount {}", &a, &b),
        Payment::DebitCard(data) => println!(
            "paying with cardnum: {} amount: {}",
            data.card_number, data.amount
        ),
        Payment::Crypto {
            account_num,
            amount,
        } => println!("Account number:{} amount:{}", account_num, amount),
    }
}

#[derive(Debug, Clone)]
pub enum Precision {
    P0,
    P1,
    P2,
    P3,
    P4,
}

impl Precision {
    fn inner(&self) -> i32 {
        match self {
            Self::P0 => 0,
            Self::P1 => 1,
            Self::P2 => 2,
            Self::P3 => 3,
            Self::P4 => 4,
        }
    }
}

impl Default for Precision {
    fn default() -> Self {
        Self::P0
    }
}

//serialize enum as a number  //dependency  - serde_repr = "0.1.9"

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(u8)]
enum SmallPrime {
    Two = 2,
    Three = 3,
    Five = 5,
    Seven = 7,
}