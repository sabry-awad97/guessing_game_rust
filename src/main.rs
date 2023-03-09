use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
use std::time::{Duration, Instant};

enum GuessResult {
    Correct,
    TooLow,
    TooHigh,
}

#[derive(Copy, Clone)]
enum Difficulty {
    Easy((u32, u32)),
    Medium((u32, u32)),
    Hard((u32, u32)),
}

struct Score {
    points: u32,
    multiplier: u32,
}

struct Game {
    secret_number: u32,
    range: (u32, u32),
    difficulty: Difficulty,
}

impl Game {
    fn new() -> Game {
        let range = (1, 100);
        let secret_number = rand::thread_rng().gen_range(range.0..=range.1);
        Game {
            secret_number,
            range,
            difficulty: Difficulty::Easy(range),
        }
    }

    fn set_difficulty(&mut self, difficulty: Difficulty) {
        use Difficulty::*;
        self.difficulty = difficulty;

        match difficulty {
            Easy(range) => self.range = range,
            Medium(range) => self.range = range,
            Hard(range) => self.range = range,
        }

        self.secret_number = rand::thread_rng().gen_range(self.range.0..=self.range.1);
    }

    fn play(&self) -> GuessResult {
        println!("Please input your guess.");

        let mut guess = String::new();
        stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => return GuessResult::TooLow,
        };

        match guess.cmp(&self.secret_number) {
            Ordering::Less => GuessResult::TooLow,
            Ordering::Greater => GuessResult::TooHigh,
            Ordering::Equal => GuessResult::Correct,
        }
    }
}

enum GameOutcome {
    Win,
    Lose,
}

struct GameManager {
    game: Game,
    score: Score,
    start_time: Instant,
    end_time: Instant,
    time_limit: Duration,
}

impl GameManager {
    fn new(time_limit: Duration) -> GameManager {
        GameManager {
            game: Game::new(),
            score: Score {
                points: 0,
                multiplier: 1,
            },
            start_time: Instant::now(),
            end_time: Instant::now(),
            time_limit,
        }
    }

    fn run(&mut self) -> GameOutcome {
        // Prompt player to choose a difficulty level
        println!("Choose a difficulty level:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        let mut difficulty_choice = String::new();
        stdin()
            .read_line(&mut difficulty_choice)
            .expect("Failed to read line");

        let difficulty = match difficulty_choice.trim() {
            "1" => Difficulty::Easy((1, 100)),
            "2" => Difficulty::Medium((1, 500)),
            "3" => Difficulty::Hard((1, 1000)),
            _ => {
                println!("Invalid difficulty level, defaulting to easy");
                Difficulty::Easy((1, 100))
            }
        };

        self.game.set_difficulty(difficulty);

        let mut num_guesses = 0;
        let start_time = Instant::now();
        loop {
            if start_time.elapsed() >= self.time_limit {
                println!("Time's up!");
                return GameOutcome::Lose;
            }
            match self.game.play() {
                GuessResult::Correct => {
                    let score = self.score.points * self.score.multiplier;
                    self.end_time = Instant::now();
                    let elapsed_time = self.end_time - self.start_time;
                    let elapsed_seconds = elapsed_time.as_secs();
                    println!(
                        "You win! Your score is {} and you took {} seconds to complete the game.",
                        score, elapsed_seconds
                    );
                    return GameOutcome::Win;
                }
                GuessResult::TooLow => {
                    num_guesses += 1;
                    println!("Too low!");
                }
                GuessResult::TooHigh => {
                    num_guesses += 1;
                    println!("Too high!");
                }
            }

            // Update score multiplier based on number of guesses
            if num_guesses == 10 {
                self.score.multiplier = 3;
            } else if num_guesses == 5 {
                self.score.multiplier = 2;
            }

            // Update score based on number of guesses and current multiplier
            self.score.points = 100 - (10 * num_guesses);
        }
    }
}

fn main() {
    let seconds: u64 = 10;
    println!("{}", format!("Guess the number! in {} seconds", seconds));
    let mut manager = GameManager::new(Duration::from_secs(seconds));
    manager.run();
}
