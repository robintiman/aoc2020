const INPUT_FILE: &str = "/home/private/Documents/aoc/day1/src/input.txt";

fn read_integers(path: &str) -> Vec<i32> {
    let file_string = fs::read_to_string(path).unwrap();
    file_string.lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn day1() {
    let input = read_integers(INPUT_FILE);
    // let input = vec![1721, 979, 366, 299, 675, 1456];

    'outer: for i in 0..input.len() {
        for j in 0..input.len() {
            for k in 0..input.len() {
                if i == j || i == k || j == k {
                    continue
                }
                if input[i] + input[j] + input[k] == 2020 {
                    println!("{}", input[i] * input[j] * input[k]);
                    break 'outer
                }
            }
        }
    }
}