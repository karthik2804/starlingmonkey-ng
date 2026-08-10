// SPDX-License-Identifier: Apache-2.0-WITH-LLVM-exception

//! StarlingMonkey — a JavaScript runtime built on SpiderMonkey.
//!
//! Parses command-line arguments into a [`RuntimeConfig`](libstarling::config::RuntimeConfig)
//! and delegates execution to [`libstarling::run`].

#[cfg(not(target_arch = "wasm32"))]
use std::process::exit;

#[cfg(not(target_arch = "wasm32"))]
use libstarling::config::RuntimeConfig;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let config = match RuntimeConfig::from_args(std::env::args()) {
        Ok(config) => config,
        Err(e) => {
            let _ = e.print();
            exit(0);
        }
    };

    let extra: &[libstarling::runtime::GlobalInitFn] = if config.wpt_mode {
        &[wpt_support::add_to_global]
    } else {
        &[]
    };
    let _ = libstarling::run(config, extra).map_err(|e| println!("{e}"));
}

#[cfg(target_arch = "wasm32")]
mod wasm_entry {

    use libstarling::config::RuntimeConfig;

    struct StarlingCli;

    impl wasip3::exports::cli::run::Guest for StarlingCli {
        async fn run() -> Result<(), ()> {
            let config = match RuntimeConfig::from_args(std::env::args()) {
                Ok(config) => config,
                Err(e) => {
                    let _ = e.print();
                    return Err(());
                }
            };

            let extra: &[libstarling::runtime::GlobalInitFn] = if config.wpt_mode {
                &[wpt_support::add_to_global]
            } else {
                &[]
            };

            libstarling::run(config, extra).await.map_err(|e| {
                eprintln!("{e}");
            })
        }
    }

    wasip3::cli::command::export!(StarlingCli);
}

#[cfg(target_arch = "wasm32")]
fn main() {
    unreachable!("On wasm32-wasip3, the StarlingCli impl above is used as the entry point");
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[test]
fn cli_runs() {
    let config = libstarling::config::RuntimeConfig::from_args(
        ["starling", "-e", "1 + 1"].iter().map(|s| s.to_string()),
    )
    .unwrap();
    libstarling::run(config, &[])
        .map_err(|e| println!("{e}"))
        .expect("Run failed");
}
