mod days;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 3 {
        print_usage();
        std::process::exit(1);
    }

    let day = args[1].parse::<u32>().unwrap_or_else(|_| {
        println!("Invalid day: {}", args[1]);
        print_usage();
        std::process::exit(1);
    });

    let part = args[2].parse::<u32>().unwrap_or_else(|_| {
        println!("Invalid part: {}", args[2]);
        print_usage();
        std::process::exit(1);
    });

    // There has to be a better way to do this
    match (day, part) {
        (1, 1) => days::day01::first(),
        (1, 2) => days::day01::second(),
        (2, 1) => println!("Day 2, part 1: {}", days::day02::first(None)),
        (2, 2) => println!("Day 2, part 2: {}", days::day02::second(None)),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}

fn print_usage() {
    println!("Usage: aoc2023 <day> <part>");
    println!("Example: aoc2023 1 1");
}
