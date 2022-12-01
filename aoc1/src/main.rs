use std::fs;

fn main() {
    let mut highest_cal: u32 = 0;
    let mut second_highest_cal: u32 = 0;
    let mut third_highest_cal: u32 = 0;
    let mut current_total: u32;
    let contents:String = fs::read_to_string("./src/input.txt").expect("file should be read");
    // println!("\n{contents}");

    for group in contents.split("\n\n") {

        current_total = 0;

        for number in group.split_whitespace() {
            current_total += number.parse::<u32>().unwrap();
        }

        if current_total >= highest_cal{
            third_highest_cal = second_highest_cal;
            second_highest_cal = highest_cal;
            highest_cal = current_total;
        }else if current_total >= second_highest_cal {
            third_highest_cal = second_highest_cal;
            second_highest_cal = current_total
        }else if current_total >= third_highest_cal {
            third_highest_cal = current_total;
        }
    }
    
    let top_total:u32 = highest_cal + second_highest_cal + third_highest_cal;

    print!("{highest_cal}\n");
    print!("{top_total}\n");

}
