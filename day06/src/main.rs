#![allow(dead_code)]

use std::time::Instant;
use rayon::prelude::*;


fn main() {
    let filepath = "input.txt";
    let signal = std::fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str());

    println!("Using Skips + For Loop");
    let start = Instant::now();
    let part1 = find_start_of_marker(signal.as_str(), MarkerType::PACKET);
    println!("Part 1 = {}, calculated in {} us", part1, (Instant::now() - start).as_micros());
    let start = Instant::now();
    let part2 = find_start_of_marker(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 2 = {}, calculated in {} us", part2, (Instant::now() - start).as_micros());
    println!();
    
    println!("Using iterators");
    let start = Instant::now();
    let part1 = iter_find_start_of_marker(signal.as_str(), MarkerType::PACKET);
    println!("Part 1 = {}, calculated in {} us", part1, (Instant::now() - start).as_micros());
    let start = Instant::now();
    let part2 = iter_find_start_of_marker(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 2 = {}, calculated in {} us", part2, (Instant::now() - start).as_micros());
    println!();
    
    println!("Using parallel iterators in rayon");
    let start = Instant::now();
    let part1 = par_find_start_of_marker(signal.as_str(), MarkerType::PACKET);
    println!("Part 1 = {}, calculated in {} us", part1, (Instant::now() - start).as_micros());
    let start = Instant::now();
    let part2 = par_find_start_of_marker(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 2 = {}, calculated in {} us", part2, (Instant::now() - start).as_micros());
    println!();

    println!("Using Skips + For Loop");
    let start = Instant::now();
    let part1 = find_start_of_marker_with_skip(signal.as_str(), MarkerType::PACKET);
    println!("Part 1 = {}, calculated in {} us", part1, (Instant::now() - start).as_micros());
    let start = Instant::now();
    let part2 = find_start_of_marker_with_skip(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 2 = {}, calculated in {} us", part2, (Instant::now() - start).as_micros());
    println!();
    
    println!("Using skip + bitmask");
    let start = Instant::now();
    let part1 = find_start_of_marker_optimum(signal.as_str(), MarkerType::PACKET);
    println!("Part 1 = {}, calculated in {} us", part1, (Instant::now() - start).as_micros());
    let start = Instant::now();
    let part2 = find_start_of_marker_optimum(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 2 = {}, calculated in {} us", part2, (Instant::now() - start).as_micros());
}

#[derive(Clone, Copy)]
enum MarkerType
{
    PACKET = 4,
    MESSAGE = 14
}

fn find_start_of_marker(signal: &str, _type: MarkerType) -> u32
{
    let unique_set_len = _type as u32;
    
    assert!(signal.len() >= unique_set_len as usize, "Input signal is expected to be greater than {} characters long.", unique_set_len);
    let chars = signal.chars().collect::<Vec<char>>();
    
    for i in unique_set_len as usize..signal.len()
    {
        let mut bitmask = 0;
        for j in 0..unique_set_len as usize
        {
            bitmask |= 1 << (chars[i - j] as u32 - 'a' as u32);
        }

        // check bitmask
        let mut count = 0;
        for j in 0..26
        {
            count += ((bitmask & 1 << j) > 0) as u32
        }
        if count == unique_set_len
        {
            return (i + 1) as u32;
        }
    }
    return 0;
}


fn iter_find_start_of_marker(signal: &str, _type: MarkerType) -> u32
{
    let unique_set_len = _type as u32;
    
    let chars = signal.chars().collect::<Vec<char>>();
    let mut iter = signal.char_indices();
    iter.nth(unique_set_len as usize).expect(format!("Input signal is expected to be greater than {} characters long.", unique_set_len).as_str());
    let found_thing = iter.find(|(i, _)|  {
        let mut bitmask = 0;
        for j in 0..unique_set_len as usize
        {
            bitmask |= 1 << (chars[*i as usize - j as usize] as u32 - 'a' as u32);
        }

        // check bitmask
        let mut count = 0;
        for j in 0..26
        {
            count += ((bitmask & 1 << j) > 0) as u32
        }
        if count == unique_set_len
        {
            return true
        }
        return false;
    });

    let (idx, _) = found_thing.expect("Failed to find an answer!");
    return idx as u32 + 1;
}


fn par_find_start_of_marker(signal: &str, _type: MarkerType) -> u32
{
    let unique_set_len = _type as u32;
    assert!(signal.len() >= unique_set_len as usize, "Input signal is expected to be greater than {} characters long.", unique_set_len);
    
    let chars = signal.chars().collect::<Vec<char>>();
    let found_thing = signal.par_char_indices().find_first(|(i, _)|  {
       if *i < unique_set_len as usize
       {
           return false;
       }
        
        let mut bitmask = 0;
        for j in 0..unique_set_len as usize
        {
            bitmask |= 1 << (chars[*i as usize - j as usize] as u32 - 'a' as u32);
        }

        // check bitmask
        let mut count = 0;
        for j in 0..26
        {
            count += ((bitmask & 1 << j) > 0) as u32
        }
        if count == unique_set_len
        {
            return true
        }
        return false;
    });
    
    let (idx, _) = found_thing.expect("Failed to find an answer!");
    return idx as u32 + 1;
}

fn find_start_of_marker_with_skip(signal: &str, _type: MarkerType) -> u32
{
    let unique_set_len = _type as u32;
    assert!(signal.len() >= unique_set_len as usize, "Input signal is expected to be greater than {} characters long.", unique_set_len);
    assert!(unique_set_len < 256, "Marker size can't be longer than 256 as we store our indices using a u8");
    
    let mut map_char_to_index = [0u8; 26];
    let chars = signal.chars().collect::<Vec<char>>();
    
    let mut look_back = unique_set_len as usize;
    let mut idx = unique_set_len as usize - 1;
    'outer :while idx < chars.len()
    {
        for j in (0..look_back).rev()
        {
            let char_index = chars[idx - j] as usize - 'a' as usize;
            if map_char_to_index[char_index] > 0
            {
                let skip_by= map_char_to_index[char_index];
                map_char_to_index[char_index] = (unique_set_len - j as u32) as u8;
                idx += skip_by as usize;
                look_back = skip_by as usize + j;
                
                map_char_to_index.iter_mut().for_each(|x| *x = std::cmp::max(*x as i16 - (skip_by as i16), 0) as u8);
                continue 'outer;
            }
            else
            {
                map_char_to_index[char_index] = (unique_set_len - j as u32) as u8;
            }
        }
        return idx as u32 + 1;
    }
    
    return 0;
}

fn find_start_of_marker_optimum(signal: &str, _type: MarkerType) -> u32
{
    let unique_set_len = _type as u32;
    assert!(signal.len() >= unique_set_len as usize, "Input signal is expected to be greater than {} characters long.", unique_set_len);
    assert!(unique_set_len < 256, "Marker size can't be longer than 256 as we store our indices using a u8");

    let chars = signal.chars().collect::<Vec<char>>();
    let mut idx = unique_set_len as usize - 1;
    'outer :while idx < chars.len()
    {
        let mut bitmask = 0;
        for j in (0..unique_set_len as usize).rev()
        {
            let mask = 1u32 << (chars[idx - j] as u32 - 'a' as u32);
            if (bitmask & mask) > 0
            {
                idx += j + 1;
                continue 'outer;
            }
            else
            {
                bitmask |= mask;
            }
        }
        return idx as u32 + 1;
    }

    return 0;
}

#[test]
fn part1_test_signal0()
{
    let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let result = find_start_of_marker(signal, MarkerType::PACKET);
    assert_eq!(result, 7);
}

#[test]
fn part1_test_signal1()
{
    let signal = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let result = find_start_of_marker(signal, MarkerType::PACKET);
    assert_eq!(result, 5);
}

#[test]
fn part1_test_signal2()
{
    let signal = "nppdvjthqldpwncqszvftbrmjlhg";
    let result = find_start_of_marker(signal, MarkerType::PACKET);
    assert_eq!(result, 6);
}

#[test]
fn part1_test_signal3()
{
    let signal = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let result = find_start_of_marker(signal, MarkerType::PACKET);
    assert_eq!(result, 10);
}

#[test]
fn part1_test_signal4()
{
    let signal = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let result = find_start_of_marker(signal, MarkerType::PACKET);
    assert_eq!(result, 11);
}

#[test]
fn part2_test_signal0()
{
    let signal = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let result = find_start_of_marker(signal, MarkerType::MESSAGE);
    assert_eq!(result, 19);
}

#[test]
fn part2_test_signal1()
{
    let signal = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    let result = find_start_of_marker(signal, MarkerType::MESSAGE);
    assert_eq!(result, 23);
}

#[test]
fn part2_test_signal2()
{
    let signal = "nppdvjthqldpwncqszvftbrmjlhg";
    let result = find_start_of_marker(signal, MarkerType::MESSAGE);
    assert_eq!(result, 23);
}

#[test]
fn part2_test_signal3()
{
    let signal = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let result = find_start_of_marker(signal, MarkerType::MESSAGE);
    assert_eq!(result, 29);
}

#[test]
fn part2_test_signal4()
{
    let signal = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
    let result = find_start_of_marker(signal, MarkerType::MESSAGE);
    assert_eq!(result, 26);
}
