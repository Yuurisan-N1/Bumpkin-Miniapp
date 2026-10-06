use serde_json::Value;

use crate::domain::account::state::{
    AdState, BossEntry, BossState, DailyMission, DailyState, GateState, Player, SideTask,
    TasksState, VipState, WheelState, WorldBossState,
};

fn num(v: &Value, key: &str) -> i64 {
    v.get(key).and_then(|x| x.as_i64()).unwrap_or(0)
}

fn fnum(v: &Value, key: &str) -> i64 {
    match v.get(key) {
        Some(Value::Number(n)) => n.as_i64().unwrap_or(0),
        Some(Value::String(s)) => s.parse::<f64>().map(|f| f as i64).unwrap_or(0),
        _ => 0,
    }
}

fn text(v: &Value, key: &str) -> String {
    v.get(key)
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string()
}

fn flag(v: &Value, key: &str) -> bool {
    v.get(key).and_then(|x| x.as_bool()).unwrap_or(false)
}

pub fn parse_welcome(frame: &Value) -> (i64, String) {
    (num(frame, "uid"), text(frame, "name"))
}

pub fn parse_me(frame: &Value) -> Player {
    let power = frame
        .get("st")
        .and_then(|s| s.get("power"))
        .and_then(|p| p.as_i64())
        .unwrap_or(0);
    Player {
        level: num(frame, "lv"),
        gold: fnum(frame, "gold"),
        gems: fnum(frame, "gems"),
        coins: fnum(frame, "coins"),
        hp: fnum(frame, "hp"),
        max_hp: fnum(frame, "max"),
        xp: fnum(frame, "xp"),
        need: fnum(frame, "need"),
        power,
        vip: num(frame, "vip"),
        seen: true,
    }
}

pub fn parse_gate(frame: &Value) -> GateState {
    let mut links = Vec::new();
    if let Some(map) = frame.get("links").and_then(|x| x.as_object()) {
        for (_, v) in map {
            if let Some(s) = v.as_str() {
                links.push(s.to_string());
            }
        }
    }
    let reason = ["reason", "msg", "message", "err", "note"]
        .iter()
        .find_map(|k| frame.get(*k).and_then(|x| x.as_str()))
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    GateState {
        ok: flag(frame, "ok"),
        channel: flag(frame, "channel"),
        group: flag(frame, "group"),
        story: flag(frame, "story"),
        bio: flag(frame, "bio"),
        nick: flag(frame, "nick"),
        asked: false,
        reason,
        links,
    }
}

pub fn parse_daily(frame: &Value) -> DailyState {
    let mut list = Vec::new();
    if let Some(arr) = frame.get("list").and_then(|x| x.as_array()) {
        for item in arr {
            list.push(DailyMission {
                title: text(item, "t"),
                done: flag(item, "done"),
                count: fnum(item, "c"),
                need: fnum(item, "n"),
            });
        }
    }
    DailyState {
        list,
        bonus: flag(frame, "bonus"),
        streak: num(frame, "streak"),
        seen: true,
    }
}

pub fn parse_tasks(frame: &Value) -> TasksState {
    let mut list = Vec::new();
    if let Some(arr) = frame.get("list").and_then(|x| x.as_array()) {
        for item in arr {
            list.push(SideTask {
                id: item.get("id").cloned().unwrap_or(Value::Null),
                title: text(item, "title"),
                section: text(item, "sec"),
                kind: text(item, "kind"),
                level: num(item, "lv"),
                wait: num(item, "wait"),
                left: num(item, "left"),
                done: flag(item, "done"),
                gold: fnum(item, "gold"),
                gems: fnum(item, "gems"),
            });
        }
    }
    TasksState {
        list,
        pending: 0,
        seen: true,
    }
}

pub fn parse_wheel(frame: &Value) -> WheelState {
    WheelState {
        spins: num(frame, "spins"),
        free: flag(frame, "free"),
        segs: frame
            .get("segs")
            .and_then(|x| x.as_array())
            .cloned()
            .unwrap_or_default(),
        seen: true,
    }
}

pub fn parse_vip(frame: &Value) -> VipState {
    VipState {
        level: num(frame, "lv"),
        points: fnum(frame, "pts"),
        next: fnum(frame, "next"),
        daily: flag(frame, "daily"),
        wheel: num(frame, "wheel"),
        wheel_left: num(frame, "wheelLeft"),
        seen: true,
    }
}

pub fn parse_bosses(frame: &Value) -> BossState {
    let mut list = Vec::new();
    if let Some(arr) = frame.get("list").and_then(|x| x.as_array()) {
        for item in arr {
            list.push(BossEntry {
                id: item.get("id").cloned().unwrap_or(Value::Null),
                name: text(item, "n"),
                level: num(item, "lv"),
                hp: fnum(item, "hp"),
                cooldown: num(item, "cd"),
                today: num(item, "today"),
            });
        }
    }
    BossState {
        list,
        max_per_day: num(frame, "max").max(1),
        seen: true,
    }
}

pub fn parse_world_boss(frame: &Value) -> WorldBossState {
    WorldBossState {
        active: flag(frame, "active"),
        joined: flag(frame, "joined"),
        name: text(frame, "n"),
        join_until: num(frame, "joinUntil"),
        until: num(frame, "until"),
        seen: true,
    }
}

pub fn parse_ads(frame: &Value) -> AdState {
    AdState {
        need: num(frame, "need"),
        watched: num(frame, "n"),
        earned: num(frame, "earned"),
        day_max: num(frame, "dayMax"),
        free: flag(frame, "free"),
        next_ms: num(frame, "next"),
        seen: true,
    }
}

pub fn parse_inventory_boxes(frame: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    if let Some(items) = frame
        .get("items")
        .and_then(|x| x.as_array())
        .or_else(|| frame.get("list").and_then(|x| x.as_array()))
    {
        for item in items {
            if text(item, "kind") == "box" {
                if let Some(id) = item.get("id") {
                    if !id.is_null() {
                        out.push(id.clone());
                    }
                }
            }
        }
    }
    out
}

pub fn parse_reward(frame: &Value, label: &str) -> crate::domain::econ::balance::Reward {
    let mut r = crate::domain::econ::balance::Reward {
        label: label.to_string(),
        ..Default::default()
    };
    if let Some(d) = frame.get("reward") {
        r.gold = fnum(d, "gold");
        r.gems = fnum(d, "gems");
        r.coins = fnum(d, "coins");
    }
    if r.is_empty() {
        r.gold = fnum(frame, "gold");
        r.gems = fnum(frame, "gems");
        r.coins = fnum(frame, "coins");
    }
    r
}

pub fn parse_title(frame: &Value) -> String {
    text(frame, "title")
}

pub fn parse_message(frame: &Value) -> String {
    let msg = text(frame, "msg");
    if msg.is_empty() {
        text(frame, "error")
    } else {
        msg
    }
}
