use std::collections::HashSet;
use std::fs::read_to_string;
use std::time::Instant;

fn main()
{
    let filepath = "input.txt";
    let start = Instant::now();
    part1(filepath);
    println!("Part 1 took {} us", (Instant::now() - start).as_micros());
    let start = Instant::now();
    part2(filepath);
    println!("Part 2 took {} us", (Instant::now() - start).as_micros());
}

fn build_structure(filepath: &str) -> HashSet<(i32, i32)>
{
    let contents = read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let lines = contents
        .lines()
        .map(|line|
            line.split(" -> ")
                .map(|pair|
                    {
                        let (a,b) = pair.split_once(",").expect(format!("Failed to split {} by comma", pair).as_str());
                        (a.parse::<i32>().expect("Failed to parse"), b.parse::<i32>().expect("Failed to parse"))
                    }));

    let mut structure = HashSet::new();
    for line in lines
    {
        let line = line.collect::<Vec<(i32,i32)>>();
        for i in 1..line.len()
        {
            if line[i-1].0 == line[i].0
            {
                for j in if line[i-1].1 < line[i].1 {line[i-1].1..line[i].1 + 1} else {line[i].1..line[i-1].1 + 1}
                {
                    structure.insert((line[i-1].0, j));
                }
            }
            else
            {
                for j in if line[i-1].0 < line[i].0 {line[i-1].0..line[i].0 + 1} else {line[i].0..line[i-1].0 + 1}
                {
                    structure.insert((j,line[i-1].1));
                }
            }
        }

    }
    return structure;
}

fn part1(filepath: &str)
{
    let mut structure = build_structure(filepath);
    
    let lowest_point = structure.iter().max_by(|a, b| a.1.cmp(&b.1)).expect("Failed to find the deepest part of the structure!").1;
    
    let mut stack = vec![];
    let mut count = 0; 
    
    let start = (500, 0);
    let mut current = start;
    loop 
    {
        let next = (current.0, current.1 + 1);
        if next.1 > lowest_point
        {
            break;
        }
        
        if structure.contains(&next)
        {
            let left = (next.0 - 1, next.1); 
            if structure.contains(&left)
            {
                let right = (next.0 + 1, next.1);
                if structure.contains(&right)
                {
                    structure.insert(current);
                    current = stack.pop().unwrap_or(start);
                    count += 1;
                }
                else 
                { 
                    stack.push(current);
                    current = right;    
                }
            }
            else 
            {
                stack.push(current);
                current = left;
            }
        }
        else
        {
            stack.push(current);
            current = next;
        }
    }
    
    println!("Part 1: {}", count);
}

fn part2(filepath: &str)
{
    let mut structure = build_structure(filepath);
    let floor = structure.iter().max_by(|a, b| a.1.cmp(&b.1)).expect("Failed to find the deepest part of the structure!").1 + 2;

    let mut count = 0;
    let mut stack = vec![];

    let start = (500, 0);
    let mut current = start;
    loop
    {
        let next = (current.0, current.1 + 1);
        if next.1 >= floor
        {
            structure.insert(current);
            current = stack.pop().unwrap_or(start);
            count += 1;
            continue;
        }
        
        if structure.contains(&next)
        {
            let left = (next.0 - 1, next.1);
            if structure.contains(&left)
            {
                let right = (next.0 + 1, next.1);
                if structure.contains(&right)
                {
                    if structure.insert(current) == false
                    {
                        break;
                    }
                    current = stack.pop().unwrap_or(start);
                    count += 1;
                }
                else
                {
                    stack.push(current);
                    current = right;
                }
            }
            else
            {
                stack.push(current);
                current = left;
            }
        }
        else
        {
            stack.push(current);
            current = next;
        }
    }

    println!("Part 2: {}", count);
}