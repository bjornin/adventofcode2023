use chrono::prelude::*;
use reqwest::header::COOKIE;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let day_of_month: u32 = if args.len() < 2 {
        Local::now().day()
    } else {
        args[1].parse::<u32>().unwrap()
    };
    println!("Day of the month: {}", day_of_month);
    let url = format!("https://adventofcode.com/2023/day/{}/input", day_of_month);
    let file_path = format!("input/day{}.txt", day_of_month);

    let input = if Path::new(&file_path).exists() {
        fs::read_to_string(file_path)?
    } else {
        println!("Read input from {}", url);
        let cookie = env::var("COOKIE").expect("COOKIE env var not set");
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).header(COOKIE, cookie).send()?.text()?;
        fs::write(file_path, &response)?;
        response
    };

    match day_of_month {
        1 => println!("day1 1:{} 2:{}", day1::solve1(&input), day1::solve2(&input)),
        2 => println!("day2 1:{} 2:{}", day2::solve1(&input), day2::solve2(&input)),
        3 => println!("day3 1:{} 2:{}", day3::solve1(&input), day3::solve2(&input)),
        // Add more cases for other days as needed
        _ => println!("No solution available for day {}", day_of_month),
    }

    Ok(())
}
