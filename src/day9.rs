use std::fs::read_to_string;
use std::ops::Range;

pub fn day9(path: &str) {
//     let input = "35
// 20
// 15
// 25
// 47
// 40
// 62
// 55
// 65
// 95
// 102
// 117
// 150
// 182
// 127
// 219
// 299
// 277
// 309
// 576";
    let input = read_to_string(path).unwrap();

    let mut numbers: Vec<i64> = input.lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let sum_to_find = 15690279;
    let mut i = 0;
    let mut sum = 0;
    let mut num = 0;
    let mut seq_range = 0..0;
    loop {
        if i >= numbers.len() { break }

        num = numbers[i];
        sum += num;
        seq_range.end += 1;
        if sum == sum_to_find {
            break;
        }
        i += 1;
        if sum > sum_to_find {
            sum = 0;
            seq_range.start += 1;
            i = seq_range.start;
            seq_range.end = seq_range.start
        }
    }

    let mut set = &mut numbers[seq_range];
    set.sort();
    let min = set.first().unwrap();
    let max = set.last().unwrap();
    println!("{} + {} = {}", min, max, min + max);

}

fn part1(numbers: &Vec<i64>) {
    let window_size = 25;
    let mut sums = Vec::new();
    let mut remove_i = Vec::new();
    let mut j = 0;
    for i in 1..window_size {
        remove_i.push(j);
        j += i;
    }
    println!("{:?}", remove_i);

    let mut first_i;
    let mut last_i;
    for (i, num) in numbers.iter().enumerate() {

        let diff = i as i32 - window_size as i32;

        if diff < 0 { first_i = 0 } else { first_i = i - window_size + 1 }

        if i == 0 { continue }
        last_i = i - 1;


        if i >= window_size {
            if !sums.contains(num) {
                println!("{}", num);
                break;
            }
            sums = sums.iter()
                .enumerate()
                .filter(|&(i, val)| !remove_i.contains(&i))
                .map(|(_, &val)| val)
                .collect();
        }

        let window = &numbers[first_i..=last_i];
        println!("window: {:?}", window);
        println!("num: {}", num);
        for b in window {
            sums.push(num + b)
        }
        println!("sums: {:?}", sums);
        println!();
    }
}