use std::fs;

const OUTCOME_LOSS: i32 = 0;
const OUTCOME_DRAW: i32 = 3;
const OUTCOME_WIN: i32 = 6;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    fn selection_to_lose(self) -> Selection {
        match self {
            Selection::Rock => Selection::Scissors,
            Selection::Paper => Selection::Rock,
            Selection::Scissors => Selection::Paper,
        }
    }

    fn selection_to_win(self) -> Selection {
        match self {
            Selection::Rock => Selection::Paper,
            Selection::Paper => Selection::Scissors,
            Selection::Scissors => Selection::Rock,
        }
    }

    fn score_against(self, other: Selection) -> i32 {
        let baseline = match self {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        };
        if self == other {
            baseline + OUTCOME_DRAW
        } else if self == other.selection_to_win() {
            baseline + OUTCOME_WIN
        } else {
            baseline + OUTCOME_LOSS
        }
    }
}

pub fn day_2() {
    let contents = fs::read_to_string("input/day_2")
        .expect("Something went wrong reading the file");

    // Part 1
    let total_score_1 =
        contents.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s|
                s.split(" ")
                    .map(|s| {
                        match s {
                            "A" => Selection::Rock,
                            "B" => Selection::Paper,
                            "C" => Selection::Scissors,
                            "X" => Selection::Rock,
                            "Y" => Selection::Paper,
                            "Z" => Selection::Scissors,
                            _ => panic!("Invalid selection '{}'", s),
                        }
                    })
                    .collect::<Vec<Selection>>()
            )
            .map(|s| s[1].score_against(s[0]))
            .sum::<i32>();
    println!("{}", total_score_1);

    // Part 2
    let total_score_2 =
        contents.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| {
                let split = s.split(" ").collect::<Vec<&str>>();
                let their_choice = match split[0] {
                    "A" => Selection::Rock,
                    "B" => Selection::Paper,
                    "C" => Selection::Scissors,
                    _ => panic!("Invalid selection '{}'", split[0]),
                };
                let my_choice = match split[1] {
                    "X" => their_choice.selection_to_lose(),
                    "Y" => their_choice,
                    "Z" => their_choice.selection_to_win(),
                    _ => panic!("Invalid selection '{}'", split[1]),
                };
                my_choice.score_against(their_choice)
            })
            .sum::<i32>();
    println!("{}", total_score_2);
}
