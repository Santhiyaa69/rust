#[derive(Debug)]
#[allow(dead_code)]

struct Student {
    id: usize,
    name: String,
    marks: usize
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


    // let mut products:Vec<&str> = vec!["pen","pencil"];
    // println!("vectors:{:?}",products);

    // products[0] = "Pen";
    // println!("vectors:{:?}",products);
    // println!("vectors length:{:?}",products.len());

    // products.push("marker");
    // products.push("sketch");
    // println!("Add:{:?}",products);

    // products.pop();
    // println!("remove:{:?}",products);

    // for x in products.iter(){
    //     println!("{}",x);
    // }

    // for no in num.iter_mut(){
    //     *no +=2;
    // }
    // println!("{:?}",num);

    let s1 = Student {
        id: 1,
        name: "sandy".to_string(),
        marks:400
    };
    let s2 = Student{
        id: 2,
        name: "devi".to_string(),
        marks:450
    };
    let s3 = Student {
        id: 3,
        name: "divya".to_string(),
        marks:480

    };
    let vect = vec![s1,s2,s3];
    // println!("{:#?}",vect);
    for stud in vect.iter(){
       let s = stud.marks;
       if s>450 {
           println!("{:?}",stud)
       }
    }
}