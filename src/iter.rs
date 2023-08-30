// iter() iterates over the items by reference &T
// iter_mut() iterates over the items, giving a mutable reference to each item &mut T
// into_iter() iterates over the items, moving them into the new scope  T

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    salary: usize,
    age: u8,
}

pub fn run() {
    // //Array
    let arr = [2, 4, 6, 8, 3, 5, 7, 1];
    //     let iter = arr.iter();
    //     println!("{:?}",iter);
    //     println!("{:?}",arr.iter().next());
    // //last element
    //     println!("{:?}",arr.iter().last());

    // //count
    let count = arr.iter().count();
    //     println!("Count:{}",count);

    // //min
    let min = arr.iter().min();
    //     println!("min value:{:?}",arr.iter().min_by(|x, y| x.cmp(y)).unwrap());
    //     println!("min number:{:?}",min);

    // //max
    let max = arr.iter().max();
    //     let max_value = arr.iter().max_by(|x,y| x.cmp(y)).unwrap();
    //     println!("max num:{:?}",max);
    //     println!("max value:{:?}",max_value);

    //filter
    // let mut fil = arr.iter().filter(|x| **x > 5 );
    let mut fil = arr.iter().next().filter(|x| **x % 2 == 0);
    // println!("filter:{:?}",fil);
    // println!("filter:{:?}",fil.next());
    // println!("filter:{:?}",fil.next());
    // println!("filter:{:?}",fil.next());

    // //enumerate - current index of the iteration and value is returned
    let mut iter = arr.iter().enumerate();
    //     println!("{:?}", iter.next());

    // //skip
    let mut skip = arr.iter().skip(2);
    //     println!("{:?}", skip.next());

    // //map
    //     let m:Vec<i32>= arr.iter().map(|x| x*2).collect();
    //     println!("map:{:?}",m);

    // //find
    //     let find = arr.iter().find(|x| **x == 9);
    //     println!("{:?}",find); //None
    //     let find1 = arr.iter().find(|x| **x >5);
    //     println!("{:?}",&find1);

    //Vector

    let v1: Vec<i32> = vec![1, 12, 23, 34, 45];
    // let v = v1.reverse();
    // println!("rev:{:?}",v);
    // let map:Vec<i32> = v1.iter().next().map(|x| x+1).collect();
    // println!("map:{:?}", map);

    // let a1 = String::from("santhiyaa");
    // let a2 = String::from("sai");
    // let v2 = vec![a1,a2];
    // let vect:Vec<String> = v2.iter().map(|x| x.to_uppercase()).collect();
    // println!("upperCase:{:?}", vect);

    // let vect1: Vec<i32> = v1.into_iter().filter(|&x| x>25).collect();
    // println!("filter:{:?}",vect1);

    // let vect2= v1.into_iter().find(|&x| x <3) ;
    // let vect2= v1.into_iter().find(|&x| x <3).unwrap() ;
    // println!("find:{:?}", vect2);

    let test = vec!["one", "two", "three"];
    let index = test.iter().position(|&i| i == "two").unwrap();
    println!("test- {}", index);

    // let test = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    // let index = test.into_iter().position(|i| i == "four".to_string()).expect("no value");
    // println!("{}", index);

    //flatten
    let nested_vec: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
    let flat_vec: Vec<i32> = nested_vec.into_iter().flatten().collect();
    println!("flatten = {:?}", flat_vec);

    // let a1 = String::from("sun");
    // let a2 = String::from("moon");
    // let v:Vec<Vec<String>> = vec![vec![a1],vec![a2]];
    // let flat: Vec<String> = v.into_iter().flatten().collect();
    // println!("{:?}",flat);

    // fn main() {
    //     let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    //     for &index in [0, 2, 99].iter() {
    //         match fruits.get(index) {
    //             Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
    //             None => println!("There is no fruit! :("),
    //         }
    //     }
    // }

    let emp1 = Employee {
        name: "riya".to_string(),
        salary: 10000,
        age: 24,
    };
    let emp2 = Employee {
        name: "Thiya".to_string(),
        salary: 15000,
        age: 26,
    };
    let emp3 = Employee {
        name: "sai".to_string(),
        salary: 25000,
        age: 30,
    };
    let v = vec![emp1, emp2, emp3];
    let fil: Vec<Employee> = v.clone().into_iter().filter(|x| x.salary > 10000).collect();
    let find = v.iter().find(|x| x.age == 30);
    let map: Vec<String> = find.clone().iter().map(|x| x.name.to_uppercase()).collect(); // convert name to uppercase 
    // println!("emp-arr - {:#?}", &map);
    println!("fil - {:#?}", fil);
    println!("find - {:?}", find);
    println!("map: {:?}", &map);
    println!("emp-arr - {:#?}", &v);

    let mobile = vec!["980765890", "7654890321"];
    for mob in mobile {
        // println!("{}",&mob);
    }
    //iter vs into_iter
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // `iter()` for vecs yields `&i32`. Destructure to `i32`.
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // `into_iter()` for vecs yields `i32`. No destructuring required.
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // enumerate - current count and the element during iteration
    let items = ["pen", "pencil", "eraser"];
    for (i, x) in items.iter().enumerate() {
        println!("index {} item {}", i, x);
    }

    //parse &str into u32
    let v = ["1", "2", "5"];
    let res: Vec<_> = v.iter().map(|x| x.parse::<u32>()).collect();
    println!("res - {:?}", res);

    let v1 = "s";
    let res1 = v1.parse::<u32>().map_err(|x| x.to_string());
    println!("res1 - {:?}", res1);

    //filtermap
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    //find max element 
    let  a = [-1, 2, 3,5,-100,3,97];
    println!("max = {:?}", a.iter().max()); //97
    println!("min = {:?}", a.iter().min()); //-100
    println!("max-by = {:?}", a.iter().max_by(|x,y| x.cmp(y))); //97

    let a = [1, 2, 3];
    println!("out={}",a.iter().any(|&x| x > 0));
}
