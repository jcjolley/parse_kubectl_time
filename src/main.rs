use std::env;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    let time_str = &args[1];
    let days_re = Regex::new(r"(\d+)d").unwrap();
    let hours_re = Regex::new(r"(\d+)h").unwrap();
    let minutes_re = Regex::new(r"(\d+)m").unwrap();
    let seconds_re = Regex::new(r"(\d+)s").unwrap();

    let days = capture_time_unit(time_str, &days_re);
    let hours = capture_time_unit(time_str, &hours_re);
    let minutes = capture_time_unit(time_str, &minutes_re);
    let seconds = capture_time_unit(time_str, &seconds_re);

    let total_seconds = (((days * 24) + hours) * 60 + minutes) * 60 + seconds;
    println!("{}", total_seconds);
}

fn capture_time_unit(time_str: &str, regex: &Regex) -> u32 {
    match regex.captures(time_str) {
        Some(cap) => cap.get(1).map_or("0", |m| m.as_str()).parse::<u32>().unwrap(),
        None => 0
    }
}
