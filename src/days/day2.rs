use crate::days;

#[derive(Clone, Copy)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn from_opponent(c: char) -> Option<Self> {
        match c {
            'A' => Some(RPS::Rock),
            'B' => Some(RPS::Paper),
            'C' => Some(RPS::Scissors),
            _ => None,
        }
    }

    fn from_me(c: char) -> Option<Self> {
        match c {
            'X' => Some(RPS::Rock),
            'Y' => Some(RPS::Paper),
            'Z' => Some(RPS::Scissors),
            _ => None,
        }
    }

    fn from_opponent_win(&self, win: &Win) -> Self {
        match (self, win) {
            (RPS::Rock, Win::Me) => Self::Paper,
            (RPS::Rock, Win::Opponent) => Self::Scissors,
            (RPS::Paper, Win::Me) => Self::Scissors,
            (RPS::Paper, Win::Opponent) => Self::Rock,
            (RPS::Scissors, Win::Me) => Self::Rock,
            (RPS::Scissors, Win::Opponent) => Self::Paper,
            (opponent, Win::Draw) => opponent.clone(),
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }
}

enum Win {
    Me,
    Draw,
    Opponent,
}

impl Win {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'Z' => Some(Win::Me),
            'Y' => Some(Win::Draw),
            'X' => Some(Win::Opponent),
            _ => None,
        }
    }

    fn from_rps(me: &RPS, opponent: &RPS) -> Self {
        match (me, opponent) {
            (RPS::Rock, RPS::Scissors) => Win::Me,
            (RPS::Scissors, RPS::Paper) => Win::Me,
            (RPS::Paper, RPS::Rock) => Win::Me,

            (RPS::Rock, RPS::Paper) => Win::Opponent,
            (RPS::Paper, RPS::Scissors) => Win::Opponent,
            (RPS::Scissors, RPS::Rock) => Win::Opponent,

            (RPS::Rock, RPS::Rock) => Win::Draw,
            (RPS::Paper, RPS::Paper) => Win::Draw,
            (RPS::Scissors, RPS::Scissors) => Win::Draw,
        }
    }

    fn to_score(&self) -> u32 {
        match self {
            Win::Me => 6,
            Win::Draw => 3,
            Win::Opponent => 0,
        }
    }
}

pub struct Day {}

impl days::Day for Day {
    type Input = Vec<(RPS, char)>;

    fn get_num(&self) -> u8 {
        2
    }

    fn new() -> Self {
        Self {}
    }

    fn part1(&mut self, input: &Self::Input) -> (String, bool) {
        let input = input
            .iter()
            .map(|(opponent, me)| (opponent.clone(), RPS::from_me(*me).unwrap()))
            .collect::<Vec<(RPS, RPS)>>();
        let mut total_score = 0;
        for (opponent, me) in input.iter() {
            total_score += Win::from_rps(me, opponent).to_score() + me.to_score();
        }
        (total_score.to_string(), true)
    }

    fn part2(&mut self, input: &Self::Input) -> (String, bool) {
        let input = input
            .iter()
            .map(|(opponent, win)| (opponent.clone(), Win::from_char(*win).unwrap()))
            .collect::<Vec<(RPS, Win)>>();
        let mut total_score = 0;
        for (opponent, win) in input.iter() {
            total_score += opponent.from_opponent_win(win).to_score() + win.to_score();
        }
        (total_score.to_string(), true)
    }

    fn parse_input(&mut self, input: &String) -> Self::Input {
        input
            .lines()
            .map(|x| {
                let mut split = x.split(" ");
                (
                    RPS::from_opponent(split.next().unwrap().chars().next().unwrap()).unwrap(),
                    split.next().unwrap().chars().next().unwrap(),
                )
            })
            .collect()
    }
}
