use std::env;
use std::fs;


struct Line {
    ch: char,
    min: u8,
    max: u8,
    pass: String,
}

fn parse(line: String) -> Line {
   let v: Vec<&str> = line.split(' ').collect();

   println!("{}\n", line);

   let m_v: Vec<&str> = v[0].split("-").collect();
   Line {
       pass: v[2].to_string(),
       ch: v[1].chars().nth(0).unwrap(),
       min: m_v[0].parse::<u8>().unwrap(),
       max: m_v[1].parse::<u8>().unwrap(),
   }
}

fn valid(l: &Line) -> bool {
    let count = l.pass.matches(l.ch).count();
    
    if (count >= (l.min as usize)) && (count <= (l.max as usize)) {
        true
    } else {
        false
    }
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut valid_lines: i32 = 0;

    for l in 0..(lines.len()-1) {
        let line: Line = parse(lines[l].to_string());
        
        if valid(&line) {
            valid_lines += 1;
        }
        println!(" {} {} {} {}", line.min, line.max, line.ch, line.pass);
    }
    println!("Valid lines: {}", valid_lines);
}
