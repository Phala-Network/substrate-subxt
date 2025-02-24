// Copyright 2019-2025 Parity Technologies (UK) Ltd.
// This file is dual-licensed as Apache-2.0 or GPL-3.0.
// see LICENSE for license details.

/// Checks that code generated by `subxt-cli codegen` compiles. Allows inspection of compiler errors
/// directly, more accurately than via the macro and `cargo expand`.
///
/// Generate by running this at the root of the repository:
///
/// ```
/// cargo run --bin subxt -- codegen --file artifacts/polkadot_metadata_full.scale | rustfmt > testing/integration-tests/src/full_client/codegen/polkadot.rs
/// ```
#[rustfmt::skip]
#[allow(clippy::all)]
mod polkadot;

mod documentation;
