
//! Autogenerated weights for `pallet_kitties_v2`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-27, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/node-template
// benchmark
// pallet
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_kitties_v2
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --json-file=raw.json
// --output
// ./pallets/kitties-v2/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_kitties_v2`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_kitties_v2::WeightInfo for WeightInfo<T> {
	// Storage: KittiesModuleV2 Nonce (r:1 w:1)
	// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: KittiesModuleV2 Kitties (r:1 w:1)
	// Storage: KittiesModuleV2 KittyId (r:1 w:1)
	// Storage: KittiesModuleV2 KittiesOwned (r:1 w:1)
	fn create_kitty() -> Weight {
		(31_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: KittiesModuleV2 Kitties (r:1 w:1)
	// Storage: KittiesModuleV2 KittiesOwned (r:2 w:2)
	fn transfer() -> Weight {
		(21_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
