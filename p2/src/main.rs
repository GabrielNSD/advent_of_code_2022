use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // let mut vec: Vec<i32> = Vec::new();

    let mut result = 0;
    for line in reader.lines() {
        let line_1 = line.unwrap();
        let splitted = line_1.split_whitespace().collect::<Vec<_>>();

        let points_a = get_points(splitted[0]);
        let points_b = get_points(splitted[1]);

        let match_result = match_round(points_a, points_b);

        result += match_result + points_b;
    }

    println!("{result}");

    Ok(())
}

fn get_points(play: &str) -> i32 {
    match play {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn match_round(points_a: i32, points_b: i32) -> i32 {
    if points_b == 1 {
        match points_a {
            1 => 3,
            2 => 0,
            3 => 6,
            _ => 0,
        }
    } else if points_b == 2 {
        match points_a {
            1 => 6,
            2 => 3,
            3 => 0,
            _ => 0,
        }
    } else if points_b == 3 {
        match points_a {
            1 => 0,
            2 => 6,
            3 => 3,
            _ => 0,
        }
    } else {
        println!("no points condition");
        0
    }
}
