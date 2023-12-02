use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut sum = 0;
    for line in lines {
        let line = line.replace("eightwo", "82").replace("eighthree", "83").replace("oneight", "18").replace("fiveight", "58").replace("threeight", "38")
                    .replace("nineight", "98").replace("twone", "21").replace("sevenine", "79").replace("zero", "0").replace("one", "1").replace("two", "2").replace("three", "3")
                    .replace("four", "4").replace("five", "5").replace("six", "6").replace("seven", "7").replace("eight", "8").replace("nine", "9");
        let mut first = '\0';
        let mut last = '\0';
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                if first != '\0' {
                    last = c;
                } else {
                    first = c;
                    last = c;
                }
            }
        }
        let num = first.to_digit(10).unwrap()*10 + last.to_digit(10).unwrap();
        sum += num;
    }
    println!("{}", sum);
}
