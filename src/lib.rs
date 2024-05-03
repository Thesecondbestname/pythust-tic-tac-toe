#![feature(variant_count)]
use pyo3::prelude::*;
use std::mem;
const BOARDSIZE: usize = 3;
const PLAYERS: usize = mem::variant_count::<Player>();
const PADDING: usize = BOARDSIZE * 3 + 2;
struct Render;
struct Update;
struct Ready;
struct Over;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Player {
    Circle,
    Cross,
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
    Empty,
    Player(Player),
}
#[derive(Copy, Clone, PartialEq, Eq)]
struct Row([Field; BOARDSIZE]);
impl Row {
    fn new() -> Self {
        Row([Field::Empty; BOARDSIZE])
    }
    fn check_win(&self) -> bool {
        let init = self.0[0];
        self.0.iter().all(|a| a == &init)
    }
}
struct Until<T> {
    repeatable_action: fn() -> String,
    raw_data: String,
    state: State
}
struct RepeatUntil<T> {
    repeatable_action: fn() -> String,
    raw_data: String,
    state: State,
    predicate: Option<T>
}
impl<Player> RepeatUntil<Player> {
    fn repeat_by(mut self, fallback: fn(State) -> State) -> State {
        loop {
            match predicate(&l) {
                true => break,
                false => {
                    fallback(self.state);
                    self.state.game = (self.state.stack[self.state.level])(self.state.game);
                    self.raw_data
                    continue;
                }
            }
        }
        self.game = l;
    }}
impl<T> Until<T> {
    fn until(self, predicate: fn(&String) -> Option<T>) -> RepeatUntil<T> {
        RepeatUntil {
            raw_data: self.raw_data,
            state: self.state,
            repeatable_action: self.repeatable_action,
            predicate
        }
    }
}
type Action = Box<(dyn Fn(Game) -> Game + Send + Sync + 'static)>;
#[pyclass]
struct State {
    game: Game,
    stack: Vec<Action>,
    level: usize,
}
#[pymethods]
impl State {
    fn start_by(get_inp: fn() -> String) -> Until<Player> {
        let player = &get_inp()[..];
        let initial_state = State { game: Game::default(), stack: vec![], level: 0};
         Until::<Player> {repeatable_action:get_inp, raw_data:player.to_string(), state: initial_state}}
    
    

    fn after_that(mut self, f: fn(Game) -> Game) -> Self {
        self.stack.push(Box::new(move |game| f(game)));
        self.level += 1;
        self
    }
    fn and(self, f: fn(Game) -> Game) -> Self {
        Self {
            stack: vec![Box::new(move |game| f((self.stack[self.level])(game)))],
            ..self
        }
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
                    l = (self.stack[self.level])(l);
                    continue;
                }
            }
        }
        self.game = l;
        self
    }
}
struct Game {
    board: [Row; BOARDSIZE],
    turn: [Player; PLAYERS],
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: [Row::new(); BOARDSIZE],
            turn: [Player::Circle, Player::Cross],
        }
    }
}
impl Game {
    fn start(p: [Player; PLAYERS]) -> Self {
        Game {
            board: [Row::new(); BOARDSIZE],
            turn: p,
        }
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
fn input_player_is_valid(inp: String) -> bool {
    let player_order = match &inp[..] {
        "Circle" | "O" => [Player::Circle, Player::Cross],
        "Cross" | "X" => [Player::Cross, Player::Circle],
        e => Err(e),
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
        State::start_by(|| "O".to_owned()).until(input_player_is_valid);
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
impl_display!(Row, |s: &Row| {
    format!(
        "{}|",
        s.0.iter()
            .fold(String::new(), |acc, a| format!("{acc}| {a}"))
    )
});
impl_display!(Field, |s: &Field| {
    match s {
        Field::Player(a) => format!("{a}"),
        Field::Empty => " ".to_owned(),
    }
});
impl_display!(Player, |s: &Player| {
    match s {
        Player::Circle => "O",
        Player::Cross => "X",
    }
});
#[macro_export]
/// Takes a struct name as first argument and a closure of it's Struct
/// Synopsys: (`struct_name`, |s: &`struct_name`| match s{...})
macro_rules! impl_display {
    ($struct_name:ident, $write_calls:expr) => {
        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", &$write_calls(&self))
            }
        }
    };
}
