pub fn run() {
    let arr1 = [10,20,30];
    let arr2 = arr1;
    println!("{:?}",(arr2,arr1));

    let arr1:[i32;4] = [1,0,3,5];
    let arr2 = &arr1;
    println!("{:?}",(arr2,arr1));

    let v1 = vec![23,18,24,27];
    let v2 = &v1;
    println!("{:?}",(&v1,v2));

    
    let v1:Vec<i32> = vec![25,50,75];
    let v2 = &v1;
    println!("{:?}",(&v1,v2));

}