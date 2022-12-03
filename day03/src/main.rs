#![allow(dead_code)]

fn main() {
    let filepath = "input.txt";
    // part1(filepath);
    part2(filepath);
}

fn to_priority(c: char) -> u8
{
    match c.is_lowercase()
    {
        true => c as u8 - 'a' as u8 + 1,
        false => c as u8 - 'A' as u8 + 27
    }
}

fn part1(filepath : &str)
{
    let contents = std::fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str());
    let lines = contents.split("\n");

    let mut score = 0;
    for line in lines
    {
        let parts = line.split_at(line.len()/2);
        let mut bitmask1 : u64 = 0;
        for c in parts.0.chars() {
            bitmask1 |= 1 << to_priority(c);
        }
        
        let mut bitmask2 : u64 = 0;
        for c in parts.1.chars() {
            bitmask2 |= 1 << to_priority(c);
        }
        score += ((bitmask1 & bitmask2) as f32).log2() as u64;
    }
    
    println!("{}", score);
}

fn part2(filepath: &str)
{
    let contents = std::fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str());
    let mut score = 0;
    
    let lines : Vec<&str> = contents.lines().collect();
    for i in 0..lines.len() / 3 
    {
        let mut bitmask1 : u64 = 0;
        for c in lines[i * 3].chars() {
            bitmask1 |= 1 << to_priority(c);
        }
        let mut bitmask2 : u64 = 0;
        for c in lines[i * 3+1].chars() {
            bitmask2 |= 1 << to_priority(c);
        }
        let mut bitmask3 : u64 = 0;
        for c in lines[i * 3+2].chars() {
            bitmask3 |= 1 << to_priority(c);
        }
        
        score += ((bitmask1 & bitmask2 & bitmask3) as f32).log2() as u64;
    }
    println!("{}", score);
}