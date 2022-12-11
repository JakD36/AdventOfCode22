use std::fs;

fn main() 
{
    let filepath = "input.txt";
    part1(filepath);
    part2(filepath);
}

fn part1(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    
    let mut check_idx = 0;
    let cycle_check = [20, 60, 100, 140, 180, 220];
    let mut current_cyle = 0;
    let mut x = 1;
    
    let mut sum = 0;
    
    'outer: for line in contents.lines()
    {
        if line.starts_with("noop")
        {
            let finishes_at = current_cyle + 1;
            loop 
            {
                if check_idx >= cycle_check.len()
                {
                    break 'outer;
                }
                
                if finishes_at >= cycle_check[check_idx]
                {
                    // println!("{}", cycle_check[check_idx] * x);
                    sum += cycle_check[check_idx] * x;
                    check_idx += 1;
                }
                else 
                {
                    break;
                }
            }
            current_cyle = finishes_at;
        }
        else if line.starts_with("addx")
        {
            let finishes_at = current_cyle + 2;
            loop
            {
                if check_idx >= cycle_check.len()
                {
                    break 'outer;
                }

                if finishes_at >= cycle_check[check_idx]
                {
                    // println!("{}", cycle_check[check_idx] * x);
                    sum += cycle_check[check_idx] * x;
                    check_idx += 1;
                }
                else
                {
                    break;
                }
            }
            let substr = &line["addx".len() + 1..];
            x += substr.parse::<i32>().expect(format!("Failed to parse {} as a number",substr).as_str());
            current_cyle = finishes_at;
        }
    }
    
    println!("Part 1 {}", sum);
}

fn part2(filepath: &str)
{
    let contents = fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    
    let mut current_cyle = 0;
    let mut x = 1;
    
    'outer: for line in contents.lines()
    {
        let res = if line.starts_with("noop")
        {
            Some((1, 0))
        }
        else if line.starts_with("addx")
        {
            let substr = &line["addx".len() + 1..];
            Some((2, substr.parse::<i32>().expect(format!("Failed to parse {} as a number",substr).as_str())))
        }
        else { 
            None  
        };

        let (move_cycles, addx) = res.expect("Unexpected cmd found!");

        for i in current_cyle..(current_cyle + move_cycles) as i32 
        {
            if i % 40 == 0
            {
                print!("\n");
            }
            if (x - i % 40).abs() < 2
            {
                print!("#")
            }
            else 
            {
                print!(".")   
            }
        }
        current_cyle += move_cycles;
        x += addx;
    }
}