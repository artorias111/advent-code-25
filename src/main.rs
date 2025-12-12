use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    let file= &args.file;

    parse_combination(file);
}


fn parse_combination(file: &str) {
    let f = File::open(file).unwrap();
    let reader = BufReader::new(f);

    let mut current_position = 50;
    
    let mut zero_count = 0;

    for line_result in reader.lines() { 
        let line = line_result.unwrap();
        let line = line.trim();
        
        current_position = return_position(current_position, line);
        
        if current_position == 0 {
            zero_count += 1
        }
        
    }

    println!("{zero_count}");

}



fn return_position(current_position: i32, code: &str) -> i32 {

    let mut dir = 0;
    let mut stride = String::new();

    for c in code.chars() {
        if c == 'R' {
            dir = 1;
        } else if c == 'L' {
            dir = -1;
        } else {
            stride.push(c);
        }
    }

    let stride_num = match stride.parse::<i32>() {
        Ok(number) => number,
        Err(_e) => 0
    };

    // okay, now you have your current position, direction and stride

    if dir == 1 {
        if current_position + stride_num <= 99 {
            return current_position + stride_num;
        } else {
            let mut cur_num = current_position + stride_num - 100;
            loop {
                if cur_num > 99 {
                    cur_num -= 100;
                } else {
                    break;
                }
            }
            return cur_num;
        }
    } else {
        if current_position - stride_num >= 0 {
            return current_position - stride_num;
        } else {
            let mut cur_num = current_position - stride_num + 100;
            loop {
                if cur_num < 0 {
                    cur_num += 100;
                } else {
                    break;
                }
            }
            return cur_num;
        }
    }
}




#[cfg(test)]
mod tests {
    use crate::return_position;

    #[test]
    fn test1_return_position() {
        let code = "L68";
        assert_eq!(return_position(50, code), 82);
    }

    #[test]
    fn test2_return_position() {
        let code = "R48";
        assert_eq!(return_position(52, code), 0);
    }
}
