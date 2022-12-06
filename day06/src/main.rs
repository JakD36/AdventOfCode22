

fn main() {
    let filepath = "input.txt";
    let signal = std::fs::read_to_string(filepath)
        .expect(format!("Failed to read {}", filepath).as_str());
    let part1 = find_start_of_marker(signal.as_str(), MarkerType::PACKET);
    let part2 = find_start_of_marker(signal.as_str(), MarkerType::MESSAGE);
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
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
