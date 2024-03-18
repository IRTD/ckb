use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct SortedKeySet {
    keys: Vec<(usize, Key)>,
}

impl TryFrom<String> for SortedKeySet {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self> {
        let mut keys: HashMap<Key, usize> = HashMap::new();
        for c in value.chars() {
            let key = Key::from_str(&c.to_string())?;
            match keys.get_mut(&key) {
                Some(mut count) => *count += 1,
                None => {
                    keys.insert(key, 1);
                }
            }
        }

        let mut keys = keys
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect::<Vec<(usize, Key)>>();

        keys.sort_by(|own, other| other.0.cmp(&own.0));

        Ok(SortedKeySet { keys })
    }
}

#[derive(strum::EnumString, Debug, Hash, Eq, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Key {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
    #[strum(serialize = "1", serialize = "!")]
    Num1,
    #[strum(serialize = "2", serialize = "@")]
    Num2,
    #[strum(serialize = "3", serialize = "#")]
    Num3,
    #[strum(serialize = "4", serialize = "$")]
    Num4,
    #[strum(serialize = "5", serialize = "%")]
    Num5,
    #[strum(serialize = "6", serialize = "^")]
    Num6,
    #[strum(serialize = "7", serialize = "&")]
    Num7,
    #[strum(serialize = "8", serialize = "*")]
    Num8,
    #[strum(serialize = "9", serialize = "(")]
    Num9,
    #[strum(serialize = "0", serialize = ")")]
    Num0,
    #[strum(serialize = ",", serialize = "<")]
    Comma,
    #[strum(serialize = ".", serialize = ">")]
    Period,
    #[strum(serialize = "/", serialize = "?")]
    ForSlash,
    #[strum(serialize = "\\", serialize = "|")]
    BackSlash,
    #[strum(serialize = ";", serialize = ":")]
    Semicolon,
    #[strum(serialize = "'", serialize = r#"""#)]
    SingleQuote,
    #[strum(serialize = "`", serialize = "~")]
    Tick,
    #[strum(serialize = "-", serialize = "_")]
    Minus,
    #[strum(serialize = "=", serialize = "+")]
    Equal,
    #[strum(serialize = "[", serialize = "{")]
    LBracket,
    #[strum(serialize = "]", serialize = "}")]
    RBracket,
    #[strum(serialize = "<BckSp>")]
    Backspace,
    #[strum(serialize = "<Enter>")]
    Enter,
    #[strum(serialize = "<LShft>", serialize = "<RShft>")]
    Shift,
    #[strum(serialize = "<LCtrl>", serialize = "<RCtrl>")]
    Ctrl,
    #[strum(serialize = "<LAlt>", serialize = "<RAlt>")]
    Alt,
    #[strum(serialize = "<Tab>")]
    Tab,
    #[strum(serialize = "<Esc>")]
    Esc,
    #[strum(serialize = "<Up>")]
    Up,
    #[strum(serialize = "<Down>")]
    Down,
    #[strum(serialize = "<Right>")]
    Right,
    #[strum(serialize = "<Left>")]
    Left,
    #[strum(serialize = "<Space>")]
    Space,
}
