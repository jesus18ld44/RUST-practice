use std::collections::HashMap;

fn main() {

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Red)"), 50);

    let teams = vec![String::from("Blue"), String::from("Red")];
    let initial_scores = vec![10 ,50];

    let mut scores: HashMap<_,_> =  teams.into_iter().zip(initial_scores.into_iter()).collect();
    //                      ^ ^ the compiler infers the type that the hash map contains
    /// Ownership
    /// For types that implement the Copy trait, like i32, the values are copied into 
    /// the hash map. For owned values like `String`, the values will be moved and the 
    /// hash map will be the owner of those values
    
    // println!("{:?}", teams);
    //                  ^^^^^ borrow of moved value Vec<String> does not implement Copy trait
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score.unwrap());
    println!("{:?}", scores);

    for (key,value) in &scores {
    //                 ^ iterate with a reference, so the vector can be accessed later
        println!("{}: {}", key, value);
    }

    println!("{:?}", scores);
    
    // Although the number if K&V is growable, each key can only have one value associated with it at a sys_time
    


}