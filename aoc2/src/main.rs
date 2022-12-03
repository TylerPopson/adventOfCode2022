use std::fs;

fn main() {

    let contents:String = fs::read_to_string("./src/input.txt").expect("file should be read");
    let mut your_score: u32 = 0;

    for round in contents.split("\n"){
        let elf = round.chars().next().unwrap();
        let you = round.chars().next_back().unwrap();

        if elf == 'A'{
            if you == 'X' {
                your_score += 3;
            }
            if you == 'Y' {
                your_score += 4;
            }
            if you == 'Z' {
                your_score += 8;
            }
        }else if elf == 'B' {
            if you == 'X' {
                your_score += 1;
            }
            if you == 'Y' {
                your_score += 5;
            }
            if you == 'Z' {
                your_score += 9;
            }
        }else if elf == 'C' {
            if you == 'X' {
                your_score += 2;
            }
            if you == 'Y' {
                your_score += 6;
            }
            if you == 'Z' {
                your_score += 7;
            }
        }

    }

    println!("{your_score}")

}
