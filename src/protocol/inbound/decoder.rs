use serde_json::Value;

use crate::domain::account::state::{Player, Snapshot};
use crate::domain::econ::balance::Reward;
use crate::domain::world::map::MapView;
use crate::protocol::inbound::frames as f;

pub enum Event {
    Gate,
    Me,
    Welcome,
    Daily,
    Tasks,
    Wheel,
    WheelResult(Reward),
    Vip,
    VipClaim(Reward),
    Bosses,
    WorldBoss,
    Ads,
    BoxResult(Value),
    Reward(Reward),
    NeedAuto,
    Unlock(String),
    Toast(String),
    Banned(String),
    AuthFail,
    Offline(i64),
    Login(i64, String),
    QuestDone(String),
    DailyDone(String),
    TaskDone(Reward),
    Other(String),
}

pub fn apply(snap: &mut Snapshot, map: &mut MapView, frame: &Value) -> Vec<Event> {
    let kind = frame.get("t").and_then(|x| x.as_str()).unwrap_or("");
    let mut out = Vec::new();

    match kind {
        "welcome" => {
            let (uid, name) = f::parse_welcome(frame);
            snap.uid = uid;
            if !name.is_empty() {
                snap.name = name;
            }
            out.push(Event::Welcome);
        }
        "me" => {
            snap.player = merge_player(&snap.player, frame);
            out.push(Event::Me);
        }
        "gate" => {
            snap.gate = f::parse_gate(frame);
            out.push(Event::Gate);
        }
        "login" => {
            let day = frame.get("day").and_then(|x| x.as_i64()).unwrap_or(0);
            let reward = frame
                .get("reward")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string();
            out.push(Event::Login(day, reward));
        }
        "daily" => {
            snap.daily = f::parse_daily(frame);
            out.push(Event::Daily);
        }
        "dailydone" => {
            out.push(Event::DailyDone(f::parse_title(frame)));
        }
        "tasks" => {
            let mut state = f::parse_tasks(frame);
            state.pending = state.list.iter().filter(|t| !t.done).count() as i64;
            snap.tasks = state;
            out.push(Event::Tasks);
        }
        "taskcount" => {
            snap.tasks.pending = frame.get("n").and_then(|x| x.as_i64()).unwrap_or(0);
        }
        "taskdone" => {
            out.push(Event::TaskDone(f::parse_reward(frame, "task")));
        }
        "wheel" => {
            snap.wheel = f::parse_wheel(frame);
            out.push(Event::Wheel);
        }
        "wheelres" => {
            out.push(Event::WheelResult(f::parse_reward(frame, "wheel")));
        }
        "vip" => {
            snap.vip = f::parse_vip(frame);
            out.push(Event::Vip);
        }
        "vipclaim" => {
            out.push(Event::VipClaim(f::parse_reward(frame, "vip")));
        }
        "bosses" => {
            snap.bosses = f::parse_bosses(frame);
            out.push(Event::Bosses);
        }
        "bossend" => {
            out.push(Event::Reward(f::parse_reward(frame, "boss")));
        }
        "wb" => {
            snap.world_boss = f::parse_world_boss(frame);
            out.push(Event::WorldBoss);
        }
        "wbend" => {
            out.push(Event::Reward(f::parse_reward(frame, "world boss")));
        }
        "ads" => {
            snap.ads = f::parse_ads(frame);
            out.push(Event::Ads);
        }
        "adticket" | "monetag" | "monetagdone" => {
            out.push(Event::Other(kind.to_string()));
        }
        "boxres" => {
            out.push(Event::BoxResult(frame.clone()));
        }
        "inv" => {
            snap.box_ids = f::parse_inventory_boxes(frame);
            out.push(Event::Other("inv".to_string()));
        }
        "loot" => {
            out.push(Event::Reward(f::parse_reward(frame, "loot")));
        }
        "quest" => {
            out.push(Event::Other("quest".to_string()));
        }
        "questdone" => {
            out.push(Event::QuestDone(f::parse_title(frame)));
        }
        "map" => {
            map.apply(frame);
            snap.map_key = map.key.clone();
            snap.map_name = map.label();
            snap.in_boss_room = map.in_boss_room();
            out.push(Event::Other("map".to_string()));
        }
        "s" => {
            map.apply_state(frame);
        }
        "sys" => {
            out.push(Event::Toast(f::parse_message(frame)));
        }
        "unlock" => {
            out.push(Event::Unlock(f::parse_message(frame)));
        }
        "needauto" => {
            out.push(Event::NeedAuto);
        }
        "needpass" => {
            out.push(Event::Other("needpass".to_string()));
        }
        "offline" => {
            let ms = frame.get("ms").and_then(|x| x.as_i64()).unwrap_or(0);
            out.push(Event::Offline(ms));
        }
        "banned" => {
            snap.banned = true;
            let msg = f::parse_message(frame);
            out.push(Event::Banned(if msg.is_empty() {
                "multi account block".to_string()
            } else {
                msg
            }));
        }
        "auth_fail" => {
            snap.auth_failed = true;
            out.push(Event::AuthFail);
        }
        "prices" | "wbrank" | "wb_info" | "info" | "leave" | "top" | "referral"
        | "refshare" | "friends" | "friendcount" | "joinreq" | "clans" | "clan"
        | "clanchat" | "clanres" | "chat" | "chatlog" | "season" | "board"
        | "eqmarket" | "myorders" | "auctions" | "ton_order" | "invoice" | "paid"
        | "promores" | "trainres" | "awkres" | "fuseres" | "mkres" | "mkitem"
        | "upres" | "runeres" | "openwin" | "packdone" | "usdt" | "payrecent"
        | "offline_end" => {}
        other => {
            out.push(Event::Other(other.to_string()));
        }
    }

    out
}

fn merge_player(prev: &Player, frame: &Value) -> Player {
    let fresh = f::parse_me(frame);
    let mut next = prev.clone();
    if frame.get("lv").is_some() {
        next.level = fresh.level;
    }
    if frame.get("gold").is_some() {
        next.gold = fresh.gold;
    }
    if frame.get("gems").is_some() {
        next.gems = fresh.gems;
    }
    if frame.get("coins").is_some() {
        next.coins = fresh.coins;
    }
    if frame.get("hp").is_some() {
        next.hp = fresh.hp;
    }
    if frame.get("max").is_some() {
        next.max_hp = fresh.max_hp;
    }
    if frame.get("xp").is_some() {
        next.xp = fresh.xp;
    }
    if frame.get("need").is_some() {
        next.need = fresh.need;
    }
    if frame.get("vip").is_some() {
        next.vip = fresh.vip;
    }
    if fresh.power != 0 {
        next.power = fresh.power;
    }
    next.seen = true;
    next
}
