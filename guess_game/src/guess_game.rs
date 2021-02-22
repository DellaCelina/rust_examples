/// Guess game result
pub enum GameResult {
    Success,
    Failed,
}

/// Guess game trait
pub trait Guesser {
    /// Get guess value
    fn guess(&self) -> u32;

    /// Get target value
    fn target(&self) -> u32;

    /// Compare guess and target and getting result
    fn result(&self) -> GameResult {
        use std::cmp::Ordering;
        match self.guess().cmp(&self.target()) {
            Ordering::Equal => {
                println!("Success the game!");
                GameResult::Success
            }
            Ordering::Less => {
                println!("Guess is less than target");
                GameResult::Failed
            }
            Ordering::Greater => {
                println!("Guess is greater than target");
                GameResult::Failed
            }
        }
    }

    /// Start game and loop until game is succeed
    fn gaming(&self) {
        println!("Guessing game!");

        loop {
            match self.result() {
                GameResult::Success => break,
                GameResult::Failed => continue,
            };
        }
    }
}

/// Game status
/// Save target value to guess
pub struct GameStatus {
    target: u32,
}

impl GameStatus {
    /// Create new GameStatus with target value
    pub fn new() -> GameStatus {
        GameStatus {
            target: GameStatus::get_target_number(),
        }
    }

    /// Get guess value from user
    fn get_guess_from_user() -> std::io::Result<u32> {
        use std::io;

        let guess = loop {
            let mut guess = String::new();
            io::stdin().read_line(&mut guess)?;
            match guess.trim().parse() {
                Ok(num) => break num,
                Err(_) => {
                    println!("Guess number should be number");
                    continue;
                }
            };
        };

        println!("You guessed {}", guess);

        Ok(guess)
    }

    /// Get target number from rand
    fn get_target_number() -> u32 {
        use rand::Rng;

        let num = rand::thread_rng().gen_range(1, 101);
        println!("Target number is {}", num);
        num
    }
}

impl Guesser for GameStatus {
    fn guess(&self) -> u32 {
        GameStatus::get_guess_from_user().expect("io result error")
    }

    fn target(&self) -> u32 {
        self.target
    }
}
