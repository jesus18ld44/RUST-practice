

fn main() {
    let mut v = vec![1,2,0,4,6,345,214,6,45,0,345,0];
    // insert a zero after each zero of the original vector   
    let mut cnt = 0;

    for i in &v {
        if *i == 0 {
            cnt += 1;
        }
    }
    assert_eq!(cnt, 3);

    for i in v.iter().rev() {
        println!("{}", i);
    }

}

