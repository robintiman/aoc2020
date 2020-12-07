use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;

pub fn day4(path: &str) {
//     let input = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in";
    let input = read_to_string(path).unwrap();

    let mut validation;
    let mut num_valid = 0;
    let sep = Regex::new(r"\n\n").unwrap();
    for passport in sep.split(&input) {
        validation = HashMap::new();
        for part in passport.split_whitespace() {
            let kv: Vec<&str> = part.splitn(2, ':').collect();
            // println!("{:?}", kv);
            validation.insert(kv[0], kv[1]);
        }
        // Check if valid
        if validation.len() == 8 {
            num_valid += 1;
        } else {
            if validation.len() == 7 && !validation.contains_key("cid") {
                num_valid += 1;
            }
        }
    }
    println!("{}", num_valid);
}
