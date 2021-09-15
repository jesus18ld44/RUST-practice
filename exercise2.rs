/// Pig latin
/// 1st consonant is moved to the end, and 'ay' is appended
/// p.e.: first -> irst-fay
/// Words that start with a vowel have 'hay'
/// added to the end 
/// p.e.: apple -> apple-hay

use std::io;

fn pig_latin(word: &String) -> String {
    let res;
    

    match &word[0..1] {
        "a" | "e" | "i" | "o" | "u" => res = word.to_string() + "-hay",
        _ => res = word[1..].to_string() + "-" + &word[0..1] + "ay",
    }

    res

}

fn main() {

    loop {
        let mut word = String::new();
        let mut pig_word = String::new(); 

        io::stdin()
            .read_line(&mut word)
            .expect("Failed");

        let word: String = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,

        };

        pig_word = pig_latin(&word);
        
        println!("{:?}", pig_word);

    }
}