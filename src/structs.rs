#[derive(Debug)]
// struct Employee {
//     id:u8,
//     name: String,
//     age:u8
// }

//tuple struct
// struct Employee(u8,String,u8);

// struct Products {
//     id: i32,
//     prod: String,
//     cost: i32
// }

struct Person {
    id: i32,
    first_name: String,
    last_name: String
}



pub fn run() {
//    let mut emp = Employee {
//     id: 100,
//     name: String::from("santhiyaa"),
//     age: 23
//   };
//   emp.name = "Santhiyaa".to_string();
//   emp.age = 22;

//    println!("empId:{} empName:{} age:{}",emp.id,emp.name,emp.age);

    // let mut e = Employee(1,"Thiya".to_string(),23);
    // println!("{} {} {}",e.0,e.1,e.2);

    // let p1 = Products {
    //     id: 1,
    //     prod: String::from("Samsung"),
    //     cost: 10000
    // };
    // let p2 = Products {
    //     id: 2,
    //     prod: String::from("Headphone"),
    //     cost: 1500
    // };
    // let filter = filter_by_cost(p1,p2);
    //  println!("{:#?}",filter);
    // println!("{:#?}",p1); //error

    let per = Person {
        id: 1,
        first_name: "Harry".to_string(),
        last_name: "Potter".to_string()
    };
     println!("{:?}",per.get());
    // //  println!("{:#?}",(per.id,per.first_name,per.last_name));
    // println!("{:#?}",Person::get_details(per.id,per.first_name,per.last_name));

}

// fn filter_by_cost(prod1: Products, prod2: Products) -> Products{
//     // println!("{:?}",prod1);

//      if prod1.cost > prod2.cost {
//          prod1
//      } else {
//          prod2
//      }
//     }


impl Person {
    fn get(&self) {
      println!("{} {} {}",
             self.id,self.first_name,self.last_name)
    }

//     fn get(&self) -> String {
//         format!("{} {} {}",
//                self.id,self.first_name,self.last_name)
//       }

//     //static method
//     fn get_details(id: i32, first_name: String, last_name: String) -> Person {
//         Person {
//            id,
//            first_name,
//            last_name
//         }
//     }
}    