use advent_code_25::{get_args, read_lines};

fn main() {
    let args = get_args();
    let filepath = &args.file;

    let file = read_lines(filepath).unwrap();

    for line_result in file {
        let line = line_result.unwrap();
        let line = line.trim();

        println!("{line}");
    }
}
