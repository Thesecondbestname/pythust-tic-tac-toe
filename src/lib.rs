#![feature(variant_count)]
use std::mem;
use pyo3::prelude::*;
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
type Action = Box<(dyn Fn(Game) -> Game + Send + Sync + 'static)>;
#[pyclass]
struct State {
    game: Game,
    actions: Action,
    ops: Option<String>
}
impl State {
    fn start() -> Self {
        State {
            game: Game::start([Player::Circle, Player::Cross]),
            actions: Box::new(move |game| game),
            ops: None,
        }
    }
    fn then(self, f: fn(Game) -> Game) -> Self {
        Self {
            actions: Box::new(move |game| f((self.actions)(game))),
            ..self
        }
    }
    fn and(self, f: fn(Game) -> Game) -> Self {
        Self {
            actions: Box::new(move |game| f((self.actions)(game))),
            ..self
        }
    }
    fn quit(self) -> () {
        (self.actions)(self.game);
    }
    fn until(mut self, predicate: fn(&Game) -> bool) -> State {
        fn rec(game: Game, predicate: fn(&Game) -> bool, a: &Action) -> Game {
            match predicate(&game) {
                true => return game,
                false => rec((a)(game), predicate, a),
            }
        }
        self.game = rec(self.game, predicate, &self.actions);
        self
    }
}
struct Game {
    board: [Row; BOARDSIZE],
    turn: [Player; PLAYERS],
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
fn try_update(mut game: Game, inp: String) -> Option<Game> {
    let mut coords = inp.chars().skip(2);
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
    game.board[a].0[b] = Field::Player(game.turn[0]);
    Some(game)
}
fn advance_turn(game: Game) -> Game {
    let mut a = game.turn;
    a.swap(0, a.len());
    Game {
        turn: a,
        board: game.board,
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> () {
        State::start().then(print).quit();
    }
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
