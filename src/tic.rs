use core::mem;
use crate::impl_display;
const BOARDSIZE: usize = 3;
const PLAYERS: usize = mem::variant_count::<Player>();
const PADDING: usize = BOARDSIZE * 3 + 2;
struct Coordinate(usize, usize);
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
