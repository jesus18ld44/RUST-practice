fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1,2,3];
    //         ^ macro to create new vector with initial values

    // to add elements `push` method
    v.push(5);
    
    println!("{:?}", v);

    let elem = &v[2];
    v.push(10);
    println!("{:?}", v);


    println!("{}", v.len());

    
    
    for i in v {
        println!("{}", i);
    }
    
}
