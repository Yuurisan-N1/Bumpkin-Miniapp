use crate::domain::account::state::Snapshot;

#[derive(Clone, Debug, Default)]
pub struct Wallet {
    pub gold: i64,
    pub gems: i64,
    pub coins: i64,
    pub vip_points: i64,
    pub first_seen: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Reward {
    pub gold: i64,
    pub gems: i64,
    pub coins: i64,
    pub label: String,
}

impl Reward {
    pub fn is_empty(&self) -> bool {
        self.gold == 0 && self.gems == 0 && self.coins == 0
    }

    pub fn line(&self) -> String {
        let mut parts: Vec<String> = Vec::new();
        if self.gold != 0 {
            parts.push(format!("{} gold", self.gold));
        }
        if self.gems != 0 {
            parts.push(format!("{} gems", self.gems));
        }
        if self.coins != 0 {
            parts.push(format!("{} coins", self.coins));
        }
        if parts.is_empty() {
            self.label.clone()
        } else {
            parts.join(" ")
        }
    }
}

pub struct Tracker {
    wallet: Wallet,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            wallet: Wallet::default(),
        }
    }

    pub fn sync(&mut self, snap: &Snapshot) -> Reward {
        let mut r = Reward::default();
        if !self.wallet.first_seen {
            self.wallet.first_seen = true;
            self.wallet.gold = snap.player.gold;
            self.wallet.gems = snap.player.gems;
            self.wallet.coins = snap.player.coins;
            self.wallet.vip_points = snap.vip.points;
            return r;
        }
        if snap.player.gold > self.wallet.gold {
            r.gold = snap.player.gold - self.wallet.gold;
        }
        if snap.player.gems > self.wallet.gems {
            r.gems = snap.player.gems - self.wallet.gems;
        }
        if snap.player.coins > self.wallet.coins {
            r.coins = snap.player.coins - self.wallet.coins;
        }
        self.wallet.gold = snap.player.gold;
        self.wallet.gems = snap.player.gems;
        self.wallet.coins = snap.player.coins;
        self.wallet.vip_points = snap.vip.points;
        r
    }

    pub fn wallet(&self) -> &Wallet {
        &self.wallet
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}
