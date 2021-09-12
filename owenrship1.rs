fn print_msg() {
    let mut msg = vec![1,1,1];
    for i in 3..10 {
        let next = msg[i-3] + msg[i-2];
        msg.push(next);
    }
    println!("P(1..10) = {:?}", msg);

}

fn main() {
    print_msg();    
    let msg = "Hola mundo";       // &str  allocated in the stack
    let mut msg_string = String::from("hello");     // allocated on the heap
        
    // msg_string.push_str(", world");
    //    ^^^^^ to push_str must be mut
    //msg.push_str(", mundo");
    //    ^^^^^^^^ method not found in &str

    
    {
        let mut r1 = &mut msg_string;
        change(&mut r1);
    }

    change(&mut msg_string);
    println!("{}", msg_string);
    first_word(&msg);

}

// some_string is a reference so we don't take ownership
// and must be mut in order to be able to change its value
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    println!("{:?}", bytes);

    for (i,&item) in bytes.iter().enumerate() {
    //                              ^^^^ enumerate returns a tuple (index, &element)        
        if item == b' ' {
            return &s[..i]
        }
    }

    &s[..]
}