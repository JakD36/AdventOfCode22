#![allow(dead_code)]

use std::fs::read_to_string;

#[derive(Debug)]
struct Section
{
    min: u32,
    max: u32
}

impl Section
{
    fn contains(&self, other: &Section) -> bool
    {
        self.min >= other.min && self.max <= other.max
    }

    fn overlaps(&self, other: &Section) -> bool
    {
        self.min <= other.max && self.max >= other.min
    }
}

impl FromIterator<u32> for Section {
    fn from_iter<T: IntoIterator<Item=u32>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Section
        {
            min: iter.next().expect("Failed to find first value in pair."),
            max: iter.next().expect("Failed to find second value in pair."),
        }
    }
}

fn main()
{
    let filepath = "input.txt";
    part1(filepath);
    part2(filepath);
}

fn part1(filepath: &str)
{
    let contents = read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let pairs = contents
        .lines()
        .map(|line| Vec::from_iter(line.split(",")
            .map(|pair| Section::from_iter(
                pair.split("-")
                    .map(|x| x.parse::<u32>().expect(format!("Failed to parse {} as u32",x).as_str()))
            ))
        ));
    
    let mut count = 0;
    for pair in pairs 
    {
        // println!("{:?}",&pair[0]);
        // println!("{:?}",&pair[1]);
        count += (pair[0].contains(&pair[1]) || pair[1].contains(&pair[0])) as i32;
    }
    
    println!("Part 1 = {}", count);
}

fn part2(filepath: &str)
{
    let contents = read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let pairs = contents
        .lines()
        .map(|line| Vec::from_iter(line.split(",")
            .map(|pair| Section::from_iter(
                pair.split("-")
                    .map(|x| x.parse::<u32>().expect(format!("Failed to parse {} as u32",x).as_str()))
            ))
        ));

    let mut count = 0;
    for pair in pairs
    {
        count += (pair[0].overlaps(&pair[1])) as i32; // AABB collision
    }

    println!("Part 2 = {}", count);
}