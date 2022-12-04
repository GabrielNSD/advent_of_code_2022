use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    // let mut vec: Vec<i32> = Vec::new();

    let mut result = 0;
    let mut result_2 = 0;
    for line in reader.lines() {
        let line_1 = line.unwrap();
        let splitted = line_1.split_whitespace().collect::<Vec<_>>();

        let points_a = get_points(splitted[0]);
        let points_b = get_points(splitted[1]);

        let match_result = match_round(points_a, points_b);

        result += match_result + points_b;

        // part 2
        let cheat_move = equivalent_move(splitted[1], points_a);

        let match_result_2 = match_round(points_a, cheat_move);

        result_2 += match_result_2 + cheat_move;
    }

    println!("Result part 1: {result}");
    println!("Result part 2: {result_2}");

    Ok(())
}

// A | X => rock ,1
// B | Y => paper ,2
// C | Z => scisors ,3

fn get_points(play: &str) -> i32 {
    match play {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        _ => 0,
    }
}

fn equivalent_move(cheat_move: &str, enemy_point: i32) -> i32 {
    match cheat_move {
        "X" => what_wins_loses(enemy_point, false),
        "Y" => enemy_point,
        "Z" => what_wins_loses(enemy_point, true),
        _ => 0,
    }
}

// receives the enemy move and an indication if it should be a win or not
fn what_wins_loses(enemy_points: i32, wins: bool) -> i32 {
    let mut win_point = 0;
    if !wins {
        win_point += 1;
    } else {
        win_point = 0;
    }

    if enemy_points == 1 {
        2 + win_point
    } else if enemy_points == 2 {
        match wins {
            true => 3,
            false => 1,
        }
    } else if enemy_points == 3 {
        1 + win_point
    } else {
        0
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
        println!("no points condition {points_b}");
        0
    }
}
