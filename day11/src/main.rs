use std::collections::VecDeque;
use std::fs;

fn main()
{
    let filepath = "input.txt";
    println!("Part 1 {}", simulate(filepath, 20, 3.0));
    println!("Part 2 {}", simulate(filepath, 10_000, 1.0));
}

#[test]
fn test_part1()
{
    assert_eq!(simulate("src/test_input.txt", 20, 3.0), 10605)
}

#[test]
fn test_part2()
{
    assert_eq!(simulate("src/test_input.txt", 10_000, 1.0), 2713310158)
}

enum Op
{
    ADD,
    MULTIPLY, SQUARE
}

// 13, 19, 11, 17, 3, 7, 5, 2. Are all prime numbers!

struct Monkey
{
    items: VecDeque<u64>,
    op: Op,
    op_val: u64,
    test_divide_by: u64,
    on_true_pass_to: usize,
    on_false_pass_to: usize,
}

fn simulate(filepath: &str, rounds: u32, divide_by_relief: f64) -> u64
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let monkey_inputs = contents.split("\n\n");
    
    let mut monkeys: Vec<Monkey> = Vec::new();
    
    for input in monkey_inputs
    {
        let lines = input.lines().collect::<Vec<&str>>();
        let starting_items = lines[1]["  Starting items: ".len()..]
            .split(",")
            .map(|val| val.trim().parse().expect(format!("Failed to parse {}", val).as_str()))
            .collect();
        
        let operation_vec = lines[2][2..].split(" ").collect::<Vec<&str>>();
        let op = if operation_vec[4] == "+"
        {
            Some((Op::ADD, operation_vec[5].parse().expect("Failed to parse")))
        }
        else if operation_vec[4] == "*"
        {
            if operation_vec[5] == "old"
            {
                Some((Op::SQUARE, 0))
            }
            else { 
                Some((Op::MULTIPLY, operation_vec[5].parse().expect("Failed to parse into u32")))
            }
        }
        else { 
            None
        }.expect("Unexpected Operations found!");
        
        let mon = Monkey
        {
            items: starting_items,
            op: op.0,
            op_val: op.1,
            test_divide_by: lines[3]["  Test: divisible by ".len()..].parse().expect("Failed to parse divide by value!"),
            on_true_pass_to: lines[4]["    If true: throw to monkey ".len()..].parse::<usize>().expect("Failed to parse divide by value!"),
            on_false_pass_to: lines[5]["    If false: throw to monkey ".len()..].parse::<usize>().expect("Failed to parse divide by value!")
        };
        
        monkeys.push(mon);
    }
    
    let modulo = monkeys.iter().fold(1, |acc, x| acc * x.test_divide_by);
    
    let mut inspect_count = vec![0u64; monkeys.len()];
    
    for _ in 0..rounds
    {
        for i in 0..monkeys.len()
        {
            inspect_count[i] += monkeys[i].items.len() as u64;
            while let Some(item) = monkeys[i].items.pop_front() 
            {
                let item = (match monkeys[i].op
                {
                    Op::SQUARE => item * item,
                    Op::ADD => item + monkeys[i].op_val,
                    Op::MULTIPLY => item * monkeys[i].op_val,
                } as f64 / divide_by_relief).floor() as u64 % modulo;
                
                let idx = if item % monkeys[i].test_divide_by == 0
                {
                    monkeys[i].on_true_pass_to
                }
                else 
                {
                    monkeys[i].on_false_pass_to
                };
                monkeys[idx].items.push_back(item);
            }
        }
    }
    
    inspect_count.sort();
    return inspect_count[inspect_count.len() - 1] * inspect_count[inspect_count.len() - 2];
}
