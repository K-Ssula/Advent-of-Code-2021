#![allow(dead_code)]
#![allow(unused_variables)]

mod util;
mod puzzles;

fn main() {    
    // To Do - refactoring will happen after day 5 is completed
    // add a puzzle selector and input being handled here makes no sense

    // let input = util::input_handler::read_file("day01");
    // puzzles::day01::execute(input) //p1: 1462 & p2: 1497     

    // let input = util::input_handler::read_file("day02");
    // puzzles::day02::execute(input) //p1: 1648020 & p2: 1759818555

    // let input = util::input_handler::read_file("day03");
    // puzzles::day03::execute(input) //p1: 4191876 & p2: 3414905

    let input = util::input_handler::read_file("day04");
    puzzles::day04::execute(input) //p1: 71708 & p2: 34726



}
