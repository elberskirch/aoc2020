use std::env;
use std::fs;
use std::collections::HashMap; 
use std::iter::FromIterator;

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
        let mut res: bool = true;
        if !self.check_fields() { res = false; return res; } 
        if !self.valid_fields() { res = false; } 
        return res;
    }

    fn check_fields(&self) -> bool {
        // these 7 keys are mandatory 
        let mandatory = ["ecl", 
                         "pid", "eyr", "hcl", "byr", "iyr", "hgt"];   
        let mut res: bool = true;
    
        for f in &mandatory {
            if !self.fields.contains_key(*f) { res = false; }
        }
        return res;
    }
    
    fn valid_fields(&self) -> bool {
        let mut res: bool = true;
        if !self.valid_byr() { res = false; }
        if !self.valid_iyr() { res = false; }
        if !self.valid_eyr() { res = false; }
       
        if !self.valid_hgt() { res = false; }
        if !self.valid_hcl() { res = false; }
        if !self.valid_ecl() { res = false; }
        if !self.valid_pid() { res = false; }
        return res;   
    }

    //byr (Birth Year) - four digits; at least 1920 and at most 2002.
    fn valid_byr(&self) -> bool {
        let mut res: bool = false;
        let raw = self.fields.get("byr"); 
        
        let mut ystr: &String;
        ystr = raw.unwrap();
        let year = ystr.parse::<i32>().unwrap();
        
        if year >= 1920 && year <= 2002 { res = true; }
        println!("byr {:?} {:?} {}", *ystr, ystr.parse::<i32>().unwrap(), res);
        return res
    }


    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    fn valid_iyr(&self) -> bool {
        let mut res: bool = false;
        let raw = self.fields.get("iyr"); 
        
        let mut ystr: &String;
        ystr = raw.unwrap();
        let year = ystr.parse::<i32>().unwrap();
        
        if year >= 2010 && year <= 2020 { res = true; }
        println!("iyr {:?} {:?} {}", *ystr, ystr.parse::<i32>().unwrap(), res);
        
        return res
    }
    
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    fn valid_eyr(&self) -> bool {
        let mut res: bool = false;
        let raw = self.fields.get("eyr"); 
        
        let mut ystr: &String;
        ystr = raw.unwrap();
        let year = ystr.parse::<i32>().unwrap();
        
        if year >= 2020 && year <= 2030 { res = true; }
        println!("eyr {:?} {:?} {}", *ystr, ystr.parse::<i32>().unwrap(), res);
        return res
    }

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    fn valid_hgt(&self) -> bool {
        let mut res: bool = true;
        let valid_hgt = [[150, 193], //cm span
                         [59,76]];   //inch span
        let raw = self.fields.get("hgt"); 
        let mut hgtstr: &String;
        hgtstr = raw.unwrap();
       
        // is cm/inch
        let last_two: Vec<char> = hgtstr.chars().rev().take(2).collect();
        let m = String::from_iter(last_two);
        
        let mut hgtvec: Vec<char> = hgtstr.chars().collect();
        // remove unit 
        hgtvec.pop();
        hgtvec.pop();
        
        // create numeric value
        let hgtparse = String::from_iter(hgtvec).parse::<i32>();
        let mut hgt: i32;

        println!("{:?}", hgtparse);
        match hgtparse {
            Err(hgtparse) => return false,
            Ok(hgtparse) => hgt = hgtparse,
        }
        // too lazy to switch it around
        if m == "mc" {
            if !(hgt >= valid_hgt[0][0] && hgt <= valid_hgt[0][1]) { res = false; }
        } else if m == "ni" {
            if !(hgt >= valid_hgt[1][0] && hgt <= valid_hgt[1][1]) { res = false; }
        } else {
            res = false;
        }
        println!("hgt {:?} {} {}", hgtstr, m, res);
        return res
    }

    //hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    fn valid_hcl(&self) -> bool {
        let mut res: bool = true;
        let raw = self.fields.get("hcl"); 
        let mut hclstr: &String;
        hclstr = raw.unwrap();
       
        let first = hclstr.chars().nth(0).unwrap();
        
        if hclstr.chars().count() != 7 { return false; }
        if first != '#' { res = false; }
        else {
            let mut it = hclstr.chars();

            //println!("{:?}", it.enumerate());
            for (i,_) in it.enumerate().skip(1) {
                let x = hclstr.chars().nth(i).unwrap();
                match x {
                    '0'..='9' => println!("{} {} 0..9", x, i as i8),
                    'a'..='f' => println!("{} {} a..f", x, i as i8),
                    _ => res = false,
                }
            }
        }
        println!("hcl {:?} {}", hclstr, res);
        return res
    }

    // validate eyecolor (ecl)
    fn valid_ecl(&self) -> bool {
        let mut res: bool = false;
        let valid_ecl = [ "amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        for ecl in &valid_ecl {
            if self.fields.get("ecl") == Some(&ecl.to_string()) { res = true; break; }
        }
        println!("ecl {}", res);
        return res;   
    }

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    fn valid_pid(&self) -> bool {
        let mut res: bool = true;
        // count digits
        let raw = self.fields.get("pid"); 
        
        let mut ystr: &String;
        ystr = raw.unwrap();
        let pid = ystr.matches(char::is_numeric).count();
        
        if pid != 9 {res = false;}
        println!("pid {}", res);
        return res;   
    }

    // cid (Country ID) - ignored, missing or not.
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let contents = fs::read_to_string(input_filename)
        .expect("Something went wrong reading the file");

    let ppraw: Vec<String> = contents.split("\n\n")
                                .map(|s| s.replace("\n"," "))
                                .collect();
   
    //println!("{:?}", ppraw);
    
    let mut valid_passports = 0;
    let mut invalid_passports = 0;
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
        
        println!("------------------");
        println!("{:?}", fields);
        for kv in &fields {
            let mut it = kv.split(":");
            let key = it.next().unwrap();
            let val = it.next().unwrap();
            passport.addfield(key, val);
        }
        if passport.valid() {
            valid_passports += 1;
            println!("--- VALID");
        } else {
            invalid_passports += 1;
            println!("--- INVALID");
        }
    }

    println!("Valid passports: {}", valid_passports);
    println!("Invalid passports: {}", invalid_passports);
}
