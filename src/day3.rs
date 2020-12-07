use std::fs;

pub fn day3(path: &str) {
//     let input = "..##.......
// #...#...#..
// .#....#..#.
// ..#.#...#.#
// .#...##..#.
// ..#.##.....
// .#.#.#....#
// .#........#
// #.##...#...
// #...##....#
// .#..#...#.#";
    let input = fs::read_to_string(path).unwrap();
    let area = construct_area(&input);
    let step_configs = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let mut solution = 1;
    let mut num_trees;
    for conf in step_configs {
        num_trees = traverse(&area, conf);
        println!("{}", num_trees);
        solution *= num_trees;
    }
    println!("{}", solution);
}

fn traverse(area: &Vec<Vec<i8>>, step_config: (usize, usize)) -> i64 {
    let mut num_trees = 0;
    let mut pos = (0, 0);
    let shape = (area.len(), area[0].len());
    loop {
        pos = step(pos, shape.1, step_config);
        // Check if we're at the bottom.
        if pos.0 >= shape.0 {
            break;
        }
        num_trees += area[pos.0][pos.1] as i64;
    }
    return num_trees;
}

fn step(pos: (usize, usize), width: usize, step_config: (usize, usize),
) -> (usize, usize) {
    let mut next = (pos.0 + step_config.0, pos.1 + step_config.1);
    if next.1 >= width {
        next.1 = next.1 - width;
    }
    return next;
}

fn get_dims_of_area(input: &str) -> (usize, usize) {
    let mut height = 0 as usize;
    let mut width = 0;
    for (i, line) in input.lines().enumerate() {
        height += 1;
        if i == 0 {
            width = line.chars().count()
        }
    }
    return (height, width);
}

fn construct_area(input: &str) -> Vec<Vec<i8>> {
    let (height, width) = get_dims_of_area(input);
    let mut area = vec![vec![0; width]; height];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                area[y][x] = 1;
            }
        }
    }
    return area;
}