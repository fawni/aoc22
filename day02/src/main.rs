#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Hand {
    Rock = 1,
    Paper,
    Scissors,
}

impl From<&str> for Hand {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            _ => Self::Scissors,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum HandResult {
    Win,
    Lose,
    Draw,
}

impl From<&str> for HandResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            _ => Self::Win,
        }
    }
}

trait Hierarchy {
    fn beats(&self) -> Self;
    fn beaten_by(&self) -> Self;
}

impl Hierarchy for Hand {
    fn beats(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let rounds: Vec<&str> = input.lines().collect();
    let mut score = 0;
    // 01
    for round in &rounds {
        let hands: Vec<&str> = round.split(' ').collect();
        let (opponent, own) = (Hand::from(hands[0]), Hand::from(hands[1]));
        let res = play_hand(opponent, own);
        match res {
            HandResult::Win => score += 6 + own as u32,
            HandResult::Draw => score += 3 + own as u32,
            HandResult::Lose => score += own as u32,
        }
    }
    println!("{score}");

    //02
    score = 0;
    for round in rounds {
        let values: Vec<&str> = round.split(' ').collect();
        let (opponent, needed_outcome) = (Hand::from(values[0]), HandResult::from(values[1]));
        let own = calculate_hand(opponent, needed_outcome);
        let res = play_hand(opponent, own);
        match res {
            HandResult::Win => score += 6 + own as u32,
            HandResult::Draw => score += 3 + own as u32,
            HandResult::Lose => score += own as u32,
        }
    }
    println!("{score}");
}

fn play_hand(opponent: Hand, own: Hand) -> HandResult {
    if own.beats() == opponent {
        HandResult::Win
    } else if opponent.beats() == own {
        HandResult::Lose
    } else {
        HandResult::Draw
    }
}

fn calculate_hand(opponent: Hand, outcome: HandResult) -> Hand {
    match outcome {
        HandResult::Draw => opponent,
        HandResult::Lose => opponent.beats(),
        HandResult::Win => opponent.beaten_by(),
    }
}
