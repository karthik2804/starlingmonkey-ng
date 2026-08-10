// SPDX-License-Identifier: Apache-2.0-WITH-LLVM-exception

//! Regression test for `#[jsglobals]` class registration via `pub use`.
//!
//! Only `pub use Foo;` (a plain name) was registered; a grouped
//! `pub use {A, B};` or a renamed `pub use Foo as Bar;` was silently skipped,
//! so the class was never installed on the global. All forms are now handled.

// This file contains nothing platform-specific, so skip it on wasm32.
#![cfg(not(target_arch = "wasm32"))]

use core_runtime::test_util::eval_with_setup;
use core_runtime::{jsclass, jsglobals, jsmethods};

#[jsclass]
struct Alpha {}

#[jsmethods]
impl Alpha {
    #[constructor]
    fn construct() -> Self {
        Self {}
    }

    #[getter]
    fn tag(&self) -> String {
        "alpha".to_string()
    }
}

#[jsclass]
struct Beta {}

#[jsmethods]
impl Beta {
    #[constructor]
    fn construct() -> Self {
        Self {}
    }

    #[getter]
    fn tag(&self) -> String {
        "beta".to_string()
    }
}

#[jsglobals]
mod grouped_globals {
    pub use super::{Alpha, Beta};
}

fn setup_globals(scope: &js::gc::scope::Scope<'_>, global: js::Object<'_>) {
    grouped_globals::add_to_global(scope, global);
}

#[test]
fn grouped_use_registers_all_classes() {
    assert_eq!(eval_with_setup(&[setup_globals], "typeof Alpha"), "function");
    assert_eq!(eval_with_setup(&[setup_globals], "typeof Beta"), "function");
    assert_eq!(eval_with_setup(&[setup_globals], "new Alpha().tag"), "alpha");
    assert_eq!(eval_with_setup(&[setup_globals], "new Beta().tag"), "beta");
}
