#![allow(dead_code)]
use std::fs;
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

fn build_visibility_map(rows: &Vec<Vec<i32>>) -> Vec<u128>
{
    let mut visibility_map: Vec<u128> = vec![0; rows.len()];
    let mut column_maxes = vec![-1; rows.len()];
    for i in 0..rows.len()
    {
        let mut row_max = -1;
        for j in 0..rows[i].len()
        {
            if rows[i][j] > row_max
            {
                row_max = rows[i][j];
                visibility_map[i] |= 1 << j
            }

            if rows[i][j] > column_maxes[j]
            {
                column_maxes[j] = rows[i][j];
                visibility_map[i] |= 1 << j
            }
        }
    }

    column_maxes.fill(-1);
    for i in (0..rows.len()).rev()
    {
        let mut row_max = -1;
        for j in (0..rows[i].len()).rev()
        {
            if rows[i][j] > row_max
            {
                row_max = rows[i][j];
                visibility_map[i] |= 1 << j
            }

            if rows[i][j] > column_maxes[j]
            {
                column_maxes[j] = rows[i][j];
                visibility_map[i] |= 1 << j
            }
        }
    }
    return visibility_map;
}

fn part1(filepath: &str)
{
    let rows = fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str())
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).expect(format!("{} is not a number!", c).as_str()) as i32)
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    
    let visibility_map = build_visibility_map(&rows);
    
    let mut count = 0;
    for val in visibility_map 
    {
        for i in 0..128
        {
            count += (val & (1 << i) > 0) as u32;
        }
    }
    
    println!("Part 1 {}", count);
}

fn part2(filepath: &str)
{
    let rows = fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str())
        .lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).expect(format!("{} is not a number!", c).as_str()) as i32)
            .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let visibility_map = build_visibility_map(&rows);

    let mut max_scenic_score = 0;
    for i in 0..visibility_map.len()
    {
        for j in 0..rows[i].len()
        {
            if visibility_map[i] & (1 << j) > 0 // Comment out to check all trees 
            {
                let score = calculate_scenic_score(&rows, i, j);
                max_scenic_score = std::cmp::max(max_scenic_score, score);
            }
        }
    }

    println!("Part 2 {}", max_scenic_score);
}


fn calculate_scenic_score(rows: &Vec<Vec<i32>>, i: usize, j: usize) -> i32
{
    let height = rows[i][j];
    
    let mut left = 0;
    for k in (0..j).rev()
    {
        left += 1;
        if rows[i][k] >= height
        {
            break;
        }
    }
    
    let mut right = 0;
    for k in j+1..rows[i].len()
    {
        right += 1;
        if rows[i][k] >= height
        {
            break;
        }
    }

    let mut above = 0;
    for k in (0..i).rev()
    {
        above += 1;
        if rows[k][j] >= height
        {
            break;
        }
    }

    let mut below = 0;
    for k in i+1..rows.len()
    {
        below += 1;
        if rows[k][j] >= height
        {
            break;
        }
    }
    
    return left * right * above * below;
}