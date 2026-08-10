// SPDX-License-Identifier: Apache-2.0-WITH-LLVM-exception

//! Shared test helpers for `core-runtime` in-crate tests.

use js::{conversion::FromJSVal, error::ExnThrown};

use crate::{
    config::RuntimeConfig,
    runtime::{GlobalInitFn, RuntimeBuilder},
};

/// Create a temp directory that works on both native and wasm targets.
///
/// On WASI, `std::env::temp_dir()` panics because there is no temp filesystem.
/// The wasmtime runner mounts the CWD, so we use `tempdir_in("/tmp")` instead.
/// The component runtime needs to be invoked with a `--dir=/tmp` option for
/// this to work.
pub fn test_tempdir() -> tempfile::TempDir {
    #[cfg(target_arch = "wasm32")]
    {
        tempfile::Builder::new()
            .tempdir_in("/tmp")
            .expect("failed to create temp dir")
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tempfile::tempdir().expect("failed to create temp dir")
    }
}

/// Create a runtime with the given global initializers, evaluate JS code,
/// and convert the result to a string.
pub fn eval_with_setup(initializers: &[GlobalInitFn], code: &str) -> String {
    let rt = RuntimeBuilder::default()
        .with_initializers(initializers)
        .init(&RuntimeConfig::default());
    let scope = rt.default_global();
    match js::compile::evaluate_with_filename(&scope, code, "test.js", 1) {
        Ok(val) => String::from_jsval(&scope, val, ()).unwrap(),
        Err(_) => panic!(
            "JS evaluation threw an exception: {:?}",
            ExnThrown::capture(&scope)
        ),
    }
}

/// Create a runtime with the given global initializers and check whether JS code throws.
pub fn throws_with_setup(initializers: &[GlobalInitFn], code: &str) -> bool {
    let rt = RuntimeBuilder::default()
        .with_initializers(initializers)
        .init(&RuntimeConfig::default());
    let scope = rt.default_global();
    js::compile::evaluate_with_filename(&scope, code, "test.js", 1).is_err()
}
