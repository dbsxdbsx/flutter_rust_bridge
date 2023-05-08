#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.75.2.

use flutter_rust_bridge::*;

// Section: imports

pub use crate::custom::CrossSharedStructInBlock1And2;
pub use crate::custom::CrossSharedStructInBlock2And3;
pub use crate::custom::SharedStructInAllBlocks;
pub use crate::custom::SharedStructInBlock1And2;
pub use crate::custom::SharedStructInBlock2And3;
pub use crate::custom::SharedStructOnlyForSyncTest;

// Section: wire functions

// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<f32> for f32 {
    fn wire2api(self) -> f32 {
        self
    }
}
impl Wire2Api<f64> for f64 {
    fn wire2api(self) -> f64 {
        self
    }
}
impl Wire2Api<i32> for i32 {
    fn wire2api(self) -> i32 {
        self
    }
}

impl Wire2Api<u64> for u64 {
    fn wire2api(self) -> u64 {
        self
    }
}
impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for CrossSharedStructInBlock1And2 {
    fn into_dart(self) -> support::DartAbi {
        vec![self.name.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for CrossSharedStructInBlock1And2 {}

impl support::IntoDart for CrossSharedStructInBlock2And3 {
    fn into_dart(self) -> support::DartAbi {
        vec![self.name.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for CrossSharedStructInBlock2And3 {}

impl support::IntoDart for SharedStructInAllBlocks {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SharedStructInAllBlocks {}

impl support::IntoDart for SharedStructInBlock1And2 {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SharedStructInBlock1And2 {}

impl support::IntoDart for SharedStructInBlock2And3 {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SharedStructInBlock2And3 {}

impl support::IntoDart for SharedStructOnlyForSyncTest {
    fn into_dart(self) -> support::DartAbi {
        vec![self.name.into_dart(), self.score.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for SharedStructOnlyForSyncTest {}

// Section: executor

/* nothing since executor detected */

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated_shares.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated_shares.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
