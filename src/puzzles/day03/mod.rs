pub fn execute(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let mut frequency = vec![0,0,0,0,0,0,0,0,0,0,0,0];
    let frequency_treshold = (input.len()/2) as i32;

    for line in input {     
        let mut bits: Vec<_> = line.split("").collect();
        bits.remove(0);
        bits.remove(bits.len()-1);

        for (index, bit) in bits.iter().enumerate(){
            let value = bit.parse::<i32>().unwrap();
            frequency[index] += value;
        }
    }

    let mut v: String = String::new();
    for bit in frequency {
        print!(" {}", bit);       
        if bit > frequency_treshold {
            v.push('1');
        }
        else{
            v.push('0');
        }
    }
    println!("\n{}", v); // 2028 2067
    // 011111101100
    // 100000010011
    

    println!("part 1: {}\n", "day03")
}

fn part2(input: &Vec<String>) {
    println!("part 2: {}\n", "day03")
}