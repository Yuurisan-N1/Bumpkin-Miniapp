use serde_json::{json, Value};

pub fn verify() -> Value {
    json!({ "t": "verify" })
}

pub fn prices() -> Value {
    json!({ "t": "prices" })
}

pub fn inventory() -> Value {
    json!({ "t": "inv" })
}

pub fn task_list() -> Value {
    json!({ "t": "task", "a": "list" })
}

pub fn task_open(id: &Value) -> Value {
    json!({ "t": "task", "a": "open", "id": id })
}

pub fn task_claim(id: &Value) -> Value {
    json!({ "t": "task", "a": "claim", "id": id })
}

pub fn wheel_state() -> Value {
    json!({ "t": "wheel", "a": "state" })
}

pub fn wheel_free() -> Value {
    json!({ "t": "wheel", "a": "free" })
}

pub fn wheel_vip() -> Value {
    json!({ "t": "wheel", "a": "vip" })
}

pub fn vip_state() -> Value {
    json!({ "t": "vip", "a": "state" })
}

pub fn vip_claim() -> Value {
    json!({ "t": "vip", "a": "claim" })
}

pub fn box_open(id: &Value) -> Value {
    json!({ "t": "box_open", "id": id })
}

pub fn bosses() -> Value {
    json!({ "t": "bosses" })
}

pub fn boss_enter(id: &Value) -> Value {
    json!({ "t": "boss_enter", "id": id })
}

pub fn boss_leave() -> Value {
    json!({ "t": "boss_leave" })
}

pub fn wb_info() -> Value {
    json!({ "t": "wb_info" })
}

pub fn wb_join() -> Value {
    json!({ "t": "wb_join" })
}

pub fn move_to(x: i64, y: i64) -> Value {
    json!({ "t": "move", "x": x, "y": y })
}

pub fn travel(to: &str) -> Value {
    json!({ "t": "travel", "to": to })
}

pub fn target(id: &Value) -> Value {
    json!({ "t": "target", "id": id })
}
