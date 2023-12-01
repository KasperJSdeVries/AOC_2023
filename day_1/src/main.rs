use std::{env, fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let file_name = env::args().nth(1).expect("Please provide input file");

    let mut f = File::open(file_name)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;

    let result: i32 = buf
        .lines()
        .map(|line| {
            let mut first_and_last = String::new();
            first_and_last += &line
                .chars()
                .find(|x| x.is_ascii_digit())
                .unwrap()
                .to_string();
            first_and_last += &line
                .chars()
                .rev()
                .find(|x| x.is_ascii_digit())
                .unwrap()
                .to_string();

            first_and_last.parse::<i32>().unwrap()
        })
        .sum();

    println!("{}", result);

    Ok(())
}
