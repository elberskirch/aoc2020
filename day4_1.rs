use std::env;
use std::fs;
use std::collections::HashMap; 

struct Passport {
    fields: HashMap<String, String>
}

impl Passport {
    fn addfield(&mut self, key: &str, value: &str ) -> () {
       self.fields.insert(key.to_string(), value.to_string());  
    } 

    /*
       ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
    */
    fn valid(&self) -> bool {
        // these 7 keys are mandatory 
        let mandatory = ["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];   
        let mut res: bool = true;
    
        for f in &mandatory {
            if !self.fields.contains_key(*f) { res = false; }
        }
        return res;
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let mut ppraw: Vec<String> = contents.split("\n\n")
                                .map(|s| s.replace("\n"," "))
                                .collect();
   
    println!("{:?}", ppraw);
    
    let mut valid_passports = 0;
    for pp in ppraw {
        let mut passport = Passport{ fields: HashMap::new()};
        /*
        let fields: Vec<String> = pp.split_whitespace()
                                    .map(|f|{
                                        let iter = f.split(":"); 
                                        let key = iter.next().unwrap();
                                        let value = iter.next().unwrap();
                                        passport.addfield(key,value) 
                                    })
                                    .collect();
        */
        let fields: Vec<String> = pp.split_whitespace().map(|s| s.to_string()).collect();
        for kv in &fields {
            let mut it = kv.split(":");
            let key = it.next().unwrap();
            let val = it.next().unwrap();
            passport.addfield(key, val);
        }
        if passport.valid() {
            valid_passports += 1;
            println!("--- VALID");
        } 
        println!("{:?}", fields);
    }

    /*
    let mut pp1 = Passport{ fields: HashMap::new()};
    pp1.addfield("ecl", "gry");
    pp1.addfield("pid", "868686");
    pp1.addfield("eyr", "2020");
    pp1.addfield("hcl", "#fffffd");
    pp1.addfield("byr", "1937");
    pp1.addfield("iyr", "2017");
    pp1.addfield("cid", "147");
    pp1.addfield("hgt", "183cm");
    */
    println!("Valid passports: {}", valid_passports);
}
