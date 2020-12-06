use std::env;
use std::fs;
use std::collections::HashMap; 

struct Group {
    fields: HashMap<char, bool>
}

impl Group {
    fn add_answer(&mut self, key: char) -> () {
       match key {
           'a'..='z' => self.fields.insert(key, true),
           //'x'..='z' => self.fields.insert(key, true),
           _ => None,
       };

    } 

    /*
    */
    fn get_count(&self) -> i32 {
        let val: i32 = self.fields.keys().count() as i32;
        val
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let raw: Vec<String> = contents.split("\n\n")
                                .map(|s| s.replace("\n",""))
                                .collect();
   
    println!("{:?}", raw);
   
    let mut sum: i32 = 0;
    for g in raw {
        let mut group = Group { fields: HashMap::new()};
        
        println!("------");
        println!("{:?}", g);
        for c in g.chars() {
            group.add_answer(c);
        }
        sum += group.get_count();
        println!("Valid fields: {}", group.get_count());
    }
    
    /*let mut group = Group { fields: HashMap::new()};
    
    group.add_answer('z');
    group.add_answer('a');
    group.add_answer('a');
    group.add_answer('c');
    group.add_answer('m');
    group.add_answer('b');
    group.add_answer('j');*/

    println!("Sum: {}", sum);
}
