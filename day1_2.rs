use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    
    for l in 0..(lines.len()-3) { 
        for m in (l+1)..(lines.len()-2) {
            //let a1: i32 = lines[m].parse().unwrap();
            for n in (m+1)..(lines.len()-1) {
                //println!(" {} + {} + {} ", l, m, n);
                let a1: i32 = lines[l].parse().unwrap();
                let a2: i32 = lines[m].parse().unwrap();
                let a3: i32 = lines[n].parse().unwrap();
                if (a1 + a2 + a3) == 2020 {
                    println!(" {} + {} + {} = 2020", a1, a2, a3);
                    println!(" {} * {} * {} = {}", a1, a2, a3, a1*a2*a3);
                    break;
                }
            }
        }
    }
}
