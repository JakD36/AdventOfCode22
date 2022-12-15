#![allow(dead_code)]

use std::collections::VecDeque;
use std::fs;

fn main() 
{
    let filepath = "input.txt";
    part1(filepath);
    part2(filepath);
}

fn build_shortest_path(map: &Vec<i8>, width: i32, height: i32, search_from: usize) -> Vec<i32>
{
    let mut steps_to = vec![i32::MAX-1; map.len()];
    steps_to[search_from] = 0;
    let mut next_steps = VecDeque::new();
    next_steps.push_back(search_from);

    let matrix: [(i32, i32); 4] = [(0, 1), (-1, 0), (1, 0), (0, -1)];

    while let Some(next) = next_steps.pop_front()
    {
        let (from_x,from_y) = to_coords(next, width);
        let indices = matrix.iter()
            .map(|(offset_x, offset_y)| (from_x + offset_x, from_y + offset_y))
            .filter(|(x,y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
            .map(|(x, y)| to_idx(x, y, width))
            .filter(|idx| (map[*idx] - map[next]) > -2);

        indices.into_iter().for_each(|v|
            {
                if steps_to[next] + 1 < steps_to[v]
                {
                    next_steps.push_back(v);
                    steps_to[v] = steps_to[next] + 1;
                }
            });
    }
    return steps_to;
}

fn part1(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let width = contents.chars().position(|c| c == '\n').expect("Failed to determine the width of the map") as i32;
    let map = contents.lines().flat_map(|line| line.chars().map(|c| c as i8 - 'a' as i8)).collect::<Vec<i8>>();

    let height = map.len() as i32 / width;

    let start = map.iter().position(|c| *c == 'S' as i8 - 'a' as i8).expect("Failed to find the start");
    let end = map.iter().position(|c| *c == 'E' as i8 - 'a' as i8).expect("Failed to find the end");
    let map = map.iter().map(|x|
        {
            match *x
            {
                -14 => 0, // S
                -28 => 26, // E
                _ => *x
            }
        }).collect::<Vec<i8>>();
    // println!("Starts at {}, ends at {} and the map has a width of {}",start, end, width);
    
    let steps_to = build_shortest_path(&map, width, height, end);
    
    println!("Part 1 {}", steps_to[start]);
}

fn part2(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let width = contents.chars().position(|c| c == '\n').expect("Failed to determine the width of the map") as i32;
    let map = contents.lines().flat_map(|line| line.chars().map(|c| c as i8 - 'a' as i8)).collect::<Vec<i8>>();
    let height = map.len() as i32 / width;
    
    let end = map.iter().position(|c| *c == 'E' as i8 - 'a' as i8).expect("Failed to find the end");
    let map = map.iter().map(|x|
        {
            match *x
            {
                -14 => 0, // S
                -28 => 26, // E
                _ => *x
            }
        }).collect::<Vec<i8>>();
    // println!("Starts at {}, ends at {} and the map has a width of {}",start, end, width);

    let steps_to = build_shortest_path(&map, width, height, end);

    let (index, _) = map.iter()
        .enumerate()
        .filter(|(_, height)| **height == 0)
        .min_by(|x, y| steps_to[x.0].cmp(&steps_to[y.0])).expect("Failed to find the minimum");
    
    println!("Part 2 {}", steps_to[index]);
}

fn to_idx(x: i32, y: i32, width: i32) -> usize
{
    (y * width + x) as usize
}

fn to_coords(idx: usize, width: i32) -> (i32, i32)
{
    (idx as i32 % width, idx as i32 / width)
}
