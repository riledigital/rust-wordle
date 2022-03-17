use ggez::event;
use ggez::graphics;

use ggez::GameError;
use ggez::{Context, GameResult};
use glam::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env;

use std::io::{stdin, Read};
use std::path;

// First we make a structure to contain the game's state
struct MainState {
    frames: usize,
    text: graphics::Text,
    word_answer: String,
    tries: u8,

    history: Vec<Vec<GuessState>>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        // The ttf file will be in your resources directory. Later, we
        // will mount that directory so we can omit it in the path here.
        let word_answer = choose_random_word(ctx)?;
        println!("word is {}", word_answer);

        let font = graphics::Font::new(ctx, "/LiberationMono-Regular.ttf")?;
        let text = graphics::Text::new((word_answer.clone(), font, 48.0));

        let history = Vec::new();
        let s = MainState {
            frames: 0,
            text,
            word_answer,
            tries: 0,
            history,
        };
        Ok(s)
    }
}

// Then we implement the `ggez:event::EventHandler` trait on it, which
// requires callbacks for updating and drawing the game state each frame.
//
// The `EventHandler` trait also contains callbacks for event handling
// that you can override if you wish, but the defaults are fine.
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        while *&self.tries < 6 {
            let remaining = 6 - &self.tries;
            let mut buf = String::new();

            println!(
                "Tries remaining: {} • Please guess a 5-letter word: ",
                remaining
            );
            while buf.chars().count() != 5 {
                buf = String::new();
                stdin().read_line(&mut buf).expect("Type an input");
                buf.pop();
                if buf.chars().count() != 5 {
                    println!("Your guess must be 5 letters! {:?} is invalid.", buf);
                    buf = String::new();
                }
            }

            let result = test_guess(&buf, &self.word_answer);
            self.tries += 1;
            self.history.push(result.clone());
            for tries in &self.history {
                println!("{}", result_to_emoji(&tries));
            }
            if score_results(&result) == ResultScore::Done {
                println!("You win!");
                ggez::event::quit(_ctx);
                std::process::exit(0);
            } else {
            }
        }
        println!("Out of guesses!");
        ggez::event::quit(_ctx);
        std::process::exit(0);
        // Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        // Drawables are drawn from their top-left corner.
        let offset = self.frames as f32 / 10.0;
        let dest_point = glam::Vec2::new(offset, offset);
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }
}

// Now our main function, which does three things:
//
// * First, create a new `ggez::ContextBuilder`
// object which contains configuration info on things such
// as screen resolution and window title.
// * Second, create a `ggez::game::Game` object which will
// do the work of creating our MainState and running our game.
// * Then, just call `game.run()` which runs the `Game` mainloop.
pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("helloworld", "ggez").add_resource_path(resource_dir);
    let (mut ctx, event_loop) = cb.build()?;

    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}

// Pick a word to start with
fn choose_random_word(ctx: &mut Context) -> Result<String, GameError> {
    let mut buf = String::new();
    let mut words_file = ggez::filesystem::open(ctx, "/words.txt")?;
    words_file.read_to_string(&mut buf)?;
    let splits = buf.split("\n").collect::<Vec<_>>();
    let mut rng = thread_rng();
    // println!("{:?}", splits);
    if let Some(word) = splits.choose(&mut rng) {
        return Ok(word.to_string());
    } else {
        return Err(GameError::ResourceLoadError(
            "Error selecting word".to_string(),
        ));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GuessState {
    Correct,
    WrongPosCorrectLetter,
    WrongPosWrongLetter,
}

// Test the guess
fn test_guess(input: &String, answer: &String) -> Vec<GuessState> {
    let input = input.clone();

    let mut result = vec![GuessState::WrongPosWrongLetter; 5];
    let answer_chars = answer.chars().collect::<Vec<char>>();

    // println!("{:?}", input);
    if input == *answer {
        // If the strings are the same you win
        return vec![GuessState::Correct; 5];
    }

    for (idx, letter) in input.chars().enumerate() {
        // println!("{:?}", answer_chars);
        if letter == answer_chars[idx] {
            result[idx] = GuessState::Correct;
        } else if answer_chars.contains(&letter) {
            result[idx] = GuessState::WrongPosCorrectLetter;
        } else {
            result[idx] = GuessState::WrongPosWrongLetter;
        }
    }
    result
}

fn result_to_emoji(result: &Vec<GuessState>) -> String {
    // 🟩🟨⬛
    result
        .iter()
        .map(|char| match char {
            GuessState::Correct => "🟩",
            GuessState::WrongPosCorrectLetter => "🟨",
            GuessState::WrongPosWrongLetter => "⬛",
        })
        .collect()
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum ResultScore {
    Done,
    Incorrect,
}

fn score_results(result: &Vec<GuessState>) -> ResultScore {
    let mut victory = Vec::new();
    for _ in 0..5 {
        victory.push(GuessState::Correct);
    }

    if result
        .iter()
        .zip(victory)
        .map(|(x, y)| *x == y)
        .reduce(|acc, x| acc && x)
        .unwrap()
    {
        return ResultScore::Done;
    } else {
        return ResultScore::Incorrect;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guess_correct() {
        let all_correct = vec![
            GuessState::Correct,
            GuessState::Correct,
            GuessState::Correct,
            GuessState::Correct,
            GuessState::Correct,
        ];
        assert_eq!(
            all_correct,
            test_guess(&"rogue".to_string(), &"rogue".to_string(),)
        );
    }

    #[test]
    fn test_guess_all_wrong() {
        let correct_some = vec![
            GuessState::WrongPosCorrectLetter,
            GuessState::WrongPosCorrectLetter,
            GuessState::WrongPosCorrectLetter,
            GuessState::WrongPosCorrectLetter,
            GuessState::WrongPosCorrectLetter,
        ];
        assert_eq!(
            correct_some,
            test_guess(&"rogue".to_string(), &"eurog".to_string())
        );
    }
}
