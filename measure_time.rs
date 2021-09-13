use std::time::SystemTime;

fn main() {

    let sys_time = SystemTime::now();

    // code here
    
    let new_sys_time = SystemTime::now();
    let elapsed_time = new_sys_time.duration_since(sys_time)
        .expect("clock may have gone backwards");


}