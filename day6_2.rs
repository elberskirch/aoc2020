use std::env;
use std::fs;
use std::collections::HashMap; 

struct Group {
    members: i32,
    fields: HashMap<char, i32>,
}

impl Group {
    fn add_answer(&mut self, key: char) -> () {
    
        if self.fields.contains_key(&key) {
            
            if let Some(x) = self.fields.get_mut(&key) {
                *x += 1;
            }
            return ();
        }
        match key {
            'a'..='z' => self.fields.insert(key, 1),
            _ => None,
        };

    } 

    fn get_count(&self, key: char) -> i32 {
        *self.fields.get(&key).unwrap() as i32
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let raw: Vec<String> = contents.split("\n\n")
                                .map(|s| s.replace("\n"," "))
                                .collect();
   
    println!("{:?}", raw);
    let mut sum: i32 = 0;
    println!("{:?}", raw);
    for g in raw {
        let mut gr = g.clone();
        let mut m: Vec<&str> = g.split(" ").collect(); 
        // sanitize split, remove empty strings
        m.retain(|s| !s.is_empty());
        let mut group = Group { members: m.len() as i32, fields: HashMap::new()};
        
        gr.retain(|c| !c.is_whitespace());
        println!("------");
        println!("Members: {} - {:?}", m.len(), g);
        for c in g.chars() {
            group.add_answer(c);
        }
        for (key, value) in group.fields.iter() {
            if *value == group.members {
                sum += 1;
            }
        }
        
        //println!("Valid fields: {}", group.get_count());
    }

    println!("Sum: {}", sum);
}
