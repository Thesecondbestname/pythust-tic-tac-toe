use crate::impl_display;
use crate::BOARDSIZE;
use crate::PLAYERS;
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Player {
    Circle,
    Cross,
}
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum Field {
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: [Row; BOARDSIZE],
    pub turn: [Player; PLAYERS],
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
