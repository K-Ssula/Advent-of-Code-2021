pub fn execute(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut horizontal = 0;
    let mut depth = 0;

    for command in input {     
        let token: Vec<&str> = command.split(" ").collect();
        
        match token[0] {
            "forward" => horizontal += token[1].parse::<i32>().unwrap(),
            "down" => depth += token[1].parse::<i32>().unwrap(),
            "up" => depth -= token[1].parse::<i32>().unwrap(),
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
        
        match token[0] {
            "forward" => {horizontal += token[1].parse::<i32>().unwrap();
                          depth += aim * token[1].parse::<i32>().unwrap()},
            "down" => aim += token[1].parse::<i32>().unwrap(),
            "up" => aim -= token[1].parse::<i32>().unwrap(),
            _ => println!("{}", "error"),
        }
    }
    println!("part 2: {}\n", horizontal * depth)
}