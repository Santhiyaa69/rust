use mongodb::bson::oid::ObjectId;

#[derive(Debug)]
pub struct VoucherAccountTransaction {
    pub pending: Option<ObjectId>,
    pub account_type: String,
    pub credit: f64,
    pub debit: f64,
}

#[derive(Debug)]
struct GoodBoy {
    name: String,
    weight: i32,
}

pub fn max_by () {
    let ac_trns1 = VoucherAccountTransaction {
        account_type: "BANK_ACCOUNT".to_owned(),
        credit: 0.0,
        debit: 100.0,
        pending: None,
    };
    let ac_trns2 = VoucherAccountTransaction {
        account_type: "DIRECT_INCOME".to_owned(),
        credit: 100.0,
        debit: 0.0,
        pending: None,
    };
    let mut ac_trns = vec![ac_trns1,ac_trns2];
    ac_trns.sort_by(|x,y| y.debit.total_cmp(&x.debit));
    let txn = ac_trns.first();
    println!("out1 = {:?}",&txn);


    let mut bois = vec![
    GoodBoy { name: "Pucci".to_owned(), weight: 1 },
    GoodBoy { name: "Woofer".to_owned(), weight: 99 },
    GoodBoy { name: "Yapper".to_owned(), weight: 10 },
    GoodBoy { name: "Floaty".to_owned(), weight: -5 },
];
bois.sort_by(|a, b| b.weight.cmp(&a.weight));
println!("out2 = {:?}",&bois.first());

let mut credit = [200,100];
credit.sort_by(|a, b| b.cmp(a));
println!("{:?}", &credit.first());

let data: &mut [_] = &mut [2, 10, 5, 8];

// sort the array from largest to smallest.
data.sort_by(|a, b| a.cmp(b).reverse());
println!("{:?}", &data.first());
}
