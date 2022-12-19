#![allow(dead_code)]
use std::collections::HashSet;
use std::fs::read_to_string;
use regex::Regex;
use std::cmp::{max, min};
use std::time::Instant;
use rayon::prelude::*;


fn main() 
{
    let filepath = "input.txt";
    let start = Instant::now();
    part1_attempt1(filepath, 2000000);
    println!("Attempt 1 took {} ms", (Instant::now() - start).as_millis());
    
    let start = Instant::now();
    part1_attempt2(filepath, 2000000);
    println!("Attempt 2 took {} us", (Instant::now() - start).as_micros());
    
    let start = Instant::now();
    part2(filepath, (0, 4_000_000));
    println!("Part 2 took {} s", (Instant::now() - start).as_secs());

    let start = Instant::now();
    par_part2(filepath, (0, 4_000_000));
    println!("Part 2 took {} s", (Instant::now() - start).as_secs());
}

fn part1_attempt1(filepath: &str, row: i32)
{
    let contents = read_to_string(filepath)
        .expect(format!("Failed to read {}.", filepath).as_str());
    
    let re = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)")
        .expect("Failed to compile regex expression.");
    
    let sensor_beacons = contents
        .lines()
        .map(|line| 
            {
                let cap = re.captures(line).expect("Failed to match against regex.");
                // println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
                ((cap[1].parse::<i32>().expect(format!("Failed to parse {}", &cap[1]).as_str()), 
                  cap[2].parse::<i32>().expect(format!("Failed to parse {}", &cap[2]).as_str())),
                 (cap[3].parse::<i32>().expect(format!("Failed to parse {}", &cap[3]).as_str()), 
                  cap[4].parse::<i32>().expect(format!("Failed to parse {}", &cap[4]).as_str())))
            }
        );
    let sensor_distances = sensor_beacons.clone()
        .map(|(sensor, beacon)| (sensor, sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)));
    
    let existing_beacons = sensor_beacons.filter(|(_,beacon)| beacon.1 == row).map(|(_, beacon)|beacon.0).collect::<HashSet<i32>>();
    
    let mut row_cannot_contain = HashSet::new();
    for (sensor, distance) in sensor_distances
    {
        let width = distance as i32 - sensor.1.abs_diff(row) as i32;
        if width <= 0
        {
            continue;
        }
        
        let start = sensor.0 - width;
        for x in start..start + width * 2 + 1
        {
            if existing_beacons.contains(&x) == false
            {
                row_cannot_contain.insert(x);   
            }
        }
    }
    println!("Part 1 {}", row_cannot_contain.len());
}

#[derive(Debug, Copy, Clone)]
struct Section
{
    min: i32,
    max: i32
}

impl Section
{
    fn contains(&self, other: &Section) -> bool
    {
        self.min >= other.min && self.max <= other.max
    }

    fn contains_i32(&self, other: i32) -> bool
    {
        self.min <= other && self.max >= other
    }

    fn overlaps(&self, other: &Section) -> bool
    {
        self.min <= other.max && self.max >= other.min
    }
    
    fn try_merge(a: &Section, b: &Section) -> Option<Section>
    {
       if Section::overlaps(a, b)
       {
           return Some(Section{min: min(a.min, b.min), max: max(a.max, b.max)});
       }
        return None;
    }
    
    fn len(&self) -> i32
    {
        self.max - self.min
    }
}

fn part1_attempt2(filepath: &str, row: i32)
{
    let contents = read_to_string(filepath)
        .expect(format!("Failed to read {}.", filepath).as_str());

    let re = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)")
        .expect("Failed to compile regex expression.");

    let sensor_beacons = contents
        .lines()
        .map(|line|
            {
                let cap = re.captures(line).expect("Failed to match against regex.");
                // println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
                ((cap[1].parse::<i32>().expect(format!("Failed to parse {}", &cap[1]).as_str()),
                  cap[2].parse::<i32>().expect(format!("Failed to parse {}", &cap[2]).as_str())),
                 (cap[3].parse::<i32>().expect(format!("Failed to parse {}", &cap[3]).as_str()),
                  cap[4].parse::<i32>().expect(format!("Failed to parse {}", &cap[4]).as_str())))
            }
        );
    let sensor_distances = sensor_beacons.clone()
        .map(|(sensor, beacon)| (sensor, sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)));
    
    let existing_beacons = sensor_beacons.filter(|(_,beacon)| beacon.1 == row).map(|(_, beacon)| beacon.0).collect::<HashSet<i32>>();
    
    let mut ranges = sensor_distances
        .map(|(sensor, distance)| 
            (sensor.0, distance as i32 - sensor.1.abs_diff(row) as i32))
        .filter(|(_, width)| *width > 0)
        .map(|(x,width)| Section{min:x - width, max:x + width}).collect::<Vec<Section>>();
    
    ranges.sort_by(|a,b| a.min.cmp(&b.min));
    
    let mut folded = vec![ranges[0]];
    let mut j = 0;
    for i in 1..ranges.len()
    {
        match Section::try_merge(&folded[j], &ranges[i]) {
            None => {
                folded.push(folded[j]);
                j += 1;
            }
            Some(new) => 
                {
                    folded[j] = new;
                }
        }
    }
    
    let mut count = 0;
    for sect in folded
    {
        let subtract_beacons_in_row = existing_beacons.iter().fold(0, |sum, x| sum + sect.contains_i32(*x) as i32);
        count += sect.max + 1 - sect.min - subtract_beacons_in_row;
    }
    
    println!("Part 1 {}", count);
}

