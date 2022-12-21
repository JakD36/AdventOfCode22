use std::collections::HashMap;
use std::fs::read_to_string;
use std::mem::size_of;
use std::time::Instant;
use regex::Regex;

fn main() {
    let filepath = "input.txt";
    
    let (arr, len, aa_idx) = build_input(filepath);
    
    let start = Instant::now();
    let max = calculate_max(aa_idx, &arr, len, 1 << aa_idx, 30, 0);
    println!("Part 1 {} and took {} ms", max, (Instant::now() - start).as_millis());
    let start = Instant::now();
    let max = calculate_max_2(Person{idx: aa_idx, time_remaining: 0}, Person{idx: aa_idx, time_remaining: 0}, &arr, len, 1 << aa_idx, 26, 0);
    println!("Part 2 {} and took {} ms", max, (Instant::now() - start).as_millis());
}

fn calculate_max(current: usize, valves: &[Valve;16], len: usize, visited: u16, time_remaining: i8, score: u32) -> u32
{
    let mut max = score;
    for i in 0..len
    {
        let remaining = time_remaining - (valves[current].distance_to[i] + 1) as i8;
        if i == current || ((1 << i) & visited) > 0 || remaining <= 0
        {
            continue;
        }
        
        max = std::cmp::max(max, calculate_max(i, valves, len, visited | 1 << i as u16, remaining, score + remaining as u32 * valves[i].flowrate as u32));
    }
    return max;
}

#[derive(Clone, Copy)]
struct Person
{
    idx: usize,
    time_remaining: u8
}

#[derive(Clone, Copy)]
struct Valve
{
    flowrate: u8,
    distance_to: [u8;16]
}

fn calculate_max_2(person1: Person, person2: Person, valves: &[Valve;16], len: usize, visited: u16, time_remaining: i8, score: u32) -> u32
{
    let mut max = score;
    if person1.time_remaining == 0 && person2.time_remaining == 0
    {
        for i in 0..len
        {
            let remaining = time_remaining - (valves[person1.idx].distance_to[i] + 1) as i8;
            if i == person1.idx || i == person2.idx || ((1 << i) & visited) > 0 || remaining <= 0
            {
                continue;
            }
            let person1 = Person{idx:i, time_remaining: valves[person1.idx].distance_to[i] + 1}; // Time remaining before person 1 can take next action
            let score = score + remaining as u32 * valves[i].flowrate as u32;
            for j in 0..len
            {
                let remaining = time_remaining - (valves[person2.idx].distance_to[j] + 1) as i8;
                if j == person1.idx || j == person2.idx || ((1 << j) & visited) > 0 || remaining <= 0
                {
                    continue;
                }
                
                let person2 = Person{idx:j, time_remaining: valves[person2.idx].distance_to[j] + 1}; // Time remaining before person 1 can take next action
                let score = score + remaining as u32 * valves[j].flowrate as u32;
                
                let time_to_next_action = std::cmp::min(person1.time_remaining, person2.time_remaining);
                let person1 = Person{idx:person1.idx, time_remaining: person1.time_remaining - time_to_next_action};
                let person2 = Person{idx:person2.idx, time_remaining: person2.time_remaining - time_to_next_action};
                
                max = std::cmp::max(max, calculate_max_2(person1, person2, valves, len, visited | 1 << person1.idx as u16 | 1 << person2.idx as u16, time_remaining - time_to_next_action as i8, score));        
            }
        }
        
    }
    else if person1.time_remaining == 0
    {
        for i in 0..len
        {
            let mut person1 = person1;
            let mut score = score;
            let remaining = time_remaining - (valves[person1.idx].distance_to[i] + 1) as i8;
            if i == person1.idx || i == person2.idx || ((1 << i) & visited) > 0 || remaining <= 0
            {
                continue;
            }
            person1 = Person{idx: i, time_remaining: valves[person1.idx].distance_to[i] + 1};
            score = score + remaining as u32 * valves[i].flowrate as u32;

            let time_to_next_action = std::cmp::min(person1.time_remaining, person2.time_remaining);
            let person1 = Person{idx:person1.idx, time_remaining: person1.time_remaining - time_to_next_action};
            let person2 = Person{idx:person2.idx, time_remaining: person2.time_remaining - time_to_next_action};

            max = std::cmp::max(max, calculate_max_2(person1, person2, valves, len, visited | 1 << person1.idx as u16, time_remaining - time_to_next_action as i8, score));
        }
    }
    else if person2.time_remaining == 0
    {
        for i in 0..len
        {
            let mut person2 = person2;
            let mut score = score;
            let remaining = time_remaining - (valves[person2.idx].distance_to[i] + 1) as i8;
            if i == person1.idx || i == person2.idx || ((1 << i) & visited) > 0 || remaining <= 0
            {
                continue;
            }
            person2 = Person{idx:i, time_remaining: valves[person2.idx].distance_to[i] + 1};
            score = score + remaining as u32 * valves[i].flowrate as u32;

            let time_to_next_action = std::cmp::min(person1.time_remaining, person2.time_remaining);
            let person1 = Person{idx:person1.idx, time_remaining: person1.time_remaining - time_to_next_action};
            let person2 = Person{idx:person2.idx, time_remaining: person2.time_remaining - time_to_next_action};

            max = std::cmp::max(max, calculate_max_2(person1, person2, valves, len, visited | 1 << person2.idx as u16, time_remaining - time_to_next_action as i8, score));
        }
    }
    return max;
}

