#![allow(dead_code)]

use std::collections::{HashSet};
use std::fs;
use std::cmp::max;

fn main() 
{
    let filepath = "input.txt";
    part1(filepath);
    part2(filepath);
}

fn part1(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let instructions = contents
        .lines()
        .map(|line| line.split_once(" ").expect(format!("Failed to split the line {}", line).as_str()))
        .map(|(a, b)| (a, b.parse::<i32>().expect(format!("{} is not a number",b).as_str())));//.collect::<Vec<(&str, i32)>>();

    let mut visited : HashSet<(i32, i32)> = HashSet::new();
    
    let mut head_pos: (i32, i32) = (0, 0);
    let mut tail_pos: (i32, i32) = (0, 0);
    for (dir, count) in instructions
    {
        match dir
        {
            "R" => {
                for _ in 0..count {
                    head_pos.0 += 1;
                    tail_pos = simulate(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            },
            "L" => {
                for _ in 0..count {
                    head_pos.0 -= 1;
                    tail_pos = simulate(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            },
            "U" => {
                for _ in 0..count {
                    head_pos.1 += 1;
                    tail_pos = simulate(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            },
            "D" => {
                for _ in 0..count {
                    head_pos.1 -= 1;
                    tail_pos = simulate(head_pos, tail_pos);
                    visited.insert(tail_pos);
                }
            },
            _ => ()
        }
    }
    
    println!("Part 1 {}",visited.iter().count());
}

fn simulate(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32)
{
    let x = head_pos.0 - tail_pos.0;
    let y = head_pos.1 - tail_pos.1;
    
    let next_pos = if (x.abs() > 1 && y.abs() >= 1) || (x.abs() >= 1 && y.abs() > 1)
    {
        (tail_pos.0 + x.signum() * max(x.abs() - 1, 1), tail_pos.1 + y.signum() * max(y.abs() - 1, 1)) 
    }
    else if x.abs() > 1
    {
        (tail_pos.0 + x.signum() * max(x.abs() - 1, 0), tail_pos.1)
    }
    else if y.abs() > 1
    {
        (tail_pos.0, tail_pos.1 + y.signum() * max(y.abs() - 1, 0))
    }
    else {
        tail_pos
    };

    assert!((head_pos.0 - next_pos.0) < 2 && (head_pos.1 - next_pos.1) < 2);
    return next_pos;
}

fn part2(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let instructions = contents
        .lines()
        .map(|line| line.split_once(" ").expect(format!("Failed to split the line {}", line).as_str()))
        .map(|(a, b)| (a, b.parse::<i32>().expect(format!("{} is not a number", b).as_str())));//.collect::<Vec<(&str, i32)>>();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut positions: [(i32, i32); 10] = [(0, 0); 10];
    for (dir, count) in instructions
    {
        match dir
        {
            "R" => {
                for _ in 0..count {
                    positions[0].0 += 1;
                    for i in 1..10 {
                        positions[i] = simulate(positions[i - 1], positions[i]);
                    }
                    visited.insert(positions[9]);
                }
            },
            "L" => {
                for _ in 0..count {
                    positions[0].0 -= 1;
                    for i in 1..10 {
                        positions[i] = simulate(positions[i - 1], positions[i]);
                    }
                    visited.insert(positions[9]);
                }
            },
            "U" => {
                for _ in 0..count {
                    positions[0].1 += 1;
                    for i in 1..10 {
                        positions[i] = simulate(positions[i - 1], positions[i]);
                    }
                    visited.insert(positions[9]);
                }
            },
            "D" => {
                for _ in 0..count {
                    positions[0].1 -= 1;
                    for i in 1..10 {
                        positions[i] = simulate(positions[i - 1], positions[i]);
                    }
                    visited.insert(positions[9]);
                }
            },
            _ => ()
        }
    }

    println!("Part 2 {}", visited.iter().count());
}