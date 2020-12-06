use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    for m in 0..(lines.len()-1) {
        let a1: i32 = lines[m].parse().unwrap();
        for n in 0..(lines.len()-1) {
            let a2: i32 = lines[n].parse().unwrap();
            //println!(" a2 {} ", a2);
            if (a1 + a2) == 2020 {
                println!(" {} + {} = 2020", a1, a2);
                println!(" {} * {} = {}", a1, a2, a1*a2);
            }
        }
    }

}
