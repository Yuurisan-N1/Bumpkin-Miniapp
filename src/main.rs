#![allow(dead_code)]

mod domain;
mod net;
mod ops;
mod protocol;
mod support;

use anyhow::Result;

use domain::account::session::load_accounts;
use net::client::egress::ProxyPool;
use ops::runner::Engine;
use ops::Step;
use support::cfg::Config;
use support::consts::{CONFIG_PATH, DATA_PATH, PROXY_PATH};
use support::log::{clock, countdown, ld, lg, lr, ly, sanitize};

#[tokio::main]
async fn main() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    tokio::select! {
        result = run() => {
            if let Err(e) = result {
                lr(&format!(
                    "Bot ended due to unexpected error: {}",
                    sanitize(&e.to_string())
                ));
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!();
            lr("Script stopped by user");
        }
    }
}

async fn run() -> Result<()> {
    support::banner::print_banner();

    let cfg = match Config::load(CONFIG_PATH) {
        Ok(c) => c,
        Err(e) => {
            ly(&format!("Config file could not be loaded because {}", sanitize(&e.to_string())));
            return Ok(());
        }
    };

    let accounts = match load_accounts(DATA_PATH) {
        Ok(a) => a,
        Err(e) => {
            lr(&format!("File data.txt is empty and holds no initData string, {}", sanitize(&e.to_string())));
            return Ok(());
        }
    };

    let mut pool = ProxyPool::load_from(PROXY_PATH)?;
    ly(&format!("Using proxy {}", pool.masked()));

    lg(&format!(
        "Cycle every {} with the loop {}",
        clock(cfg.sleep_seconds()),
        if cfg.loop_enabled() { "on" } else { "off single run" }
    ));

    let max_cycles = cfg.max_cycles();
    let mut cycle: i64 = 0;

    loop {
        cycle += 1;
        ly(&format!("Starting automation cycle number {}", cycle));
        let cycle_started = std::time::Instant::now();

        for (idx, acc) in accounts.iter().enumerate() {
            if idx > 0 {
                println!();
            }
            let proxy = pool.next();
            if let Err(e) = run_account(&cfg, acc.clone(), proxy).await {
                lr(&format!(
                    "Sign in failed for account {} because {}",
                    acc.label(),
                    sanitize(&e.to_string())
                ));
            }
        }

        lg(&format!(
            "Automation cycle number {} is complete and all accounts were processed",
            cycle
        ));

        if max_cycles > 0 && cycle >= max_cycles {
            lg(&format!("The configured limit of {} cycles was reached and the bot stopped", max_cycles));
            break;
        }
        if !cfg.loop_enabled() {
            lg("Cycle loop is off, exiting");
            break;
        }

        let budget = cfg.sleep_seconds() as u64;
        let used = cycle_started.elapsed().as_secs();
        if used < budget {
            countdown(budget - used, "Next cycle starts in").await;
        }
        support::banner::print_banner();
    }

    Ok(())
}

async fn run_account(
    cfg: &Config,
    acc: domain::account::session::Account,
    proxy: net::client::egress::ProxyEntry,
) -> Result<()> {
    let mut eng = Engine::connect(cfg.clone(), acc, proxy, false).await?;

    lg(&format!(
        "Account {} signed in successfully with a device identity pinned on {}",
        eng.label(),
        eng.acc.dev_short()
    ));

    match ops::sys::bootstrap::run(&mut eng).await {
        Step::Done(n) => lg(&n),
        Step::Idle(n) => ld(&n),
        Step::Failed(n) => {
            ly(&n);
            eng.shutdown().await;
            return Ok(());
        }
    }

    match ops::sys::gate::run(&mut eng).await {
        Step::Done(n) => lg(&n),
        Step::Idle(n) => ld(&n),
        Step::Failed(n) => {
            ly(&n);
            eng.shutdown().await;
            return Ok(());
        }
    }

    run_cycle(&mut eng).await;

    eng.log_state();

    let hold = cfg.hold_seconds();
    match ops::sys::presence::hold(&mut eng, hold).await {
        Step::Done(n) => lg(&n),
        Step::Idle(n) => ld(&n),
        Step::Failed(n) => ly(&n),
    }

    lg(&format!("Account totals {}", eng.report_line()));
    eng.shutdown().await;
    Ok(())
}

async fn run_cycle(eng: &mut Engine) {
    report(ops::features::daily::run(eng).await);
    report(ops::features::vip::run(eng).await);
    report(ops::features::tasks::run(eng).await);
    report(ops::features::r#box::run(eng).await);
    report(ops::features::wheel::run(eng).await);
    report(ops::features::quest::run(eng).await);
    report(ops::features::boss::run(eng).await);

    eng.sync_if_stale().await;
}

fn report(step: Step) {
    match step {
        Step::Done(n) => lg(&n),
        Step::Idle(n) => lg(&n),
        Step::Failed(n) => ly(&n),
    }
}
