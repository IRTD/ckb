use crate::keys::Key;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
pub struct Keyboard {
    rows: Vec<Row>,
}

impl Keyboard {
    pub fn from_layout(s: impl ToString) -> anyhow::Result<Self> {
        let mut rows = Vec::new();
        let top_down_weights = vec![1.0, 0.3, 0.0, 0.5, 0.7];
        for (line, weight) in s.to_string().split("::").zip(top_down_weights) {
            rows.push(Row::from_layout(line, weight)?)
        }
        Ok(Keyboard { rows })
    }

    pub fn judge(&self, text: impl ToString) -> anyhow::Result<f32> {
        let mut res = 0.0;
        for c in text.to_string().split("") {
            if c.is_empty() || c == " " {
                continue;
            }
            let key = Key::from_str(c)?;
            for (row_i, row) in self.rows.iter().enumerate() {
                match row.map.get(&key) {
                    Some(btn) => {
                        let (closest, dist) = match btn.finger {
                            Some(_) => continue,
                            None => self.find_closest_finger(&btn, row_i),
                        };
                        res += dist * (row.weight + closest.finger.as_ref().unwrap().weight);
                    }
                    None => {}
                }
            }
        }

        Ok(res)
    }

    pub fn find_closest_finger<'a>(&'a self, btn: &Button, row_idx: usize) -> (&'a Button, f32) {
        let mut closest: Option<(&Button, f32)> = None;
        for (i, row) in self.rows.iter().enumerate() {
            for (_, button) in &row.map {
                if button.finger.is_none() {
                    continue;
                }
                let btn_distance = ((button.idx as f32).max(btn.idx as f32)
                    - (button.idx as f32).min(btn.idx as f32))
                    + self.rows[row_idx].weight;
                match closest {
                    Some((_, dist)) if dist > btn_distance => {
                        closest = Some((button, btn_distance))
                    }
                    None => closest = Some((button, btn_distance)),
                    _ => {}
                }
            }
        }
        closest.expect("No Finger Position on Keyboard")
    }
}

#[derive(Debug)]
pub struct Row {
    map: HashMap<Key, Button>,
    weight: f32,
}

impl Row {
    pub fn from_layout(row: &str, weight: f32) -> anyhow::Result<Self> {
        let mut map = HashMap::new();
        for (i, line) in row.lines().enumerate() {
            if line.is_empty() {
                continue;
            }
            if !line.contains(":") {
                let key = Key::from_str(line.trim())?;
                let button = Button {
                    idx: i,
                    finger: None,
                };
                map.insert(key, button);
                continue;
            }
            let (key_str, finger) = line.split_once(":").unwrap();
            let key = Key::from_str(key_str)?;
            let button = Button {
                idx: i,
                finger: Some(Finger::try_from(finger.to_string())?),
            };
            map.insert(key, button);
        }
        Ok(Row { map, weight })
    }
}

#[derive(Debug)]
pub struct Button {
    pub idx: usize,
    pub finger: Option<Finger>,
}

#[derive(Debug)]
pub struct Finger {
    kind: FingerKind,
    pub weight: f32,
}

impl TryFrom<String> for Finger {
    type Error = anyhow::Error;
    fn try_from(value: String) -> anyhow::Result<Self> {
        let kind = FingerKind::from_str(&value)?;
        let weight = match kind {
            FingerKind::Index => 0.0,
            FingerKind::Middle => 0.2,
            FingerKind::Ring => 0.5,
            FingerKind::Pinky => 0.8,
            FingerKind::Thumb => 1.0,
        };
        Ok(Finger { kind, weight })
    }
}

#[derive(Debug, strum::EnumString)]
pub enum FingerKind {
    Thumb,
    Ring,
    Middle,
    Index,
    Pinky,
}