fn part2(filepath: &str, bounds: (i32,i32))
{
    let contents = read_to_string(filepath)
        .expect(format!("Failed to read {}.", filepath).as_str());

    let re = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)")
        .expect("Failed to compile regex expression.");

    let sensor_beacons = contents
        .lines()
        .map(|line|
            {
                let cap = re.captures(line).expect("Failed to match against regex.");
                // println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
                ((cap[1].parse::<i32>().expect(format!("Failed to parse {}", &cap[1]).as_str()),
                  cap[2].parse::<i32>().expect(format!("Failed to parse {}", &cap[2]).as_str())),
                 (cap[3].parse::<i32>().expect(format!("Failed to parse {}", &cap[3]).as_str()),
                  cap[4].parse::<i32>().expect(format!("Failed to parse {}", &cap[4]).as_str())))
            }
        );
    let sensor_distances = sensor_beacons.clone()
        .map(|(sensor, beacon)| (sensor, sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)));

    // let existing_beacons = sensor_beacons.filter(|(_,beacon)| beacon.1 == row).map(|(_, beacon)| beacon.0).collect::<HashSet<i32>>();
    
    for row in bounds.0..bounds.1
    {
        let mut ranges = sensor_distances.clone()
            .map(|(sensor, distance)|
                (sensor.0, distance as i32 - sensor.1.abs_diff(row) as i32))
            .filter(|(_, width)| *width > 0)
            .map(|(x,width)| Section{min:x - width, max:x + width}).collect::<Vec<Section>>();

        ranges.sort_by(|a,b| a.min.cmp(&b.min));

        let mut folded = vec![ranges[0]];
        let mut j = 0;
        for i in 1..ranges.len()
        {
            match Section::try_merge(&folded[j], &ranges[i]) {
                None => {
                    folded.push(folded[j]);
                    j += 1;
                }
                Some(new) =>
                    {
                        folded[j] = new;
                    }
            }
        }

        if folded.len() > 1
        {
            println!("Part 2 {}", (folded[0].max + 1) as u64 * bounds.1 as u64 + row as u64);
            break;
        }
    }
    
}

fn par_part2(filepath: &str, bounds: (i32,i32))
{
    let contents = read_to_string(filepath)
        .expect(format!("Failed to read {}.", filepath).as_str());

    let re = Regex::new(r"Sensor at x=([\d-]+), y=([\d-]+): closest beacon is at x=([\d-]+), y=([\d-]+)")
        .expect("Failed to compile regex expression.");

    let sensor_beacons = contents
        .lines()
        .map(|line|
            {
                let cap = re.captures(line).expect("Failed to match against regex.");
                // println!("{} {} {} {}", &cap[1], &cap[2], &cap[3], &cap[4]);
                ((cap[1].parse::<i32>().expect(format!("Failed to parse {}", &cap[1]).as_str()),
                  cap[2].parse::<i32>().expect(format!("Failed to parse {}", &cap[2]).as_str())),
                 (cap[3].parse::<i32>().expect(format!("Failed to parse {}", &cap[3]).as_str()),
                  cap[4].parse::<i32>().expect(format!("Failed to parse {}", &cap[4]).as_str())))
            }
        );
    let sensor_distances = sensor_beacons.clone()
        .map(|(sensor, beacon)| (sensor, sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1)));

    // let existing_beacons = sensor_beacons.filter(|(_,beacon)| beacon.1 == row).map(|(_, beacon)| beacon.0).collect::<HashSet<i32>>();

    let (row, ranges) = (bounds.0..bounds.1)
        .into_par_iter()
        .map(|row|
            {
                let mut ranges = sensor_distances.clone()
                    .map(|(sensor, distance)|
                        (sensor.0, distance as i32 - sensor.1.abs_diff(row) as i32))
                    .filter(|(_, width)| *width > 0)
                    .map(|(x,width)| Section{min:x - width, max:x + width}).collect::<Vec<Section>>();

                ranges.sort_by(|a,b| a.min.cmp(&b.min));

                let mut folded = vec![ranges[0]];
                let mut j = 0;
                for i in 1..ranges.len()
                {
                    match Section::try_merge(&folded[j], &ranges[i]) {
                        None => {
                            folded.push(folded[j]);
                            j += 1;
                        }
                        Some(new) =>
                            {
                                folded[j] = new;
                            }
                    }
                }
                (row, folded)
                // if folded.len() > 1
                // {
                //     println!("Part 2 {}", (folded[0].max + 1) as u64 * bounds.1 as u64 + row as u64);
                //     break;
                // }  
            }).find_any(|(_, folded)| folded.len() > 1).expect("Failed to find a solution");
    
    println!("{}", (ranges[0].max + 1) as u64 * bounds.1 as u64 + row as u64);
}
