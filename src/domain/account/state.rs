use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct DailyMission {
    pub title: String,
    pub done: bool,
    pub count: i64,
    pub need: i64,
}

#[derive(Clone, Debug, Default)]
pub struct DailyState {
    pub list: Vec<DailyMission>,
    pub bonus: bool,
    pub streak: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SideTask {
    pub id: Value,
    pub title: String,
    pub section: String,
    pub kind: String,
    pub level: i64,
    pub wait: i64,
    pub left: i64,
    pub done: bool,
    pub gold: i64,
    pub gems: i64,
}

#[derive(Clone, Debug, Default)]
pub struct TasksState {
    pub list: Vec<SideTask>,
    pub pending: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct WheelState {
    pub spins: i64,
    pub free: bool,
    pub segs: Vec<Value>,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct VipState {
    pub level: i64,
    pub points: i64,
    pub next: i64,
    pub daily: bool,
    pub wheel: i64,
    pub wheel_left: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct BossEntry {
    pub id: Value,
    pub name: String,
    pub level: i64,
    pub hp: i64,
    pub cooldown: i64,
    pub today: i64,
}

#[derive(Clone, Debug, Default)]
pub struct BossState {
    pub list: Vec<BossEntry>,
    pub max_per_day: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct WorldBossState {
    pub active: bool,
    pub joined: bool,
    pub name: String,
    pub join_until: i64,
    pub until: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AdState {
    pub need: i64,
    pub watched: i64,
    pub earned: i64,
    pub day_max: i64,
    pub free: bool,
    pub next_ms: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct GateState {
    pub ok: bool,
    pub channel: bool,
    pub group: bool,
    pub story: bool,
    pub bio: bool,
    pub nick: bool,
    pub asked: bool,
    pub reason: Option<String>,
    pub links: Vec<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub level: i64,
    pub gold: i64,
    pub gems: i64,
    pub coins: i64,
    pub hp: i64,
    pub max_hp: i64,
    pub xp: i64,
    pub need: i64,
    pub power: i64,
    pub vip: i64,
    pub seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Snapshot {
    pub uid: i64,
    pub name: String,
    pub player: Player,
    pub gate: GateState,
    pub daily: DailyState,
    pub tasks: TasksState,
    pub wheel: WheelState,
    pub vip: VipState,
    pub bosses: BossState,
    pub world_boss: WorldBossState,
    pub ads: AdState,
    pub map_key: String,
    pub map_name: String,
    pub in_boss_room: bool,
    pub banned: bool,
    pub auth_failed: bool,
    pub box_ids: Vec<Value>,
    pub last_error: String,
}

impl Snapshot {
    pub fn ready(&self) -> bool {
        self.player.seen && self.gate.ok
    }

    pub fn level(&self) -> i64 {
        self.player.level
    }

    pub fn open_boxes(&self) -> Vec<Value> {
        self.box_ids.clone()
    }

    pub fn clear_boxes(&mut self) {
        self.box_ids.clear();
    }

    pub fn claimable_tasks(&self) -> Vec<Value> {
        self.tasks
            .list
            .iter()
            .filter(|t| !t.done && t.left <= 0 && t.level <= self.player.level)
            .map(|t| t.id.clone())
            .collect()
    }

    pub fn finished_tasks(&self) -> i64 {
        self.tasks.list.iter().filter(|t| t.done).count() as i64
    }

    pub fn daily_left(&self) -> i64 {
        self.daily.list.iter().filter(|m| !m.done).count() as i64
    }

    pub fn summary(&self) -> String {
        format!(
            "lv {} gold {} gems {} coins {} vip {}",
            self.player.level,
            self.player.gold,
            self.player.gems,
            self.player.coins,
            self.player.vip
        )
    }
}
