use reqwest::header::COOKIE;
use std::env;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://adventofcode.com/2023/day/1/input";
    let file_path = "input/day1.txt";

    let input = if Path::new(file_path).exists() {
        fs::read_to_string(file_path)?
    } else {
        println!("Read input from {}", url);
        let cookie = env::var("COOKIE").expect("COOKIE env var not set");
        let client = reqwest::blocking::Client::new();
        let response = client.get(url).header(COOKIE, cookie).send()?.text()?;
        fs::write(file_path, &response)?;
        response
    };

    println!("day1 1:{} 2:{}", day1::solve1(&input), day1::solve2(&input));

    Ok(())
}
