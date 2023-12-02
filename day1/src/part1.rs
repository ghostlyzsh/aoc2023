use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = input.trim().split("\n").collect();
    let mut sum = 0;
    for line in lines {
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
