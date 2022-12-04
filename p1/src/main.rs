use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut vec: Vec<i32> = Vec::new();

    vec.push(0);
    let mut index = 0;
    for line in reader.lines() {
        let resulting = line.unwrap();

        if resulting == "" {
            index += 1;
            vec.push(0);
        } else {
            let parsed = resulting.parse::<i32>().unwrap();
            vec[index] += parsed;
        }
    }

    let index_of_max = vec
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();

    let max_cal = vec[index_of_max];

    println!("Top 1 Calories: {max_cal}\n");

    vec.sort_by(|a, b| b.cmp(a));

    let mut number = 0;
    let mut top_3_sum = 0;
    loop {
        let val = vec[number];
        top_3_sum += val;
        number += 1;
        if number == 3 {
            break;
        }
    }

    println!("{top_3_sum}");

    Ok(())
}
