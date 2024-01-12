// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sync sse", "rustAsync sse", "sync moi", "rustAsync moi", "sync sse moi", "rustAsync sse moi"], "enableAll": true}

pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw};
use anyhow::Result;
#[allow(unused_imports)]
use flutter_rust_bridge::{frb, RustOpaque};

pub struct HideDataAnotherTwinNormal(HideDataRaw);
pub struct NonCloneDataAnotherTwinNormal(NonCloneDataRaw);

/// Structure for testing the SyncReturn<RustOpaque> code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must should be without wrapper like Option<> Vec<> etc.
pub struct FrbOpaqueSyncReturnTwinNormal;

#[frb(sync)]
pub fn sync_option_rust_opaque_twin_normal() -> Result<Option<RustOpaque<HideDataAnotherTwinNormal>>>
{
    Ok(Some(RustOpaque::new(HideDataAnotherTwinNormal(
        HideDataRaw::new(),
    ))))
}

#[frb(sync)]
pub fn sync_create_opaque_twin_normal() -> RustOpaque<HideDataAnotherTwinNormal> {
    RustOpaque::new(HideDataAnotherTwinNormal(HideDataRaw::new()))
}

#[frb(sync)]
pub fn sync_create_non_clone_twin_normal() -> RustOpaque<NonCloneDataAnotherTwinNormal> {
    RustOpaque::new(NonCloneDataAnotherTwinNormal(NonCloneDataRaw::new()))
}

// OpaqueSyncStruct does not implement Send trait.
//
// pub fn run_opaque_twin_normal(opaque: Opaque<OpaqueSyncStruct>) -> String {
//     data.0.hide_data()
// }

/// Structure for testing the sync-mode RustOpaque code generator.
/// FrbOpaqueSyncReturn must be only return type.
/// FrbOpaqueSyncReturn must be without wrapper like Option<> Vec<> etc.
#[frb(sync)]
pub fn frb_sync_generator_test_twin_normal() -> RustOpaque<FrbOpaqueSyncReturnTwinNormal> {
    RustOpaque::new(FrbOpaqueSyncReturnTwinNormal)
}
