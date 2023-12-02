fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(as_digits)
        .map(|mut digits| (digits.next(), digits.next_back()))
        .map(|(x, y)| (x.unwrap_or_default(), y))
        .map(|(x, y)| (x, y.unwrap_or(x)))
        .map(|(x, y)| x * 10 + y)
        .sum::<u32>();
    println!("{result}");
}

static SPELLED_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn as_digits(line: &str) -> impl DoubleEndedIterator<Item = u32> + '_ {
    let mut digits = Vec::new();
    for begin in 0..line.len() {
        let current = &line[begin..];
        for (i, spelled_digit) in SPELLED_DIGITS.iter().enumerate() {
            let digit = (i + 1) as u32;
            let digit_char = char::from_digit(digit, 10).unwrap();
            if current.starts_with(digit_char) || current.starts_with(spelled_digit) {
                digits.push(digit);
                break;
            }
        }
    }
    digits.into_iter()
}
