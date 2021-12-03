pub fn execute(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut increments: i32 = -1;
    let mut old: i32 = -1;

    for line in input {
        let val = line.parse::<i32>().unwrap();
        if  val > old {
            increments += 1;            
        }
        old = val;
    }
    println!("part 1: {}", increments)
}

fn part2(input: &Vec<String>) {
    let mut increments: i32 = -1;
    let mut old: i32 = -1;
    let mut sum: i32 = 0;

    for (index, _) in input.iter().enumerate() {
        if index + 3 > input.len() {
            break;
        }

        for i in index..index + 3 {
            sum += input[i].parse::<i32>().unwrap();
        }

        if  sum > old {
            increments += 1;            
        }
        old = sum;
        sum = 0;
    }
    println!("part 2: {}", increments)  
}
