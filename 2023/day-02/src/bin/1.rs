const AVAILABLE_SET: Set = Set {
    r: 12,
    g: 13,
    b: 14,
};

fn main() {
    let input = include_str!("../../input.txt");
    let result = input
        .lines()
        .map(|record| record.split_once(':').unwrap())
        .map(|(game, sets)| (game[5..].parse::<usize>().unwrap(), sets))
        .map(|(game_id, sets)| (game_id, sets.trim().split("; ")))
        .map(|(game_id, sets)| (game_id, sets.map(Set::from)))
        .filter(|(_, sets)| {
            sets.clone().all(|set| {
                set.r <= AVAILABLE_SET.r && set.g <= AVAILABLE_SET.g && set.b <= AVAILABLE_SET.b
            })
        })
        .map(|(game_id, _)| game_id)
        .sum::<usize>();
    println!("{result}");
}

#[derive(Debug, Default, Clone)]
struct Set {
    r: usize,
    g: usize,
    b: usize,
}

impl From<&str> for Set {
    fn from(s: &str) -> Self {
        s.split(", ")
            .map(|s| s.split_once(' ').unwrap())
            .map(|(count, color)| (count.parse::<usize>().unwrap(), color))
            .fold(
                Set::default(),
                |Set { r, g, b }, (count, color)| match color {
                    "red" => Set { r: r + count, g, b },
                    "green" => Set { r, g: g + count, b },
                    "blue" => Set { r, g, b: b + count },
                    _ => unreachable!(),
                },
            )
    }
}
