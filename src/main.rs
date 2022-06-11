use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let time_str = &args[1];
    let mut stack = Vec::<char>::new();
    let mut total_seconds: u32 = 0;
    for character in time_str.chars() {
        match character {
            '0'..='9' => stack.push(character),
            'd' | 'h' | 'm' | 's' => {
                let num = stack.iter().collect::<String>().parse::<u32>().unwrap();
                stack.clear();
                total_seconds += calc_seconds(character, num)
            }
            _ => panic!("Unrecognized character in time string")
        }
    }

    println!("{}", total_seconds);
}

fn calc_seconds(unit: char, num: u32) -> u32 {
    let multiplier = match unit {
        'd' => 24 * 60 * 60,
        'h' => 60 * 60,
        'm' => 60,
        's' => 1,
        _  => panic!("Unrecognized time unit character")
    };
    num * multiplier
}
