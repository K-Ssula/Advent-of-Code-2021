pub fn execute(input: Vec<String>) {
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let drawn_numbers: Vec<&str> = input[0].split(",").collect();
    let mut bingo_cards: Vec<Card> = make_bingo_cards(input);
    let mut winner_card: Card = bingo_cards[0];
    let mut num = -1;

    'outer: for number in drawn_numbers {
        for card in &mut bingo_cards {
            card.mark_if_match(number.parse::<i32>().unwrap());
            if card.check_if_bingo() {
                winner_card = card.clone();
                num = number.parse::<i32>().unwrap();
                break 'outer;
            }
        }
    }

    winner_card.show();
    println!("\n\n\n\n")
    // println!("Points {} * {} = {}", winner_card.card_sum, num, winner_card.card_sum * num);
    
}

fn part2(input: &Vec<String>) {


    let drawn_numbers: Vec<&str> = input[0].split(",").collect();
    let mut bingo_cards: Vec<Card> = make_bingo_cards(input);
    let mut winner_card: Card = bingo_cards[0];
    let mut num = -1;




    for number in drawn_numbers {
        mark(&mut bingo_cards, number.parse::<i32>().unwrap());
        winners(&mut bingo_cards);
        
        bingo_cards.retain(|x| still_playing(x));

    }

    // winner_card.show();
    // println!("Points {} * {} = {}", winner_card.card_sum, num, winner_card.card_sum * num);
    
}

fn mark(cards: &mut Vec<Card>, num: i32) {
    for card in cards {
        card.mark_if_match(num);
    }
}

fn winners(cards: &mut Vec<Card>){
    for card in cards {
        card.check_if_bingo();         
    }
}

fn still_playing(card: &Card) -> bool {
    if card.bingo{
        card.show();
    }
    return !card.bingo;
}

fn make_bingo_cards(input: &Vec<String>) -> Vec<Card>{
    let mut begin = 2;
    let mut end = begin + 5;
    let number_of_cards = (input.len() - 1) / 6;

    let mut cards: Vec<Card> = Vec::new();

    for _ in 0..number_of_cards {
        let b = Card::new(input[begin..end].to_vec());
        cards.push(b);
        begin = end + 1;
        end = begin + 5;
    }

    return cards;
}


const SIZE: usize = 5;

#[derive(Clone, Copy, Debug)]
struct Card {
    card: [[(i32, bool); SIZE]; SIZE],
    card_sum: i32,
    selected_sum: i32,
    bingo: bool,
    last_marked: i32,
}

impl Card {
    fn new(input_board: Vec<String>) -> Card {
        let mut c = [[(-1, false); SIZE]; SIZE];
        let mut x = 0;
        let mut y = 0;
        let mut sum = 0;

        for line in input_board {
            let tokens: Vec<&str> = line.split(" ").collect();
            for val in tokens {
                if !val.is_empty() {
                    c[x][y].0 = val.parse::<i32>().unwrap();

                    sum += val.parse::<i32>().unwrap();
                    y += 1;
                }
            }
            x += 1;
            y = 0;
        }

        return Card {
            card: c,
            card_sum: sum,
            selected_sum: 0,
            bingo: false,
            last_marked: -1,
        } 
    }

    fn mark_if_match(&mut self, number: i32) {
        for line in 0..SIZE {
            for col in 0..SIZE {
                if number == self.card[line][col].0 {
                    self.card[line][col].1 = true;
                    self.card_sum -= number;
                    self.last_marked = number;
                }
            }
        }
    }

    fn check_if_bingo(&mut self) -> bool {
        if self.check_line() || self.check_col() {
            self.bingo = true;
        }
        return self.bingo;
    }

    fn check_line(&self) -> bool {     
        for line in 0..SIZE {
            let mut streak = 0;
            for col in 0..SIZE {
                if self.card[line][col].1 {
                    streak += 1;
                }
            }
            if streak == 5 {
                return true;
            }
        }
        return false;
    }

    fn check_col(&self) -> bool {
        for col in 0..SIZE {
            let mut streak = 0;
            for line in 0..SIZE {
                if self.card[line][col].1 {
                    streak += 1;
                }
            }
            if streak == 5 {
                return true;
            }
        }
        return false;
    }

    fn show(&self) {
        println!();
        for line in self.card {
            for cell in line {
                if cell.1 == true {
                    print!("{} X\t", cell.0);
                } else {
                    print!("{}  \t", cell.0);
                }
            }
            println!();
        }
        println!("Points {} * {} = {}", self.card_sum, self.last_marked, self.card_sum * self.last_marked);
    }
}