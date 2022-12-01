use std::fs;
use std::str::FromStr;

fn main()
{
    let filepath = "../day01/input.txt";
    part1(filepath);
    part2(filepath);
}

fn part1(filepath: &str)
{
    let max : u32 = fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str())
        .split("\n\n")
        .map(|elf| elf.split("\n")
            .map(|x| u32::from_str(x)
                .expect(format!("Failed to parse {} as a u32", x).as_str()))
            .sum())
        .max().expect("Failed to sum and find the max value!");
    println!("{}\n", max);
}

fn part2(filepath: &str)
{
    let mut values : Vec<u32> = fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str())
        .split("\n\n")
        .map(|elf| elf.split("\n")
            .map(|x| u32::from_str(x)
                .expect(format!("Failed to parse {} as a u32", x).as_str()))
            .sum())
        .collect();
    values.sort();
    println!("{}\n", values[values.len()-3..].iter().sum::<u32>());
}