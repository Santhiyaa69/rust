use std::collections::HashSet;

use mongodb::bson::oid::ObjectId;

// use itertools::Itertools;

#[derive(Debug)]
#[allow(dead_code)]

struct Student {
    id: usize,
    name: String,
    marks: usize,
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

#[derive(Debug)]
#[allow(dead_code)]

struct Amount {
    debit: f64, 
    credit: f64,
}

#[derive(Debug)]
#[allow(dead_code)]

struct ChequeBook {
    id: String, 
    name: String,
}

pub fn vectors() {
    // let v:Vec<i32> = Vec::new();
    // println!("vectors length:{:?}",v.len());

    // let vect = vec![3;5];
    // println!("{:?}",vect); //[3,3,3,3,3]

    // let mut num1 = vec![1,2,3,5];
    // let num2 = num1[2]+5;
    // println!("vectors:{:?}",num2); //8

    // let mut v1 = vec![10,20,30,40,50];
    // let v2 = v1;
    // v1.push(60);
    // println!("{:?}",v1) //error

    // let mut v1 = vec![10,20,30,40,50];
    // let v2 = &v1;
    // v1.push(60);
    // println!("{:?}",v1); //[10,20,30,40,50,60]
    // println!("{:?}",v2) //error-can't borrow v1 as mutable

    // let mut v :Vec<usize> = Vec::new();
    // for val in 0..11 {
    //     v.push(val);
    // }
    // println!("{:?}",v);

    // let a1 = String::from("tree");
    // let a2 = String::from("hello");
    // let a3 = String::from("hai");
    // let mut vect = vec![a1,a2,a3];
    // let mut res = vec![];

    // for v in vect {
    //    println!("{}",v);
    //  if v == &"hello" {
    //   res.push("haii...");
    //  }
    // }
    // //  println!("{:?}",vect); //error
    //  println!("{:?}",res);

    // for v in &mut vect {
    //     if v == "hello" {
    //         v.push_str("world");
    //         res.push(v);
    //     }
    // }
    //  println!("{:?}",vect);

    // let v1 = String::from("tree");
    // let v2 = String::from("hai");
    // let r = vec![v1,v2];
    //     for v in r.iter() {
    //         if vect.contains(v) {
    //             res.push(v);
    //         }
    //     }
    //     println!("{:?}",res);

    let mut products: Vec<&str> = vec!["pen", "pencil"];
    // println!("vectors:{:?}",products);

    // products[0] = "Pen";
    // println!("vectors:{:?}",products);
    // println!("vectors length:{:?}",products.len());

    // products.push("marker");
    // products.push("sketch");
    // println!("Add:{:?}",products);

    // products.pop();
    // println!("remove:{:?}",products);

    for x in products.iter() {
        // println!("{}",x);
    }

    // for no in num.iter_mut(){
    //     *no +=2;
    // }
    // println!("{:?}",num);

    let s1 = Student {
        id: 1,
        name: "sandy".to_string(),
        marks: 400,
    };
    let s2 = Student {
        id: 2,
        name: "devi".to_string(),
        marks: 450,
    };
    let s3 = Student {
        id: 3,
        name: "divya".to_string(),
        marks: 480,
    };
    let vect = vec![s1, s2, s3];
    // println!("{:#?}",vect);
    for stud in vect.iter() {
        let s = stud.marks;
        if s > 450 {
            //    println!("{:?}",stud)
        }
    }

    //remove duplicate elements
    let mut v = vec![
        Some("a".to_string()),
        Some("b".to_string()),
        Some("a".to_string()),
        Some("c".to_string()),
        Some("b".to_string()),
    ];
    v.sort_unstable(); //[a,a,b,b,c]

    v.dedup(); //only removes consecutive elements from a vector
    // println!("dup - {:?}", v);

    let arr1 = Some([
        "aaa".to_string(),
        "bbb".to_string(),
        "ccc".to_string(),
        "ccc".to_string(),
        "bbb".to_string(),
        "".to_string(),
        "DDD".to_string(),
    ]);

    // println!()
    // Way 1: //dependency itertools
    // let mut arr: Option<Vec<_>> = None;
    // if let Some(i) = arr1 {
    //     arr = Some(
    //         i.iter()
    //             .map(|x| x.to_lowercase())
    //             .unique()
    //             .collect::<Vec<_>>(),
    //     );
    // }
    // println!("{:?}", arr);
    // let x = [1, 2, 2, 3, 4, 3, 2, 1];
    // x.iter().dedup(); // [1, 2, 3, 4, 3, 2, 1]
    // let x1: Vec<_> = x.iter().unique().collect(); // [1, 2, 3, 4]
    // println!("{:?}", x1);

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // println!("{:?}", row);
    // for i in row.iter() {
    //     // println!("{:?}",i);

    // }

    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }
    // println!("a binding: {}", a_binding);

    let another_binding;
    // // Error! Use of uninitialized binding
    // println!("another binding: {}", another_binding);
    // // FIXME ^ Comment out this line
    another_binding = 1;
    // println!("another binding: {}", another_binding);

    //parse &str/string to number
    let v = ["san", "2", "5"];
    let res: Vec<_> = v.iter().map(|x| x.parse::<i32>()).collect();
    // println!("{:?}", res);

    let v1 = ["thiya".to_string(), "5".to_string()];
    let fi: Vec<_> = v1.iter().filter_map(|v| v.parse::<i32>().ok()).collect();
    // println!("parse:{:?}", fi);

    let amt1 = Amount {
        debit:40.0,
        credit: 0.0
    };
    let amt2 = Amount {
        debit:128.0,
        credit: 0.0
    };
    let amt_arr = vec![amt1,amt2];
    let ex_amount =  amt_arr.iter().fold(0.0, |x,y| x+y.debit - y.credit); //Folding is useful whenever you have a collection of something, and want to produce a single value from it.
    println!("ex_amount={:?}", &ex_amount);

    let amount = 500.0 - ex_amount;
    println!("amount={:?}", &amount);


    let cb1 = ChequeBook {
        id: "636d00dd1fad5572243ce759".to_string(),
        name: "TMB ACCOUNT".to_string(),
    };

    let cb2 = ChequeBook {
        id: "636d00dd1fad5572243ce75a".to_string(),
        name: "HDFC ACCOUNT".to_string(),
    };
    
    //Retains only the elements specified by the predicate. - retain()
    let mut  account_name = vec![cb1,cb2];
    account_name.retain(|x| x.id != *"636d00dd1fad5572243ce759");
    println!("account_name = {:?}", &account_name);
}
