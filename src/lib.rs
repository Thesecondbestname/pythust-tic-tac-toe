use std::{marker::PhantomData, mem};

use pyo3::prelude::*;
const BOARDSIZE: usize = 3;
const PLAYERS: usize = 2;

struct Render;
struct Update;
struct Ready;
struct Over;
#[derive(Copy, Clone, PartialEq, Eq)]
enum Player {
    Circle,
    Cross,
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum Field {
    Empty,
    Player(Player),
}
struct Row([Field; BOARDSIZE]);
impl Row {
    fn check_win(&self) -> bool {
        self.0
            .windows(2)
            .scan(false, |s, i| {
                if i[0] != i[1] {
                    return None;
                } else {
                    Some(true)
                }
            })
            .fold(false, |a, b| b && a)
    }
}
struct Game<State> {
    board: [Row; BOARDSIZE],
    turn: Player,
    s: PhantomData<State>,
}
impl_display!(Row, |s: &Row| {
    format!(
        "{} |",
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
impl Game<Render> {
    fn print(self) -> Game<Update> {
        println!(
            "{2:-<0$}{1}",
            BOARDSIZE * 3,
            self.board
                .iter()
                .fold(String::new(), |acc, a| format!("{acc}\n{a}")),
            ""
        );
        Game {
            board: self.board,
            turn: self.turn,
            s: PhantomData,
        }
    }
}
struct Coordinate(usize, usize);
impl Game<Update> {
    fn try_update(mut self, inp: String) -> Option<Self> {
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
        self.board[a].0[b] = Field::Player(self.turn.clone());
        Some(self)
    }
    fn advance_turn(self, player: Player) -> Game<Ready> {
        Game {
            turn: player,
            board: self.board,
            s: PhantomData,
        }
    }
}
// impl<Idc> Game<Idc> {
//     fn check_win(self) -> Result<Game<Over>, Game<Ready>> {
//         for row in self.board {}
//     }
// }

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
