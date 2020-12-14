use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::hash::Hash;

pub fn day6(path: &str) {
//     let input = "abc
//
// a
// b
// c
//
// ab
// ac
//
// a
// a
// a
// a
//
// b";
    let input = read_to_string(path).unwrap();
    let sep = Regex::new(r"\n\n").unwrap();
    let re = Regex::new(r"\w+").unwrap();
    let mut yes_count = 0;
    for(i, group) in sep.split(&input).enumerate() {
        let mut result: HashSet<_> = HashSet::new();
        println!("group: {}", i);
        for (i, person) in re.find_iter(group)
            .map(|p| p.as_str())
            .enumerate() {
            println!("{}", person);
            let answers: HashSet<_> = person.chars().collect();
            if i == 0 {
                result = answers;
            } else {
                result = result.intersection(&answers).cloned().collect();
            }
        }
        yes_count += result.len();
        println!("{:?}", result);
    }
    println!("{}", yes_count);
}
