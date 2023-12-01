mod days;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    let day = args[1].parse::<u32>().unwrap_or_else(|_| {
        println!("Invalid day: {}", args[1]);
        print_usage();
        std::process::exit(1);
    });

    let part = args[2].parse::<u32>().unwrap_or_else(|_| {
        println!("Invalid part: {}", args[1]);
        print_usage();
        std::process::exit(1);
    });

    // There has to be a better way to do this
    match (day, part) {
        (1, 1) => days::day01::first(),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }

    std::process::exit(0);
}

fn print_usage() {
    println!("Usage: aoc2023 <day> <part> <input>");
    println!("Example: aoc2023 1 1 pqr3stu8vwx");
}
