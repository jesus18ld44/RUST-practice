use std::collections::HashMap;

fn mean(map: &HashMap<i32, i32>) -> f32{
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    for (x, y) in map.iter() {
        println!("{:?}: {:?}", x, y);
        a += x*y;
        b += x;
    }

    (a as f32)/(b as f32)
}

fn main() {

    let numbers = vec![1,4,2,1,4,6,1,3,3,3,1,1,2,6,7,8,0,10,11,15];
    // for _ in numbers.iter() {
    //     zero_values.push(0);
    // }

    let mut map: HashMap<i32, i32> = HashMap::new();
    
    /// or_insert returns a mutable reference to the value for this key
    /// We store that mutable reference in the `count` variable, so in order
    /// to assign to that value, we must first dereference using (*)
    for num in numbers.iter() {
        // let count = map.entry(*num).or_insert(*num);
        // *count += 1;
        *map.entry(*num).or_insert(*num) += 1;
    }

    println!("{:?}", map);

    let mean = mean(&map);
    println!("{:?}", mean);

}