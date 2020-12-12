use std::fs::read_to_string;
use std::collections::HashSet;

pub fn day5(path: &str) {
//     let input = "FBFBBFFRLR
// BFFFBBFRRR
// FFFBBBFRRR
// BBFFBBFRLL";
    let input = read_to_string(path).unwrap();

    let mut seats = Vec::new();
    for boarding_pass in input.split_whitespace() {
        let mut row_range = 0..=127;
        let mut seat_range = 0..=7;
        for c in boarding_pass.chars() {
            match c {
                'F' => row_range = *row_range.start()..=((*row_range.end()-*row_range.start())/2+*row_range.start()),
                'B' => row_range = ((*row_range.end()-*row_range.start())/2+1+*row_range.start())..=*row_range.end(),
                'L' => seat_range = *seat_range.start()..=((*seat_range.end()-*seat_range.start())/2+*seat_range.start()),
                'R' => seat_range = ((*seat_range.end()-*seat_range.start())/2+1+*seat_range.start())..=*seat_range.end(),
                _ => {}
            }
        }

        let row;
        if row_range.start() == row_range.end() {
            row = row_range.start();
            // println!("FOUND ROW: {}", row);
        } else {
            println!("Something went wrong. Row range: {:?}", row_range);
            break;
        }
        let col;
        if seat_range.start() == seat_range.end() {
            col = seat_range.start();
            // println!("FOUND COLUMN: {}", col);
        } else {
            println!("Something went wrong. Seat range: {:?}", seat_range);
            break;
        }

        let seat_id = row*8+col;
        // println!("Seat ID: {}", seat_id);
        seats.push(seat_id);

    }

    // Find my seat
    seats.sort();
    for (i, s) in seats.iter().enumerate() {
        if i+1 == seats.len() {
            continue
        }
        if seats[i+1] != s + 1 {
            println!("{}", s + 1);
        }
    }
    println!("{:?}", seats);
}