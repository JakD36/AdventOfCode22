
enum Type
{
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
}

enum GameResult
{
    WON = 6,
    LOST = 0,
    DRAW = 3
}

fn main()
{
    let filepath = "input.txt";
    // part1(filepath);
    part2(filepath);
}

fn decrypt_opp(input: &str) -> Option<Type>
{
    match input
    {
        "A" => Some(Type::ROCK),
        "B" => Some(Type::PAPER),
        "C" => Some(Type::SCISSORS),
        _ => None
    }
}

fn decrypt_ours(input: &str) -> Option<Type>
{
    match input
    {
        "X" => Some(Type::ROCK),
        "Y" => Some(Type::PAPER),
        "Z" => Some(Type::SCISSORS),
        _ => None
    }
}

fn part1(filepath: &str)
{
    let contents = std::fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let lines = contents.split("\n").map(|x| x.split(" "));
    let mut score : i32 = 0;
    for line in lines 
    {
        let plays : Vec<&str> = line.collect();
        let their_play = decrypt_opp(plays[0]).expect("Failed to understand their choice.");
        let our_play = decrypt_ours(plays[1]).expect("Failed to understand our choice.");
        
        let result = match (&our_play, &their_play)
        {
            (Type::ROCK, Type::PAPER) => GameResult::LOST,
            (Type::ROCK, Type::SCISSORS) => GameResult::WON,
            
            (Type::PAPER, Type::ROCK) => GameResult::WON,
            (Type::PAPER, Type::SCISSORS) => GameResult::LOST,
            
            (Type::SCISSORS, Type::ROCK) => GameResult::LOST,
            (Type::SCISSORS, Type::PAPER) => GameResult::WON,
            _ => GameResult::DRAW,
        };
        
        let round_score = result as i32 + our_play as i32;
        // println!("{}", round_score);
        score += round_score;
    }
    println!("{}", score);
}

fn decrypt_result(input: &str) -> Option<GameResult>
{
    match input 
    { 
        "X" => Some(GameResult::LOST),
        "Y" => Some(GameResult::DRAW),
        "Z" => Some(GameResult::WON),
        _ => None
    }
}

fn part2(filepath: &str)
{
    let contents = std::fs::read_to_string(filepath).expect(format!("Failed to read {}", filepath).as_str());
    let lines = contents.split("\n").map(|x| x.split(" "));
    let mut score : i32 = 0;
    for line in lines
    {
        let plays : Vec<&str> = line.collect();
        let their_play = decrypt_opp(plays[0]).expect("Failed to understand their choice.");
        let result = decrypt_result(plays[1]).expect("Failed to understand the required choice!");

        let our_play = match (&result, &their_play)
        {
            (GameResult::WON, Type::ROCK) => Type::PAPER,
            (GameResult::WON, Type::PAPER) => Type::SCISSORS,
            (GameResult::WON, Type::SCISSORS) => Type::ROCK,

            (GameResult::LOST, Type::ROCK) => Type::SCISSORS,
            (GameResult::LOST, Type::PAPER) => Type::ROCK,
            (GameResult::LOST, Type::SCISSORS) => Type::PAPER,
            _ => their_play,
        };

        let round_score = result as i32 + our_play as i32;
        // println!("{}", round_score);
        score += round_score;
    }
    println!("{}", score);
}