fn build_input(filepath: &str) -> ([Valve;16], usize, usize) 
{
    let contents = read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    
    let start = Instant::now();
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves?")
        .expect("Failed to compile regex");
    let repeating_re = Regex::new(r"[A-Z]{2}")
        .expect("Failed to compile regex");
    println!("Compiling Regex takes {} us", (Instant::now() - start).as_micros());
    
    let start = Instant::now();
    let input = contents.lines().map(|line|
        {
            let c = re.captures(line).unwrap();
            let name = c.get(1).unwrap().as_str();
            let flowrate = c[2].parse::<u8>().expect("Failed to parse flowrate.");
            let leads_to = repeating_re.captures_iter(&line[*&c[0].len() + 1..]).map(|cap| cap.get(0).unwrap().as_str()).collect::<Vec<&str>>();
            (name, flowrate, leads_to)
        });
    
    let map = input.clone()
        .map(|(name, _, leads_to)| (String::from(name), leads_to))
        .collect::<HashMap<String, Vec<&str>>>();
    
    let valve_names = input.clone()
        .filter(|(name, flowrate, _)| *flowrate > 0 || name == &"AA")
        .map(|(name, _, _)| name);

    let remove_entries = input.clone()
        .filter(|(name, flowrate, _)| *flowrate <= 0 && name != &"AA")
        .map(|(name, _, _)| name);
    
    // valve_names.clone().for_each(|name| println!("{}", name));
    let vec = valve_names.clone()
        .map(|name|
        {
            let mut output = HashMap::new();
            build_distance_graph_from(name, 0, &map, &mut output);
            for x in remove_entries.clone()
            {
                output.remove(x);
            }
            output
        })
        .collect::<Vec<HashMap<String, u8>>>();
    println!("Building Hashmap graph takes {} ms", (Instant::now() - start).as_millis());
    
    let start = Instant::now();
    let mut arr: [Valve;16] = [Valve{flowrate: 0, distance_to: [0;16]};16];
    for (name, flowrate, _) in input.filter(|(_, y,_)| *y > 0)
    {
        arr[valve_names.clone().position(|x|name == x).unwrap()].flowrate = flowrate;
    }
    for (idx, x) in vec.iter().enumerate()
    {
        for (name, dist) in x.iter()
        {
            arr[idx].distance_to[valve_names.clone().position(|x| x == name).expect(format!("Failed to find name matching {}",name).as_str())] = *dist;    
        }
    }
    let len = valve_names.clone().count();
    println!("Building Compact form of {} bytes takes {} ms", size_of::<[Valve;16]>(), (Instant::now() - start).as_millis());
    let aa_idx = valve_names.clone().position(|x|"AA" == x).unwrap();

    (arr, len, aa_idx)
}

fn build_distance_graph_from(node: &str, dist: u8, input: &HashMap<String, Vec<&str>>, output: &mut HashMap<String, u8>) 
{
    let vec = input.get(node).expect(format!("Failed to find {} in input",node).as_str());
    for x in vec
    {
        let x_string = (&x).to_string();
        if output.contains_key(&x_string) == false
        {
            output.insert(x_string.clone(), dist + 1);
            build_distance_graph_from(&x, dist + 1, input, output);
        }
        else if output[&x_string] > dist + 1
        {
            output.insert(x_string, &dist+1);
            build_distance_graph_from(&x, dist + 1, input, output);
        }
    }
}