use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct MapView {
    pub key: String,
    pub name: String,
    pub boss_room: bool,
    pub boss_world: bool,
    pub boss_clan: bool,
    pub tile: i64,
    pub players: usize,
    pub mobs: usize,
    pub seen: bool,
}

impl MapView {
    pub fn apply(&mut self, frame: &Value) {
        self.seen = true;
        if let Some(k) = frame.get("wkey").and_then(|x| x.as_str()) {
            self.key = k.to_string();
        }
        if let Some(n) = frame.get("n").and_then(|x| x.as_str()) {
            self.name = n.to_string();
        }
        if let Some(t) = frame.get("t").and_then(|x| x.as_i64()) {
            self.tile = t;
        }
        match frame.get("boss") {
            Some(Value::Bool(false)) | None | Some(Value::Null) => {
                self.boss_room = false;
                self.boss_world = false;
                self.boss_clan = false;
            }
            Some(b) => {
                self.boss_room = true;
                self.boss_world = b.get("world").and_then(|x| x.as_bool()).unwrap_or(false);
                self.boss_clan = b.get("clan").and_then(|x| x.as_bool()).unwrap_or(false);
                if let Some(n) = b.get("n").and_then(|x| x.as_str()) {
                    self.name = n.to_string();
                }
            }
        }
    }

    pub fn apply_state(&mut self, frame: &Value) {
        self.seen = true;
        if let Some(list) = frame.get("p").and_then(|x| x.as_array()) {
            self.players = list.len();
        }
        if let Some(list) = frame.get("m").and_then(|x| x.as_array()) {
            self.mobs = list.len();
        }
    }

    pub fn label(&self) -> String {
        if self.name.is_empty() {
            self.key.clone()
        } else {
            self.name.clone()
        }
    }

    pub fn in_boss_room(&self) -> bool {
        self.boss_room
    }
}
