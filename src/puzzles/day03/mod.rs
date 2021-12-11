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

    let mut gamma: String = String::new();
    let mut epsilon: String = String::new();
    for bit in frequency {   
        if bit > frequency_treshold {
            gamma.push('1');
            epsilon.push('0');
        }
        else{
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma_dec = isize::from_str_radix(&gamma, 2).unwrap();
    let epsilon_dec = isize::from_str_radix(&epsilon, 2).unwrap();
    
    println!("part 1: {}  {}", gamma , epsilon);
    println!("part 1: {} * {} = {}\n", gamma_dec, epsilon_dec, gamma_dec * epsilon_dec);
}


fn frequency_of_pos(index: usize, readings_list: &Vec<String>) -> i32 {
    let mut frequency = 0;
    
    for reading in readings_list {     
        let mut bits: Vec<_> = reading.split("").collect();
        bits.remove(0);
        bits.remove(bits.len()-1);

        frequency += bits[index].parse::<i32>().unwrap();
    }    
    return frequency;
}

fn get_val_at(reading: &String, index: usize) -> i32 {     
    let mut bits: Vec<_> = reading.split("").collect();
    bits.remove(0);
    bits.remove(bits.len()-1);

    return bits[index].parse::<i32>().unwrap();
}


fn part2(input: &Vec<String>) {
    let mut readings_generator: Vec<String> = input.clone();
    let mut readings_scrubber: Vec<String> = input.clone();
    
    for i in 0..12 {
        let treshold = (readings_generator.len() as f32 / 2.0) as f32;
        let val = frequency_of_pos(i, &readings_generator) as f32;

        // oxygen generator rating
        if val >= treshold {
            readings_generator.retain(|x| get_val_at(&x, i) == 1);
        } else {
            readings_generator.retain(|x| get_val_at(&x, i) == 0);
        }
        
        if readings_generator.len() == 1 {
            break;
        }
    }

    for i in 0..12 {
        let treshold = readings_scrubber.len() as f32 / 2.0;
        let val = frequency_of_pos(i, &readings_scrubber) as f32;       

        println!("val: {}     \ttreshold: {}     \tsize: {}", val, treshold, readings_scrubber.len());

        // CO2 scrubber rating
        if val >= treshold {
            readings_scrubber.retain(|x| get_val_at(&x, i) == 0);
        } else {
            readings_scrubber.retain(|x| get_val_at(&x, i) == 1);
        }
        
        if readings_scrubber.len() == 1 {
            break;
        }
    }

    println!("part 2: {}  {}", readings_generator[0], readings_scrubber[0]);

    let v1 = isize::from_str_radix(&readings_generator[0], 2).unwrap();
    let v2 = isize::from_str_radix(&readings_scrubber[0], 2).unwrap();

    println!("part 2: {} * {} = {}",v1, v2, v1 * v2)
}