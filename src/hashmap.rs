use std::collections::{HashMap, HashSet};

pub fn run() {
 //creating empty hashmap
    let mut map = HashMap::new();

 //inserting values
    map.insert(String::from("name"), String::from("Thiya"));
    map.insert(String::from("age"), "21".to_string());

//accessing values in hashmap
     let address = String::from("Address");
     map.insert( address.clone(), String::from("TUTY"));
     let value = map.get(&address); 
    // println!("state: {:?}",&value); //Some("TUTY")

    let state = String::from("state");
    let s = map.get(&state);
    // println!("state: {:?}",&s); // None(if there's no value for key get will return none)

//contains_key - checks if key exists in hashmap
    if map.contains_key(&address) {
        // let add = map.get(&address);
        // println!{"Address:{:?}", &add};
    }
    else {
        // let n = map.get(&state);
        // println!("State:{:?}", &n);
    }

//updating value
    map.insert(String::from("age"), "23".to_string()); //(overwritting a values)
    
    map.entry(String::from("age")).or_insert("24".to_string()); //check whether a particular key has a value,if it has it doesn't change the value , bcoz already age has the value
    map.entry(String::from("dob")).or_insert("1998".to_string()); //check whether a particular key has a value and, if it doesn’t, insert a value for it
    // println!("{:?}",&map);

//removing values
     map.remove("&state");
    //  println!("delete:{:?}",&map);

//length of the hashmap
    let length = map.len();
    // println!("length: {}", &length);

//keys in hashmap 
    let k = map.keys();
    // println!("keys:{:?}", k);

//values in hashmap
    let val = map.values();
    // println!("values: {:?}", &val);

//check the gvn hashmap empty or not - return bool
    let emp = map.is_empty();
    // println!("isEmpty:{:?}", emp);

    let stu: HashMap<String,i32> = HashMap::new();
    // println!("isEmpty{}", stu.is_empty());

//loop 
    let mut num : HashMap<String,i32> = HashMap::new();
    num.insert("k1".to_string(), 10);
    num.insert("k2".to_string(), 15);
    
    // for val in num.values_mut() {
    // //   println!("{:?}",val);
    //     *val = *val +3;
    // }
    // println!("changeVal:{:?}", &num);

    for (_k,v) in num.iter_mut() {
        *v = *v +50;
    }
    println!("changeVal:{:?}", &num);

    let mut marks: HashMap<String, Vec<i32>> = HashMap::new();
    let s1 = String::from("san");
    let s2 = String::from("thiya");
    marks.insert(s1, vec![80,75,60,78,90]);
    marks.insert(s2, vec![76,83,45,88,67]);
    // println!("{:?}",&marks);

   for (k,v) in marks {
    //    println!("key:{:?} val:{:?}", k,v);
    if k == "san" {
        for val in v {
            if val >= 90 && val <= 100{
               let r1 =  format!("{} {}", &val, "Excellent");
            //    println!("{:?}", &r1);
            }
            if val >= 80 && val <= 89 {
                let r2 = format!("{} {}", &val, "Good");
                // println!("{:?}", &r2);
 
            }
        }
    }
 }

//match
    let mut contacts  = HashMap::new();
    contacts.insert("sai", "8790654532") ;
    contacts.insert("sri", "7856342109");
    contacts.entry("sakthi").or_insert("6754231092");

    match contacts.get(&"sakthi") {
        Some(val) => 
        {
            // println!("Contact No.{}", val)
        },
        None => println!("No match found")
    }

// Try to find an entry in HashMap<&str, Vec<&str>>  
// If it does - it will push the &str (employee_name) into Vec<&str>. 
// Otherwise - it creates a new Vec<&str>.

    let mut designation : HashMap<&str, Vec<&str>> = HashMap::new();
    let department = "Testing";
    let emp_name = "Thiya";

    designation.insert("Developer", vec!["abi","abin"]);

    // designation.entry(&department)
    //     .or_insert_with(Vec::new)
    //     .push(emp_name);

    match designation.get_mut(&department) {
        Some(val) => {
            val.push(emp_name);
        } ,
        None => {
            designation.insert(department,vec!["srisha","saisha"]);
        }

    }
    // println!("{:#?}",designation);

    let mut classes: HashMap<String,Vec<String>> = HashMap::new();
    let s1 = classes.entry(String::from("class A")).or_insert(vec![String::from("Joy")]);
    // println!("{:?}", classes);

    // let mut classes : HashMap<String,HashSet<String>> = HashMap::new();
    // let stud1 = classes.entry(String::from("class B"));
    // stud1.or_default().insert(String::from("Bobby"));
    // println!("{:?}", classes);

}


// let mut marks: BTreeMap<String, Vec<i32>> = BTreeMap::new();
// let s1 = String::from("san");
// let s2 = String::from("thiya");
// marks.insert(s1, vec![80,75,60,78,90]);
// marks.insert(s2, vec![76,83,45,88,67]);
// println!("{:?}",&marks);