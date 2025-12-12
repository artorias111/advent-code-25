use advent_code_25::{get_args, read_lines};

fn main() {
    let args = get_args();
    let filepath = &args.file;

    let file = read_lines(filepath).unwrap();

    for line_result in file {
        let line = line_result.unwrap();
        let line = line.trim();
        
        let summed_dups = find_dups(&line);
        println!("{summed_dups}"); // there's only one line in the file
    }
}

fn find_dups(line: &str) -> u64 {
    let key = ",";
    let mut s: u64 = 0;
    let parts: Vec<&str> = line.split(key).collect();
    for part in parts {
        let coords: Vec<&str> = part.split("-").collect();
        let start = match coords[0].parse::<u64>() {
            Ok(number) => number,
            Err(_e) => 0,
        };
        let end = match coords[1].parse::<u64>() {
            Ok(number) => number,
            Err(_e) => 0,
        };
        s += sum_dups_range((start, end));
    }
    s 
}

fn sum_dups_range(range: (u64, u64)) -> u64 { // input: a ranged tuple, output: sum of all dups
    let (start, end) = range;
    let mut numd: u64;

    let mut s: u64 = 0;

    for i in start..end+1 {
        numd = num_digits(i);
        if numd % 2 == 0 {
            let base: u64 = 10;
            let split_num: u32 = (numd/2) as u32;
            let split_num = base.pow(split_num);

            if i % (split_num as u64) == i / (split_num as u64) {
                s += i;
            }
        }
    }
    s
}


fn num_digits(mut num: u64) -> u64 { // find the number of digits in a number
    let mut nd: u64 = 0;
    loop {
        num = num/10;
        if num == 0 {
            return nd + 1;
        } else {
            nd +=1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::num_digits;

    #[test]
    fn test1_num_digits() {
        assert_eq!(num_digits(822323), 6);
        assert_eq!(num_digits(3), 1);
    }

    use crate::sum_dups_range; 

    #[test]
    fn test1_sum_dup_range() {
        assert_eq!(sum_dups_range((11,22)), 33);
    }

    #[test]
    fn test2_sum_dup_range() {
        assert_eq!(sum_dups_range((1188511880,1188511890)), 1188511885);
    }
}
