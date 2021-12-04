pub fn execute(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in input {     
        let token: Vec<&str> = command.split(" ").collect();
        let value = token[1].parse::<i32>().unwrap();
        
        match token[0] {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => println!("{}", "error"),
        }
    }
    println!("part 1: {}", horizontal * depth)
}

fn part2(input: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    for command in input {     
        let token: Vec<&str> = command.split(" ").collect();
        let value = token[1].parse::<i32>().unwrap();
        
        match token[0] {
            "forward" => {horizontal += value;
                          depth += aim * value},
            "down" => aim += value,
            "up" => aim -= value,
            _ => println!("{}", "error"),
        }
    }
    println!("part 2: {}\n", horizontal * depth)
}