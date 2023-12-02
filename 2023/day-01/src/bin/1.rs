fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)))
        .map(|mut digits| (digits.next(), digits.next_back()))
        .map(|(x, y)| (x.unwrap_or_default(), y))
        .map(|(x, y)| (x, y.unwrap_or(x)))
        .map(|(x, y)| x * 10 + y)
        .sum::<u32>();
    println!("{result}");
}
