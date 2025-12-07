use std::env;
use std::io::{self, Read};
use anyhow::{Result, anyhow};
use reqwest::header;

mod solution;
mod year2025;

fn fetch_input(year: u32, day: u32, session_cookie: &str) -> Result<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie_value = format!("session={}", session_cookie);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&cookie_value)?
    );

    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get(url).send()?;
    let text = res.text()?;

    Ok(text)
}

fn main() -> Result<()>{
    dotenvy::dotenv().ok();

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return Err(anyhow!(
            "Usage: {} <year> <day> [OPTIONS]\n
            Options:\n
            --test \"input\"  Use inline test input
            --stdin          Read test input from stdin\n",
            args.get(0).map(|s| s.as_str()).unwrap_or("advent_of_code")
        ));
    }

    let year: u32 = args[1].parse::<u32>()
        .map_err(|_| anyhow!("Invalid year: '{}'. Year must be a number.", args[1]))?;

    let day: u32 = args[2].parse::<u32>()
        .map_err(|_| anyhow!("Invalid day: '{}'. Day must be a number.", args[2]))?;

    let input = if args.len() > 3 {
        match args[3].as_str() {
            "--test" | "-t" => {
                if args.len() < 5 {
                    return Err(anyhow!("--test flag requires an input string"));
                }
                args[4].clone()
            }
            "--stdin" => {
                let mut buffer = String::new();
                io::stdin().read_to_string(&mut buffer)?;
                buffer
            }
            flag => {
                return Err(anyhow!("Unknown flag: '{}'. Use --test or --stdin", flag));
            }
        }
    } else {
        // Fetch from Advent of Code
        let session_cookie = env::var("AOC_SESSION")
            .map_err(|_| anyhow!("AOC_SESSION environment variable not set"))?;
        fetch_input(year, day, &session_cookie)?
    };

    let solutions = match year {
        2025 => year2025::get_solutions(),
        _ => return Err(anyhow!("Year {} not implemented", year))
    };

    let solution = solutions.get(&day)
        .ok_or_else(|| anyhow!("Day {} not implemented for year {}", day, year))?;
    solution.run(&input)?;

    Ok(())
}
