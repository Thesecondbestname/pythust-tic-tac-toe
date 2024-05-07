#![feature(variant_count)]
const BOARDSIZE: usize = 3;
const PLAYERS: usize = mem::variant_count::<Player>();
const PADDING: usize = BOARDSIZE * 3 + 2;
mod tic;
use crate::tic::Game;
use crate::tic::Player;
use core::mem;
use pyo3::prelude::*;
use std::marker::PhantomData;
use std::sync::Arc;
struct Until<T> {
    recall_stack: InpAction,
    state: State,
    p: PhantomData<T>,
}
struct RepeatUntil<T> {
    repeated_call: InpAction,
    state: State,
    predicate: fn(&String) -> Option<T>,
}
impl RepeatUntil<[Player; 2]> {
    // we want to do this in closure form (for the memes)
    fn repeat_by(mut self, fallback: fn() -> String) -> State {
        let call = Arc::clone(&self.state.stack[self.state.level]);
        let closure = move |game| {
            let mut game = game;
            let mut result = (self.repeated_call)(game);
            loop {
                match (self.predicate)(&result) {
                    Some(b) => {
                        return Game {
                            turn: b,
                            ..self.state.game
                        }
                    }
                    None => {
                        game = call(game);
                        result = fallback();
                    }
                }
            }
        };
        self.state.stack[self.state.level] = Arc::new(closure);
        State { ..self.state }
    }
}
impl<T> Until<T> {
    fn until(self, predicate: fn(&String) -> Option<T>) -> RepeatUntil<T> {
        RepeatUntil {
            repeated_call: self.recall_stack,
            state: self.state,
            predicate,
        }
    }
}
type Action = Arc<(dyn Fn(Game) -> Game + Send + Sync + 'static)>;
type InpAction = Arc<(dyn Fn(Game) -> String + Send + Sync + 'static)>;
#[pyclass]
struct State {
    game: Game,
    stack: Vec<Action>,
    level: usize,
}
impl Default for State {
    fn default() -> Self {
        State {
            game: Game::default(),
            stack: vec![Arc::new(|game| game)],
            level: 0,
        }
    }
}
#[pymethods]
impl State {
    fn start_by(get_inp: fn() -> String) -> Until<[Player; 2]> {
        let state = State::default();
        Until::<[Player; 2]> {
            recall_stack: Arc::new(move |game| get_inp()),
            state,
            p: PhantomData,
        }
    }

    fn after_that(&self, f: fn(Game) -> Game) -> Self {
        let b = [self.stack, vec![Arc::new(move |game| f(game))]].concat();
        Self {
            game: self.game,
            stack: b,
            level: self.level + 1,
        }
    }
    fn and(self, f: fn(Game) -> Game) -> Self {
        Self {
            stack: vec![Arc::new(move |game| f(self.stack[self.level](game)))],
            game: self.game.clone(),
            level: self.level.clone(),
        }
    }
    fn call(&self, game: Game) -> Game {
        (self.stack[self.level])(game)
    }
    fn finally(self) -> () {
        (self.stack)[0](self.game);
    }
    fn until(mut self, predicate: fn(&Game) -> bool) -> State {
        let mut l = self.game;
        loop {
            match predicate(&l) {
                true => break,
                false => {
                    l = self.call(l);
                    continue;
                }
            }
        }
        self.game = l;
        self
    }
}
fn print(game: Game) -> Game {
    println!(
        "{0}\n{1:-<width$}",
        game.board
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (b, a)| format!(
                "{acc}\n{:-<width$}\n{b}{a}",
                "",
                width = PADDING
            )),
        "",
        width = PADDING,
    );
    game
}
// fn update(mut game: Game) -> Option<Game> {
//     game.board[a].0[b] = Field::Player(game.turn[0]);
//     Some(game)
// }
fn advance_turn(game: Game) -> Game {
    let mut a = game.turn;
    let len = a.len();
    a.swap(0, len);
    Game {
        turn: a,
        board: game.board,
    }
}
fn input_player_is_valid(inp: &String) -> Option<[Player; 2]> {
    let player_order = match &inp[..] {
        "Circle" | "O" => Some([Player::Circle, Player::Cross]),
        "Cross" | "X" => Some([Player::Cross, Player::Circle]),
        e => None,
    };
    player_order
}
/// Formats the sum of two numbers as string.
// #[pyfunction]
fn get_user(input: String) -> Option<Coordinate> {
    let mut coords = input.chars().skip(2);
    let alpha = coords.next()?;
    let num = coords.next()?;
    let a = match alpha {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        _ => return None,
    };
    let b = match num as usize {
        0..=3 => num as usize,
        _ => return None,
    };
    Some(Coordinate(a, b))
}
struct Coordinate(usize, usize);
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> () {
        State::start_by(|| "O".to_owned())
            .until(input_player_is_valid)
            .rep;
        // State::start().then(print).then(get_user(input())).until(its_valid)
    }
}
/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn tic_tac_toe(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
