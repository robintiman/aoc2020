use std::collections::HashMap;
use regex::Regex;
use std::fs::read_to_string;


pub fn day4(path: &str) {
//     let input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f
//
// eyr:2029 ecl:blu cid:129 byr:1989
// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//
// hcl:#888785
// hgt:164cm byr:2001 iyr:2015 cid:88
// pid:545766238 ecl:hzl
// eyr:2022
//
// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    let input = read_to_string(path).unwrap();

    let mut num_valid = 0;
    let sep = Regex::new(r"\n\n").unwrap();
    'outer: for passport in sep.split(&input) {
        let mut num_valid_parts = 0;
        let mut found_cid = false;
        let mut found_keys = HashMap::new();
        for part in passport.split_whitespace() {
            let kv: Vec<&str> = part.splitn(2, ':').collect();
            if !is_valid(kv[0], kv[1]) {
                continue 'outer;
            } else {
                if found_keys.contains_key(kv[0]) {
                    println!("{:?}", kv);
                    continue 'outer;
                }
                found_keys.insert(kv[0], kv[1]);
                num_valid_parts += 1;
            }
            if kv[0] == "cid" {
                found_cid = true;
            }
        }

        if num_valid_parts < 8 {
            if num_valid_parts == 7 && !found_cid {
                println!("{:?}", found_keys);
                num_valid += 1;
            }
        } else {
            num_valid += 1;
        }
    }
    println!("{}", num_valid);
}

fn is_valid(k: &str, v: &str) -> bool {
    fn is_valid_birth_year(y: i16) -> bool {
        y <= 2002 && y >= 1920
    }
    fn is_valid_issue_year(y: i16) -> bool {
        y <= 2020 && y >= 2010
    }
    fn is_valid_exp_year(y: i16) -> bool {
        y <= 2030 && y >= 2020
    }
    fn is_valid_height(h: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<digits>\d+)(?P<unit>\w+)").unwrap();
        }

        match RE.captures(h) {
            Some(cap) => {
                let digits = cap.name("digits").unwrap().as_str().parse::<i16>().unwrap();
                let unit = cap.name("unit").unwrap().as_str();
                match unit {
                    "cm" => digits >= 150 && digits <= 193,
                    "in" => digits >= 59 && digits <= 76,
                    _ => false
                }
            }
            None => false
        }
    }

    fn is_valid_hair_color(clr: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[a-zA-Z\d]+").unwrap();
        }
        RE.is_match(clr)
    }

    fn is_valid_eye_color(clr: &str) -> bool {
        match clr {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false
        }
    }

    fn is_valid_pid(pid: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d{9}").unwrap();
        }
        RE.is_match(pid)
    }
    match k {
        "byr" => if !is_valid_birth_year(v.parse::<i16>().unwrap()) {
            return false;
        },
        "iyr" => if !is_valid_issue_year(v.parse::<i16>().unwrap()) {
            return false;
        },
        "eyr" => if !is_valid_exp_year(v.parse::<i16>().unwrap()) {
            return false;
        },
        "hgt" => if !is_valid_height(v) {
            return false;
        },
        "hcl" => if !is_valid_hair_color(v) {
            return false;
        },
        "ecl" => if !is_valid_eye_color(v) {
            return false;
        },
        "pid" => if !is_valid_pid(v) {
            return false;
        },
        _ => {}
    }
    return true;
}