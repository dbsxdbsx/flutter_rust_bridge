// Section: imports

use super::*;
use crate::api::pseudo_manual::rust_opaque_twin_sync::*;
use crate::api::rust_opaque::*;
use crate::api::rust_opaque_sync::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::Handler;

// Section: impl_wire2api

impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::microseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<flutter_rust_bridge::DartOpaque> for wire_DartOpaque {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        unsafe { flutter_rust_bridge::DartOpaque::new(self.handle as _, self.port) }
    }
}
impl Wire2Api<[flutter_rust_bridge::DartOpaque; 1]> for *mut wire_list_DartOpaque {
    fn wire2api(self) -> [flutter_rust_bridge::DartOpaque; 1] {
        let vec: Vec<flutter_rust_bridge::DartOpaque> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Mutex<HideData>>> for wire_RustOpaque_MutexHideData {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Mutex<HideData>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<RwLock<HideData>>>
    for wire_RustOpaque_RwLockHideData
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<RwLock<HideData>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinNormal>>>
    for wire_RustOpaque_box_dynDartDebugTwinNormal
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinNormal>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinSync>>>
    for wire_RustOpaque_box_dynDartDebugTwinSync
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinSync>> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for wire_RustOpaque_hide_data
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<[flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2]>
    for *mut wire_list_RustOpaque_hide_data
{
    fn wire2api(
        self,
    ) -> [flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2] {
        let vec: Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> =
            self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<i32>> for wire_RustOpaque_i_32 {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<i32> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData>>
    for wire_RustOpaque_non_clone_data
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData>>
    for wire_RustOpaque_non_send_hide_data
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData> {
        unsafe { flutter_rust_bridge::support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<Vec<String>> for *mut wire_StringList {
    fn wire2api(self) -> Vec<String> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<uuid::Uuid> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuids(multiple)
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::ATwinNormal> for wire_a_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::ATwinNormal {
        crate::api::misc_example::ATwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync> for wire_a_twin_sync {
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::AbcTwinNormal> for wire_abc_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::AbcTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::misc_example::AbcTwinNormal::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::misc_example::AbcTwinNormal::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::misc_example::AbcTwinNormal::C(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.JustInt);
                crate::api::misc_example::AbcTwinNormal::JustInt(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync>
    for wire_abc_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::A(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::B(
                    ans.field0.wire2api(),
                )
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::C(
                    ans.field0.wire2api(),
                )
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.JustInt);
                crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::JustInt(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>
    for wire_application_env
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv {
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv {
            vars: self.vars.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar>
    for wire_application_env_var
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar {
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar(
            self.field0.wire2api(),
            self.field1.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings>
    for wire_application_settings
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings {
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings {
            name: self.name.wire2api(),
            version: self.version.wire2api(),
            mode: self.mode.wire2api(),
            env: self.env.wire2api(),
            env_optional: self.env_optional.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::AttributeTwinNormal> for wire_attribute_twin_normal {
    fn wire2api(self) -> crate::api::optional::AttributeTwinNormal {
        crate::api::optional::AttributeTwinNormal {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>
    for wire_attribute_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync {
        crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync {
            key: self.key.wire2api(),
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::BTwinNormal> for wire_b_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::BTwinNormal {
        crate::api::misc_example::BTwinNormal {
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync> for wire_b_twin_sync {
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync {
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::BlobTwinNormal> for wire_blob_twin_normal {
    fn wire2api(self) -> crate::api::array::BlobTwinNormal {
        crate::api::array::BlobTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync> for wire_blob_twin_sync {
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::BlobTwinSync {
        crate::api::pseudo_manual::array_twin_sync::BlobTwinSync(self.field0.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>>
    for *mut wire_application_env
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for *mut i64 {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<chrono::DateTime<chrono::Utc>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<flutter_rust_bridge::DartOpaque> for *mut wire_DartOpaque {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<flutter_rust_bridge::DartOpaque>::wire2api(*wrap).into()
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for *mut wire_RustOpaque_hide_data
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::ATwinNormal> for *mut wire_a_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::ATwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::ATwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync>
    for *mut wire_a_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::misc_example::AbcTwinNormal> for *mut wire_abc_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::AbcTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::AbcTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync>
    for *mut wire_abc_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>
    for *mut wire_application_env
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings>
    for *mut wire_application_settings
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::optional::AttributeTwinNormal> for *mut wire_attribute_twin_normal {
    fn wire2api(self) -> crate::api::optional::AttributeTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::AttributeTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>
    for *mut wire_attribute_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::misc_example::BTwinNormal> for *mut wire_b_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::BTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::BTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync>
    for *mut wire_b_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<bool> for *mut bool {
    fn wire2api(self) -> bool {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::misc_example::CTwinNormal> for *mut wire_c_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::CTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::CTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync>
    for *mut wire_c_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::method::ConcatenateWithTwinNormal>
    for *mut wire_concatenate_with_twin_normal
{
    fn wire2api(self) -> crate::api::method::ConcatenateWithTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::method::ConcatenateWithTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync>
    for *mut wire_concatenate_with_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal>
    for *mut wire_custom_nested_error_inner_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomNestedErrorInnerTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for *mut wire_custom_nested_error_inner_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal>
    for *mut wire_custom_nested_error_outer_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomNestedErrorOuterTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for *mut wire_custom_nested_error_outer_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal>
    for *mut wire_custom_struct_error_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomStructErrorTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for *mut wire_custom_struct_error_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::CustomStructTwinNormal>
    for *mut wire_custom_struct_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomStructTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::CustomStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync>
    for *mut wire_custom_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::attribute::CustomizedTwinNormal> for *mut wire_customized_twin_normal {
    fn wire2api(self) -> crate::api::attribute::CustomizedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::attribute::CustomizedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync>
    for *mut wire_customized_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNestedTwinNormal>
    for *mut wire_dart_opaque_nested_twin_normal
{
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNestedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::dart_opaque::DartOpaqueNestedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync>
    for *mut wire_dart_opaque_nested_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_type::EmptyTwinNormal> for *mut wire_empty_twin_normal {
    fn wire2api(self) -> crate::api::misc_type::EmptyTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_type::EmptyTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync>
    for *mut wire_empty_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaqueTwinNormal>
    for *mut wire_enum_dart_opaque_twin_normal
{
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaqueTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::dart_opaque::EnumDartOpaqueTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync>
    for *mut wire_enum_dart_opaque_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaqueTwinNormal> for *mut wire_enum_opaque_twin_normal {
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaqueTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::rust_opaque::EnumOpaqueTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync>
    for *mut wire_enum_opaque_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal>
    for *mut wire_enum_with_item_mixed_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemMixedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for *mut wire_enum_with_item_mixed_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal>
    for *mut wire_enum_with_item_struct_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for *mut wire_enum_with_item_struct_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal>
    for *mut wire_enum_with_item_tuple_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::EnumWithItemTupleTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for *mut wire_enum_with_item_tuple_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::event_listener::EventTwinNormal> for *mut wire_event_twin_normal {
    fn wire2api(self) -> crate::api::event_listener::EventTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::event_listener::EventTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync>
    for *mut wire_event_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionalsTwinNormal>
    for *mut wire_exotic_optionals_twin_normal
{
    fn wire2api(self) -> crate::api::optional::ExoticOptionalsTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::ExoticOptionalsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>
    for *mut wire_exotic_optionals_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<f64> for *mut f64 {
    fn wire2api(self) -> f64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChronoTwinNormal>
    for *mut wire_feature_chrono_twin_normal
{
    fn wire2api(self) -> crate::api::chrono_type::FeatureChronoTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::chrono_type::FeatureChronoTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync>
    for *mut wire_feature_chrono_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuidTwinNormal> for *mut wire_feature_uuid_twin_normal {
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuidTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::uuid_type::FeatureUuidTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync>
    for *mut wire_feature_uuid_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::array::FeedIdTwinNormal> for *mut wire_feed_id_twin_normal {
    fn wire2api(self) -> crate::api::array::FeedIdTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::FeedIdTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync>
    for *mut wire_feed_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<i16> for *mut i16 {
    fn wire2api(self) -> i16 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i32> for *mut i32 {
    fn wire2api(self) -> i32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i64> for *mut i64 {
    fn wire2api(self) -> i64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<i8> for *mut i8 {
    fn wire2api(self) -> i8 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::enumeration::KitchenSinkTwinNormal>
    for *mut wire_kitchen_sink_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::KitchenSinkTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::KitchenSinkTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>
    for *mut wire_kitchen_sink_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct> for *mut wire_macro_struct {
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::inside_macro::MacroStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::MeasureTwinNormal> for *mut wire_measure_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::MeasureTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::MeasureTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync>
    for *mut wire_measure_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::array::MessageIdTwinNormal> for *mut wire_message_id_twin_normal {
    fn wire2api(self) -> crate::api::array::MessageIdTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::MessageIdTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync>
    for *mut wire_message_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStructTwinNormal>
    for *mut wire_my_nested_struct_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::MyNestedStructTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MyNestedStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync>
    for *mut wire_my_nested_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MySize> for *mut wire_my_size {
    fn wire2api(self) -> crate::auxiliary::sample_types::MySize {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::auxiliary::sample_types::MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::MySizeFreezedTwinNormal>
    for *mut wire_my_size_freezed_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::MySizeFreezedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MySizeFreezedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>
    for *mut wire_my_size_freezed_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct> for *mut wire_my_struct {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::auxiliary::sample_types::MyStruct>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNodeTwinNormal>
    for *mut wire_my_tree_node_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::MyTreeNodeTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MyTreeNodeTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>
    for *mut wire_my_tree_node_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeIntTwinNormal>
    for *mut wire_new_type_int_twin_normal
{
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeIntTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::newtype_pattern::NewTypeIntTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync>
    for *mut wire_new_type_int_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::enumeration::NoteTwinNormal> for *mut wire_note_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::NoteTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::NoteTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync>
    for *mut wire_note_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Numbers> for *mut wire_numbers {
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Numbers {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::mirror_twin_sync::Numbers>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNestedTwinNormal>
    for *mut wire_opaque_nested_twin_normal
{
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNestedTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::rust_opaque::OpaqueNestedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync>
    for *mut wire_opaque_nested_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::optional::OptVecsTwinNormal> for *mut wire_opt_vecs_twin_normal {
    fn wire2api(self) -> crate::api::optional::OptVecsTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::OptVecsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync>
    for *mut wire_opt_vecs_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<(String, i32)> for *mut wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<(String, i32)>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Sequences> for *mut wire_sequences {
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Sequences {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::mirror_twin_sync::Sequences>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::exception::SomeStructTwinNormal> for *mut wire_some_struct_twin_normal {
    fn wire2api(self) -> crate::api::exception::SomeStructTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::exception::SomeStructTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync>
    for *mut wire_some_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal>
    for *mut wire_struct_with_comments_twin_normal
{
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::comment::StructWithCommentsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for *mut wire_struct_with_comments_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnumTwinNormal>
    for *mut wire_struct_with_enum_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::StructWithEnumTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::StructWithEnumTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync>
    for *mut wire_struct_with_enum_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal>
    for *mut wire_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for *mut wire_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal>
    for *mut wire_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for *mut wire_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal>
    for *mut wire_struct_with_zero_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::StructWithZeroFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for *mut wire_struct_with_zero_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::method::SumWithTwinNormal> for *mut wire_sum_with_twin_normal {
    fn wire2api(self) -> crate::api::method::SumWithTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::method::SumWithTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync>
    for *mut wire_sum_with_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::array::TestIdTwinNormal> for *mut wire_test_id_twin_normal {
    fn wire2api(self) -> crate::api::array::TestIdTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::TestIdTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>
    for *mut wire_test_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal>
    for *mut wire_tuple_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::TupleStructWithOneFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for *mut wire_tuple_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal>
    for *mut wire_tuple_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::structure::TupleStructWithTwoFieldTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for *mut wire_tuple_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u16> for *mut u16 {
    fn wire2api(self) -> u16 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<crate::api::attribute::UserIdTwinNormal> for *mut wire_user_id_twin_normal {
    fn wire2api(self) -> crate::api::attribute::UserIdTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::attribute::UserIdTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync>
    for *mut wire_user_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<crate::api::misc_example::WeekdaysTwinNormal> for *mut i32 {
    fn wire2api(self) -> crate::api::misc_example::WeekdaysTwinNormal {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::WeekdaysTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync> for *mut i32 {
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<Box<crate::api::array::BlobTwinNormal>> for *mut wire_blob_twin_normal {
    fn wire2api(self) -> Box<crate::api::array::BlobTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::array::BlobTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync>>
    for *mut wire_blob_twin_sync
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<bool>> for *mut bool {
    fn wire2api(self) -> Box<bool> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<crate::api::enumeration::DistanceTwinNormal>> for *mut wire_distance_twin_normal {
    fn wire2api(self) -> Box<crate::api::enumeration::DistanceTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::DistanceTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync>>
    for *mut wire_distance_twin_sync
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<Box<crate::api::optional::ExoticOptionalsTwinNormal>>
    for *mut wire_exotic_optionals_twin_normal
{
    fn wire2api(self) -> Box<crate::api::optional::ExoticOptionalsTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::optional::ExoticOptionalsTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>>
    for *mut wire_exotic_optionals_twin_sync
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<f64>> for *mut f64 {
    fn wire2api(self) -> Box<f64> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i32>> for *mut i32 {
    fn wire2api(self) -> Box<i32> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i64>> for *mut i64 {
    fn wire2api(self) -> Box<i64> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<i8>> for *mut i8 {
    fn wire2api(self) -> Box<i8> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<crate::api::enumeration::KitchenSinkTwinNormal>>
    for *mut wire_kitchen_sink_twin_normal
{
    fn wire2api(self) -> Box<crate::api::enumeration::KitchenSinkTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::KitchenSinkTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>>
    for *mut wire_kitchen_sink_twin_sync
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<Box<crate::auxiliary::sample_types::MySize>> for *mut wire_my_size {
    fn wire2api(self) -> Box<crate::auxiliary::sample_types::MySize> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::auxiliary::sample_types::MySize>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::misc_example::MySizeFreezedTwinNormal>>
    for *mut wire_my_size_freezed_twin_normal
{
    fn wire2api(self) -> Box<crate::api::misc_example::MySizeFreezedTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::MySizeFreezedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>>
    for *mut wire_my_size_freezed_twin_sync
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::enumeration::SpeedTwinNormal>> for *mut wire_speed_twin_normal {
    fn wire2api(self) -> Box<crate::api::enumeration::SpeedTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::enumeration::SpeedTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync>>
    for *mut wire_speed_twin_sync
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync>::wire2api(*wrap)
            .into()
    }
}
impl Wire2Api<Box<u8>> for *mut u8 {
    fn wire2api(self) -> Box<u8> {
        unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<Box<[u8; 1600]>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<Box<crate::api::misc_example::WeekdaysTwinNormal>> for *mut i32 {
    fn wire2api(self) -> Box<crate::api::misc_example::WeekdaysTwinNormal> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::misc_example::WeekdaysTwinNormal>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>
    for *mut i32
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync> {
        let wrap = unsafe { flutter_rust_bridge::support::box_from_leak_ptr(self) };
        Wire2Api::<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>::wire2api(
            *wrap,
        )
        .into()
    }
}
impl Wire2Api<crate::api::misc_example::CTwinNormal> for wire_c_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::CTwinNormal {
        crate::api::misc_example::CTwinNormal {
            c: self.c.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync> for wire_c_twin_sync {
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync {
            c: self.c.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::method::ConcatenateWithTwinNormal> for wire_concatenate_with_twin_normal {
    fn wire2api(self) -> crate::api::method::ConcatenateWithTwinNormal {
        crate::api::method::ConcatenateWithTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync>
    for wire_concatenate_with_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync {
        crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal>
    for wire_custom_nested_error_inner_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Three);
                crate::api::exception::CustomNestedErrorInnerTwinNormal::Three(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Four);
                crate::api::exception::CustomNestedErrorInnerTwinNormal::Four(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for wire_custom_nested_error_inner_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Three);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Three( ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Four);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Four(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal>
    for wire_custom_nested_error_outer_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.One);
                crate::api::exception::CustomNestedErrorOuterTwinNormal::One(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Two);
                crate::api::exception::CustomNestedErrorOuterTwinNormal::Two(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for wire_custom_nested_error_outer_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.One);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::One(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Two);
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::Two(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal>
    for wire_custom_struct_error_twin_normal
{
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        crate::api::exception::CustomStructErrorTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for wire_custom_struct_error_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructTwinNormal> for wire_custom_struct_twin_normal {
    fn wire2api(self) -> crate::api::exception::CustomStructTwinNormal {
        crate::api::exception::CustomStructTwinNormal {
            message: self.message.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync>
    for wire_custom_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync {
        crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync {
            message: self.message.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::attribute::CustomizedTwinNormal> for wire_customized_twin_normal {
    fn wire2api(self) -> crate::api::attribute::CustomizedTwinNormal {
        crate::api::attribute::CustomizedTwinNormal {
            final_field: self.final_field.wire2api(),
            non_final_field: self.non_final_field.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync>
    for wire_customized_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync {
        crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync {
            final_field: self.final_field.wire2api(),
            non_final_field: self.non_final_field.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNestedTwinNormal>
    for wire_dart_opaque_nested_twin_normal
{
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNestedTwinNormal {
        crate::api::dart_opaque::DartOpaqueNestedTwinNormal {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync>
    for wire_dart_opaque_nested_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync {
        crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::DistanceTwinNormal> for wire_distance_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::DistanceTwinNormal {
        match self.tag {
            0 => crate::api::enumeration::DistanceTwinNormal::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Map);
                crate::api::enumeration::DistanceTwinNormal::Map(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync>
    for wire_distance_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync {
        match self.tag {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Map);
                crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync::Map(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::misc_type::EmptyTwinNormal> for wire_empty_twin_normal {
    fn wire2api(self) -> crate::api::misc_type::EmptyTwinNormal {
        crate::api::misc_type::EmptyTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync>
    for wire_empty_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync {
        crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync {}
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaqueTwinNormal>
    for wire_enum_dart_opaque_twin_normal
{
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaqueTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::dart_opaque::EnumDartOpaqueTwinNormal::Primitive(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Opaque);
                crate::api::dart_opaque::EnumDartOpaqueTwinNormal::Opaque(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync>
    for wire_enum_dart_opaque_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync::Primitive(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Opaque);
                crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync::Opaque(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaqueTwinNormal> for wire_enum_opaque_twin_normal {
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaqueTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Struct);
                crate::api::rust_opaque::EnumOpaqueTwinNormal::Struct(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::rust_opaque::EnumOpaqueTwinNormal::Primitive(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.TraitObj);
                crate::api::rust_opaque::EnumOpaqueTwinNormal::TraitObj(ans.field0.wire2api())
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Mutex);
                crate::api::rust_opaque::EnumOpaqueTwinNormal::Mutex(ans.field0.wire2api())
            },
            4 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.RwLock);
                crate::api::rust_opaque::EnumOpaqueTwinNormal::RwLock(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync>
    for wire_enum_opaque_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Struct);
                crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Struct(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitive);
                crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Primitive(
                    ans.field0.wire2api(),
                )
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.TraitObj);
                crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::TraitObj(
                    ans.field0.wire2api(),
                )
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Mutex);
                crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Mutex(
                    ans.field0.wire2api(),
                )
            },
            4 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.RwLock);
                crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::RwLock(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal>
    for wire_enum_with_item_mixed_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        match self.tag {
            0 => crate::api::enumeration::EnumWithItemMixedTwinNormal::A,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemMixedTwinNormal::B(ans.field0.wire2api())
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::enumeration::EnumWithItemMixedTwinNormal::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for wire_enum_with_item_mixed_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        match self.tag {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::A,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::B(
                    ans.field0.wire2api(),
                )
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.C);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::C {
                    c_field: ans.c_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal>
    for wire_enum_with_item_struct_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::enumeration::EnumWithItemStructTwinNormal::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemStructTwinNormal::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for wire_enum_with_item_struct_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::A {
                    a_field: ans.a_field.wire2api(),
                }
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::B {
                    b_field: ans.b_field.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal>
    for wire_enum_with_item_tuple_twin_normal
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::enumeration::EnumWithItemTupleTwinNormal::A(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::enumeration::EnumWithItemTupleTwinNormal::B(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for wire_enum_with_item_tuple_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.A);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::A(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.B);
                crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::B(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::event_listener::EventTwinNormal> for wire_event_twin_normal {
    fn wire2api(self) -> crate::api::event_listener::EventTwinNormal {
        crate::api::event_listener::EventTwinNormal {
            address: self.address.wire2api(),
            payload: self.payload.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync>
    for wire_event_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync {
        crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync {
            address: self.address.wire2api(),
            payload: self.payload.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionalsTwinNormal>
    for wire_exotic_optionals_twin_normal
{
    fn wire2api(self) -> crate::api::optional::ExoticOptionalsTwinNormal {
        crate::api::optional::ExoticOptionalsTwinNormal {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>
    for wire_exotic_optionals_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync {
        crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync {
            int32: self.int32.wire2api(),
            int64: self.int64.wire2api(),
            float64: self.float64.wire2api(),
            boolean: self.boolean.wire2api(),
            zerocopy: self.zerocopy.wire2api(),
            int8list: self.int8list.wire2api(),
            uint8list: self.uint8list.wire2api(),
            int32list: self.int32list.wire2api(),
            float32list: self.float32list.wire2api(),
            float64list: self.float64list.wire2api(),
            attributes: self.attributes.wire2api(),
            attributes_nullable: self.attributes_nullable.wire2api(),
            nullable_attributes: self.nullable_attributes.wire2api(),
            newtypeint: self.newtypeint.wire2api(),
        }
    }
}
impl Wire2Api<[f64; 16]> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChronoTwinNormal>
    for wire_feature_chrono_twin_normal
{
    fn wire2api(self) -> crate::api::chrono_type::FeatureChronoTwinNormal {
        crate::api::chrono_type::FeatureChronoTwinNormal {
            utc: self.utc.wire2api(),
            local: self.local.wire2api(),
            duration: self.duration.wire2api(),
            naive: self.naive.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync>
    for wire_feature_chrono_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync {
        crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync {
            utc: self.utc.wire2api(),
            local: self.local.wire2api(),
            duration: self.duration.wire2api(),
            naive: self.naive.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuidTwinNormal> for wire_feature_uuid_twin_normal {
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuidTwinNormal {
        crate::api::uuid_type::FeatureUuidTwinNormal {
            one: self.one.wire2api(),
            many: self.many.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync>
    for wire_feature_uuid_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync {
        crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync {
            one: self.one.wire2api(),
            many: self.many.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::FeedIdTwinNormal> for wire_feed_id_twin_normal {
    fn wire2api(self) -> crate::api::array::FeedIdTwinNormal {
        crate::api::array::FeedIdTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync>
    for wire_feed_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync {
        crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync(self.field0.wire2api())
    }
}
impl Wire2Api<[i32; 2]> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::enumeration::KitchenSinkTwinNormal> for wire_kitchen_sink_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::KitchenSinkTwinNormal {
        match self.tag {
            0 => crate::api::enumeration::KitchenSinkTwinNormal::Empty,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitives);
                crate::api::enumeration::KitchenSinkTwinNormal::Primitives {
                    int32: ans.int32.wire2api(),
                    float64: ans.float64.wire2api(),
                    boolean: ans.boolean.wire2api(),
                }
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Nested);
                crate::api::enumeration::KitchenSinkTwinNormal::Nested(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                )
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Optional);
                crate::api::enumeration::KitchenSinkTwinNormal::Optional(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                )
            },
            4 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Buffer);
                crate::api::enumeration::KitchenSinkTwinNormal::Buffer(ans.field0.wire2api())
            },
            5 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Enums);
                crate::api::enumeration::KitchenSinkTwinNormal::Enums(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>
    for wire_kitchen_sink_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync {
        match self.tag {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Empty,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Primitives);
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Primitives {
                    int32: ans.int32.wire2api(),
                    float64: ans.float64.wire2api(),
                    boolean: ans.boolean.wire2api(),
                }
            },
            2 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Nested);
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Nested(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                )
            },
            3 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Optional);
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Optional(
                    ans.field0.wire2api(),
                    ans.field1.wire2api(),
                )
            },
            4 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Buffer);
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Buffer(
                    ans.field0.wire2api(),
                )
            },
            5 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Enums);
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Enums(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::DartOpaque>> for *mut wire_list_DartOpaque {
    fn wire2api(self) -> Vec<flutter_rust_bridge::DartOpaque> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>>
    for *mut wire_list_RustOpaque_hide_data
{
    fn wire2api(
        self,
    ) -> Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar>>
    for *mut wire_list_application_env_var
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::optional::AttributeTwinNormal>>
    for *mut wire_list_attribute_twin_normal
{
    fn wire2api(self) -> Vec<crate::api::optional::AttributeTwinNormal> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>>
    for *mut wire_list_attribute_twin_sync
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<bool>> for *mut wire_list_bool {
    fn wire2api(self) -> Vec<bool> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::auxiliary::sample_types::MySize>> for *mut wire_list_my_size {
    fn wire2api(self) -> Vec<crate::auxiliary::sample_types::MySize> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::MyTreeNodeTwinNormal>>
    for *mut wire_list_my_tree_node_twin_normal
{
    fn wire2api(self) -> Vec<crate::api::misc_example::MyTreeNodeTwinNormal> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>>
    for *mut wire_list_my_tree_node_twin_sync
{
    fn wire2api(
        self,
    ) -> Vec<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<String>>> for *mut wire_list_opt_String {
    fn wire2api(self) -> Vec<Option<String>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::optional::AttributeTwinNormal>>>
    for *mut wire_list_opt_box_autoadd_attribute_twin_normal
{
    fn wire2api(self) -> Vec<Option<crate::api::optional::AttributeTwinNormal>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>>>
    for *mut wire_list_opt_box_autoadd_attribute_twin_sync
{
    fn wire2api(
        self,
    ) -> Vec<Option<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<i32>>> for *mut wire_list_opt_box_autoadd_i_32 {
    fn wire2api(self) -> Vec<Option<i32>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::misc_example::WeekdaysTwinNormal>>>
    for *mut wire_list_opt_box_autoadd_weekdays_twin_normal
{
    fn wire2api(self) -> Vec<Option<crate::api::misc_example::WeekdaysTwinNormal>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>>
    for *mut wire_list_opt_box_autoadd_weekdays_twin_sync
{
    fn wire2api(
        self,
    ) -> Vec<Option<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<Option<Vec<i32>>>> for *mut wire_list_opt_list_prim_i_32 {
    fn wire2api(self) -> Vec<Option<Vec<i32>>> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<f32>> for *mut wire_list_prim_f_32 {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<f64>> for *mut wire_list_prim_f_64 {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i16>> for *mut wire_list_prim_i_16 {
    fn wire2api(self) -> Vec<i16> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i32>> for *mut wire_list_prim_i_32 {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i64>> for *mut wire_list_prim_i_64 {
    fn wire2api(self) -> Vec<i64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<i8>> for *mut wire_list_prim_i_8 {
    fn wire2api(self) -> Vec<i8> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u16>> for *mut wire_list_prim_u_16 {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_list_prim_u_32 {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u64>> for *mut wire_list_prim_u_64 {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u8>> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<(String, i32)>> for *mut wire_list_record_string_i_32 {
    fn wire2api(self) -> Vec<(String, i32)> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::array::TestIdTwinNormal>> for *mut wire_list_test_id_twin_normal {
    fn wire2api(self) -> Vec<crate::api::array::TestIdTwinNormal> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>>
    for *mut wire_list_test_id_twin_sync
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::WeekdaysTwinNormal>>
    for *mut wire_list_weekdays_twin_normal
{
    fn wire2api(self) -> Vec<crate::api::misc_example::WeekdaysTwinNormal> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>
    for *mut wire_list_weekdays_twin_sync
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync> {
        let vec = unsafe {
            let wrap = flutter_rust_bridge::support::box_from_leak_ptr(self);
            flutter_rust_bridge::support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct> for wire_macro_struct {
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        crate::api::inside_macro::MacroStruct {
            data: self.data.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::MeasureTwinNormal> for wire_measure_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::MeasureTwinNormal {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Speed);
                crate::api::enumeration::MeasureTwinNormal::Speed(ans.field0.wire2api())
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Distance);
                crate::api::enumeration::MeasureTwinNormal::Distance(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync>
    for wire_measure_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync {
        match self.tag {
            0 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Speed);
                crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync::Speed(
                    ans.field0.wire2api(),
                )
            },
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.Distance);
                crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync::Distance(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::array::MessageIdTwinNormal> for wire_message_id_twin_normal {
    fn wire2api(self) -> crate::api::array::MessageIdTwinNormal {
        crate::api::array::MessageIdTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync>
    for wire_message_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync {
        crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStructTwinNormal>
    for wire_my_nested_struct_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::MyNestedStructTwinNormal {
        crate::api::misc_example::MyNestedStructTwinNormal {
            tree_node: self.tree_node.wire2api(),
            weekday: self.weekday.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync>
    for wire_my_nested_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync {
            tree_node: self.tree_node.wire2api(),
            weekday: self.weekday.wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MySize> for wire_my_size {
    fn wire2api(self) -> crate::auxiliary::sample_types::MySize {
        crate::auxiliary::sample_types::MySize {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MySizeFreezedTwinNormal>
    for wire_my_size_freezed_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::MySizeFreezedTwinNormal {
        crate::api::misc_example::MySizeFreezedTwinNormal {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>
    for wire_my_size_freezed_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync {
            width: self.width.wire2api(),
            height: self.height.wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct> for wire_my_struct {
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        crate::auxiliary::sample_types::MyStruct {
            content: self.content.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNodeTwinNormal> for wire_my_tree_node_twin_normal {
    fn wire2api(self) -> crate::api::misc_example::MyTreeNodeTwinNormal {
        crate::api::misc_example::MyTreeNodeTwinNormal {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>
    for wire_my_tree_node_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync {
            value_i32: self.value_i32.wire2api(),
            value_vec_u8: self.value_vec_u8.wire2api(),
            value_boolean: self.value_boolean.wire2api(),
            children: self.children.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeIntTwinNormal> for wire_new_type_int_twin_normal {
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeIntTwinNormal {
        crate::api::newtype_pattern::NewTypeIntTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync>
    for wire_new_type_int_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync {
        crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync(
            self.field0.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::enumeration::NoteTwinNormal> for wire_note_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::NoteTwinNormal {
        crate::api::enumeration::NoteTwinNormal {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync>
    for wire_note_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync {
        crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync {
            day: self.day.wire2api(),
            body: self.body.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Numbers> for wire_numbers {
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Numbers {
        crate::api::pseudo_manual::mirror_twin_sync::Numbers(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNestedTwinNormal> for wire_opaque_nested_twin_normal {
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNestedTwinNormal {
        crate::api::rust_opaque::OpaqueNestedTwinNormal {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync>
    for wire_opaque_nested_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync {
        crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync {
            first: self.first.wire2api(),
            second: self.second.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::OptVecsTwinNormal> for wire_opt_vecs_twin_normal {
    fn wire2api(self) -> crate::api::optional::OptVecsTwinNormal {
        crate::api::optional::OptVecsTwinNormal {
            i32: self.i32.wire2api(),
            enums: self.enums.wire2api(),
            strings: self.strings.wire2api(),
            buffers: self.buffers.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync>
    for wire_opt_vecs_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync {
        crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync {
            i32: self.i32.wire2api(),
            enums: self.enums.wire2api(),
            strings: self.strings.wire2api(),
            buffers: self.buffers.wire2api(),
        }
    }
}
impl Wire2Api<(String, i32)> for wire_record_string_i_32 {
    fn wire2api(self) -> (String, i32) {
        (self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Sequences> for wire_sequences {
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Sequences {
        crate::api::pseudo_manual::mirror_twin_sync::Sequences(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::exception::SomeStructTwinNormal> for wire_some_struct_twin_normal {
    fn wire2api(self) -> crate::api::exception::SomeStructTwinNormal {
        crate::api::exception::SomeStructTwinNormal {
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync>
    for wire_some_struct_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync {
        crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync {
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::SpeedTwinNormal> for wire_speed_twin_normal {
    fn wire2api(self) -> crate::api::enumeration::SpeedTwinNormal {
        match self.tag {
            0 => crate::api::enumeration::SpeedTwinNormal::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.GPS);
                crate::api::enumeration::SpeedTwinNormal::GPS(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync>
    for wire_speed_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync {
        match self.tag {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync::Unknown,
            1 => unsafe {
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(self.kind);
                let ans = flutter_rust_bridge::support::box_from_leak_ptr(ans.GPS);
                crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync::GPS(
                    ans.field0.wire2api(),
                )
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal>
    for wire_struct_with_comments_twin_normal
{
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        crate::api::comment::StructWithCommentsTwinNormal {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for wire_struct_with_comments_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
            field_with_comments: self.field_with_comments.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnumTwinNormal>
    for wire_struct_with_enum_twin_normal
{
    fn wire2api(self) -> crate::api::misc_example::StructWithEnumTwinNormal {
        crate::api::misc_example::StructWithEnumTwinNormal {
            abc1: self.abc1.wire2api(),
            abc2: self.abc2.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync>
    for wire_struct_with_enum_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync {
        crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync {
            abc1: self.abc1.wire2api(),
            abc2: self.abc2.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal>
    for wire_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        crate::api::structure::StructWithOneFieldTwinNormal {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for wire_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
            a: self.a.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal>
    for wire_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        crate::api::structure::StructWithTwoFieldTwinNormal {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for wire_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
            a: self.a.wire2api(),
            b: self.b.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal>
    for wire_struct_with_zero_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        crate::api::structure::StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for wire_struct_with_zero_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<crate::api::method::SumWithTwinNormal> for wire_sum_with_twin_normal {
    fn wire2api(self) -> crate::api::method::SumWithTwinNormal {
        crate::api::method::SumWithTwinNormal {
            x: self.x.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync>
    for wire_sum_with_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync {
        crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync {
            x: self.x.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::TestIdTwinNormal> for wire_test_id_twin_normal {
    fn wire2api(self) -> crate::api::array::TestIdTwinNormal {
        crate::api::array::TestIdTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<[crate::api::array::TestIdTwinNormal; 4]> for *mut wire_list_test_id_twin_normal {
    fn wire2api(self) -> [crate::api::array::TestIdTwinNormal; 4] {
        let vec: Vec<crate::api::array::TestIdTwinNormal> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>
    for wire_test_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync {
        crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync(self.field0.wire2api())
    }
}
impl Wire2Api<[crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync; 4]>
    for *mut wire_list_test_id_twin_sync
{
    fn wire2api(self) -> [crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync; 4] {
        let vec: Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal>
    for wire_tuple_struct_with_one_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        crate::api::structure::TupleStructWithOneFieldTwinNormal(self.field0.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for wire_tuple_struct_with_one_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync(
            self.field0.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal>
    for wire_tuple_struct_with_two_field_twin_normal
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        crate::api::structure::TupleStructWithTwoFieldTwinNormal(
            self.field0.wire2api(),
            self.field1.wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for wire_tuple_struct_with_two_field_twin_sync
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync(
            self.field0.wire2api(),
            self.field1.wire2api(),
        )
    }
}
impl Wire2Api<[u8; 1600]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for *mut wire_list_prim_u_8 {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::attribute::UserIdTwinNormal> for wire_user_id_twin_normal {
    fn wire2api(self) -> crate::api::attribute::UserIdTwinNormal {
        crate::api::attribute::UserIdTwinNormal {
            value: self.value.wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync>
    for wire_user_id_twin_sync
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync {
        crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync {
            value: self.value.wire2api(),
        }
    }
}

// Section: wire2api_class

#[repr(C)]
#[derive(Clone)]
pub struct wire_DartOpaque {
    port: i64,
    handle: usize,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_MutexHideData {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_RwLockHideData {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_box_dynDartDebugTwinNormal {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_box_dynDartDebugTwinSync {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_hide_data {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_i_32 {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_non_clone_data {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RustOpaque_non_send_hide_data {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_StringList {
    ptr: *mut *mut wire_list_prim_u_8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_a_twin_normal {
    a: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_a_twin_sync {
    a: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_abc_twin_normal {
    tag: i32,
    kind: *mut AbcTwinNormalKind,
}

#[repr(C)]
pub union AbcTwinNormalKind {
    A: *mut wire_AbcTwinNormal_A,
    B: *mut wire_AbcTwinNormal_B,
    C: *mut wire_AbcTwinNormal_C,
    JustInt: *mut wire_AbcTwinNormal_JustInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinNormal_A {
    field0: *mut wire_a_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinNormal_B {
    field0: *mut wire_b_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinNormal_C {
    field0: *mut wire_c_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinNormal_JustInt {
    field0: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_abc_twin_sync {
    tag: i32,
    kind: *mut AbcTwinSyncKind,
}

#[repr(C)]
pub union AbcTwinSyncKind {
    A: *mut wire_AbcTwinSync_A,
    B: *mut wire_AbcTwinSync_B,
    C: *mut wire_AbcTwinSync_C,
    JustInt: *mut wire_AbcTwinSync_JustInt,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinSync_A {
    field0: *mut wire_a_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinSync_B {
    field0: *mut wire_b_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinSync_C {
    field0: *mut wire_c_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AbcTwinSync_JustInt {
    field0: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_application_env {
    vars: *mut wire_list_application_env_var,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_application_env_var {
    field0: *mut wire_list_prim_u_8,
    field1: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_application_settings {
    name: *mut wire_list_prim_u_8,
    version: *mut wire_list_prim_u_8,
    mode: i32,
    env: *mut wire_application_env,
    env_optional: *mut wire_application_env,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_attribute_twin_normal {
    key: *mut wire_list_prim_u_8,
    value: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_attribute_twin_sync {
    key: *mut wire_list_prim_u_8,
    value: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_b_twin_normal {
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_b_twin_sync {
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_blob_twin_normal {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_blob_twin_sync {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_c_twin_normal {
    c: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_c_twin_sync {
    c: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_concatenate_with_twin_normal {
    a: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_concatenate_with_twin_sync {
    a: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_inner_twin_normal {
    tag: i32,
    kind: *mut CustomNestedErrorInnerTwinNormalKind,
}

#[repr(C)]
pub union CustomNestedErrorInnerTwinNormalKind {
    Three: *mut wire_CustomNestedErrorInnerTwinNormal_Three,
    Four: *mut wire_CustomNestedErrorInnerTwinNormal_Four,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinNormal_Three {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinNormal_Four {
    field0: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_inner_twin_sync {
    tag: i32,
    kind: *mut CustomNestedErrorInnerTwinSyncKind,
}

#[repr(C)]
pub union CustomNestedErrorInnerTwinSyncKind {
    Three: *mut wire_CustomNestedErrorInnerTwinSync_Three,
    Four: *mut wire_CustomNestedErrorInnerTwinSync_Four,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinSync_Three {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorInnerTwinSync_Four {
    field0: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_outer_twin_normal {
    tag: i32,
    kind: *mut CustomNestedErrorOuterTwinNormalKind,
}

#[repr(C)]
pub union CustomNestedErrorOuterTwinNormalKind {
    One: *mut wire_CustomNestedErrorOuterTwinNormal_One,
    Two: *mut wire_CustomNestedErrorOuterTwinNormal_Two,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinNormal_One {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinNormal_Two {
    field0: *mut wire_custom_nested_error_inner_twin_normal,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_nested_error_outer_twin_sync {
    tag: i32,
    kind: *mut CustomNestedErrorOuterTwinSyncKind,
}

#[repr(C)]
pub union CustomNestedErrorOuterTwinSyncKind {
    One: *mut wire_CustomNestedErrorOuterTwinSync_One,
    Two: *mut wire_CustomNestedErrorOuterTwinSync_Two,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinSync_One {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_CustomNestedErrorOuterTwinSync_Two {
    field0: *mut wire_custom_nested_error_inner_twin_sync,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_error_twin_normal {
    a: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_error_twin_sync {
    a: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_twin_normal {
    message: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_custom_struct_twin_sync {
    message: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_customized_twin_normal {
    final_field: *mut wire_list_prim_u_8,
    non_final_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_customized_twin_sync {
    final_field: *mut wire_list_prim_u_8,
    non_final_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_dart_opaque_nested_twin_normal {
    first: wire_DartOpaque,
    second: wire_DartOpaque,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_dart_opaque_nested_twin_sync {
    first: wire_DartOpaque,
    second: wire_DartOpaque,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_distance_twin_normal {
    tag: i32,
    kind: *mut DistanceTwinNormalKind,
}

#[repr(C)]
pub union DistanceTwinNormalKind {
    Unknown: *mut wire_DistanceTwinNormal_Unknown,
    Map: *mut wire_DistanceTwinNormal_Map,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DistanceTwinNormal_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DistanceTwinNormal_Map {
    field0: f64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_distance_twin_sync {
    tag: i32,
    kind: *mut DistanceTwinSyncKind,
}

#[repr(C)]
pub union DistanceTwinSyncKind {
    Unknown: *mut wire_DistanceTwinSync_Unknown,
    Map: *mut wire_DistanceTwinSync_Map,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DistanceTwinSync_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DistanceTwinSync_Map {
    field0: f64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_empty_twin_normal {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_empty_twin_sync {}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_dart_opaque_twin_normal {
    tag: i32,
    kind: *mut EnumDartOpaqueTwinNormalKind,
}

#[repr(C)]
pub union EnumDartOpaqueTwinNormalKind {
    Primitive: *mut wire_EnumDartOpaqueTwinNormal_Primitive,
    Opaque: *mut wire_EnumDartOpaqueTwinNormal_Opaque,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaqueTwinNormal_Primitive {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaqueTwinNormal_Opaque {
    field0: wire_DartOpaque,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_dart_opaque_twin_sync {
    tag: i32,
    kind: *mut EnumDartOpaqueTwinSyncKind,
}

#[repr(C)]
pub union EnumDartOpaqueTwinSyncKind {
    Primitive: *mut wire_EnumDartOpaqueTwinSync_Primitive,
    Opaque: *mut wire_EnumDartOpaqueTwinSync_Opaque,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaqueTwinSync_Primitive {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumDartOpaqueTwinSync_Opaque {
    field0: wire_DartOpaque,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_opaque_twin_normal {
    tag: i32,
    kind: *mut EnumOpaqueTwinNormalKind,
}

#[repr(C)]
pub union EnumOpaqueTwinNormalKind {
    Struct: *mut wire_EnumOpaqueTwinNormal_Struct,
    Primitive: *mut wire_EnumOpaqueTwinNormal_Primitive,
    TraitObj: *mut wire_EnumOpaqueTwinNormal_TraitObj,
    Mutex: *mut wire_EnumOpaqueTwinNormal_Mutex,
    RwLock: *mut wire_EnumOpaqueTwinNormal_RwLock,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinNormal_Struct {
    field0: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinNormal_Primitive {
    field0: wire_RustOpaque_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinNormal_TraitObj {
    field0: wire_RustOpaque_box_dynDartDebugTwinNormal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinNormal_Mutex {
    field0: wire_RustOpaque_MutexHideData,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinNormal_RwLock {
    field0: wire_RustOpaque_RwLockHideData,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_opaque_twin_sync {
    tag: i32,
    kind: *mut EnumOpaqueTwinSyncKind,
}

#[repr(C)]
pub union EnumOpaqueTwinSyncKind {
    Struct: *mut wire_EnumOpaqueTwinSync_Struct,
    Primitive: *mut wire_EnumOpaqueTwinSync_Primitive,
    TraitObj: *mut wire_EnumOpaqueTwinSync_TraitObj,
    Mutex: *mut wire_EnumOpaqueTwinSync_Mutex,
    RwLock: *mut wire_EnumOpaqueTwinSync_RwLock,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinSync_Struct {
    field0: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinSync_Primitive {
    field0: wire_RustOpaque_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinSync_TraitObj {
    field0: wire_RustOpaque_box_dynDartDebugTwinSync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinSync_Mutex {
    field0: wire_RustOpaque_MutexHideData,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumOpaqueTwinSync_RwLock {
    field0: wire_RustOpaque_RwLockHideData,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_mixed_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemMixedTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemMixedTwinNormalKind {
    A: *mut wire_EnumWithItemMixedTwinNormal_A,
    B: *mut wire_EnumWithItemMixedTwinNormal_B,
    C: *mut wire_EnumWithItemMixedTwinNormal_C,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_A {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_B {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinNormal_C {
    c_field: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_mixed_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemMixedTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemMixedTwinSyncKind {
    A: *mut wire_EnumWithItemMixedTwinSync_A,
    B: *mut wire_EnumWithItemMixedTwinSync_B,
    C: *mut wire_EnumWithItemMixedTwinSync_C,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_A {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_B {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemMixedTwinSync_C {
    c_field: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_struct_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemStructTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemStructTwinNormalKind {
    A: *mut wire_EnumWithItemStructTwinNormal_A,
    B: *mut wire_EnumWithItemStructTwinNormal_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinNormal_A {
    a_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinNormal_B {
    b_field: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_struct_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemStructTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemStructTwinSyncKind {
    A: *mut wire_EnumWithItemStructTwinSync_A,
    B: *mut wire_EnumWithItemStructTwinSync_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinSync_A {
    a_field: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemStructTwinSync_B {
    b_field: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_tuple_twin_normal {
    tag: i32,
    kind: *mut EnumWithItemTupleTwinNormalKind,
}

#[repr(C)]
pub union EnumWithItemTupleTwinNormalKind {
    A: *mut wire_EnumWithItemTupleTwinNormal_A,
    B: *mut wire_EnumWithItemTupleTwinNormal_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinNormal_A {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinNormal_B {
    field0: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_enum_with_item_tuple_twin_sync {
    tag: i32,
    kind: *mut EnumWithItemTupleTwinSyncKind,
}

#[repr(C)]
pub union EnumWithItemTupleTwinSyncKind {
    A: *mut wire_EnumWithItemTupleTwinSync_A,
    B: *mut wire_EnumWithItemTupleTwinSync_B,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinSync_A {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EnumWithItemTupleTwinSync_B {
    field0: *mut wire_list_prim_i_32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_event_twin_normal {
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_event_twin_sync {
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_exotic_optionals_twin_normal {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_list_prim_u_8,
    int8list: *mut wire_list_prim_i_8,
    uint8list: *mut wire_list_prim_u_8,
    int32list: *mut wire_list_prim_i_32,
    float32list: *mut wire_list_prim_f_32,
    float64list: *mut wire_list_prim_f_64,
    attributes: *mut wire_list_attribute_twin_normal,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute_twin_normal,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute_twin_normal,
    newtypeint: *mut wire_new_type_int_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_exotic_optionals_twin_sync {
    int32: *mut i32,
    int64: *mut i64,
    float64: *mut f64,
    boolean: *mut bool,
    zerocopy: *mut wire_list_prim_u_8,
    int8list: *mut wire_list_prim_i_8,
    uint8list: *mut wire_list_prim_u_8,
    int32list: *mut wire_list_prim_i_32,
    float32list: *mut wire_list_prim_f_32,
    float64list: *mut wire_list_prim_f_64,
    attributes: *mut wire_list_attribute_twin_sync,
    attributes_nullable: *mut wire_list_opt_box_autoadd_attribute_twin_sync,
    nullable_attributes: *mut wire_list_opt_box_autoadd_attribute_twin_sync,
    newtypeint: *mut wire_new_type_int_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feature_chrono_twin_normal {
    utc: i64,
    local: i64,
    duration: i64,
    naive: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feature_chrono_twin_sync {
    utc: i64,
    local: i64,
    duration: i64,
    naive: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feature_uuid_twin_normal {
    one: *mut wire_list_prim_u_8,
    many: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feature_uuid_twin_sync {
    one: *mut wire_list_prim_u_8,
    many: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feed_id_twin_normal {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_feed_id_twin_sync {
    field0: *mut wire_list_prim_u_8,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_kitchen_sink_twin_normal {
    tag: i32,
    kind: *mut KitchenSinkTwinNormalKind,
}

#[repr(C)]
pub union KitchenSinkTwinNormalKind {
    Empty: *mut wire_KitchenSinkTwinNormal_Empty,
    Primitives: *mut wire_KitchenSinkTwinNormal_Primitives,
    Nested: *mut wire_KitchenSinkTwinNormal_Nested,
    Optional: *mut wire_KitchenSinkTwinNormal_Optional,
    Buffer: *mut wire_KitchenSinkTwinNormal_Buffer,
    Enums: *mut wire_KitchenSinkTwinNormal_Enums,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Empty {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Primitives {
    int32: i32,
    float64: f64,
    boolean: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Nested {
    field0: i32,
    field1: *mut wire_kitchen_sink_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Optional {
    field0: *mut i32,
    field1: *mut i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Buffer {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinNormal_Enums {
    field0: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_kitchen_sink_twin_sync {
    tag: i32,
    kind: *mut KitchenSinkTwinSyncKind,
}

#[repr(C)]
pub union KitchenSinkTwinSyncKind {
    Empty: *mut wire_KitchenSinkTwinSync_Empty,
    Primitives: *mut wire_KitchenSinkTwinSync_Primitives,
    Nested: *mut wire_KitchenSinkTwinSync_Nested,
    Optional: *mut wire_KitchenSinkTwinSync_Optional,
    Buffer: *mut wire_KitchenSinkTwinSync_Buffer,
    Enums: *mut wire_KitchenSinkTwinSync_Enums,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Empty {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Primitives {
    int32: i32,
    float64: f64,
    boolean: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Nested {
    field0: i32,
    field1: *mut wire_kitchen_sink_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Optional {
    field0: *mut i32,
    field1: *mut i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Buffer {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_KitchenSinkTwinSync_Enums {
    field0: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_list_DartOpaque {
    ptr: *mut wire_DartOpaque,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_RustOpaque_hide_data {
    ptr: *mut wire_RustOpaque_hide_data,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_application_env_var {
    ptr: *mut wire_application_env_var,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute_twin_normal {
    ptr: *mut wire_attribute_twin_normal,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_attribute_twin_sync {
    ptr: *mut wire_attribute_twin_sync,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_bool {
    ptr: *mut bool,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_size {
    ptr: *mut wire_my_size,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node_twin_normal {
    ptr: *mut wire_my_tree_node_twin_normal,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_my_tree_node_twin_sync {
    ptr: *mut wire_my_tree_node_twin_sync,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_String {
    ptr: *mut *mut wire_list_prim_u_8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute_twin_normal {
    ptr: *mut *mut wire_attribute_twin_normal,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_attribute_twin_sync {
    ptr: *mut *mut wire_attribute_twin_sync,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_i_32 {
    ptr: *mut *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_weekdays_twin_normal {
    ptr: *mut *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_box_autoadd_weekdays_twin_sync {
    ptr: *mut *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_opt_list_prim_i_32 {
    ptr: *mut *mut wire_list_prim_i_32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_32 {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_f_64 {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_16 {
    ptr: *mut i16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_32 {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_64 {
    ptr: *mut i64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_i_8 {
    ptr: *mut i8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_16 {
    ptr: *mut u16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_32 {
    ptr: *mut u32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_64 {
    ptr: *mut u64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_prim_u_8 {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_record_string_i_32 {
    ptr: *mut wire_record_string_i_32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_test_id_twin_normal {
    ptr: *mut wire_test_id_twin_normal,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_test_id_twin_sync {
    ptr: *mut wire_test_id_twin_sync,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_weekdays_twin_normal {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_weekdays_twin_sync {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_macro_struct {
    data: i32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_measure_twin_normal {
    tag: i32,
    kind: *mut MeasureTwinNormalKind,
}

#[repr(C)]
pub union MeasureTwinNormalKind {
    Speed: *mut wire_MeasureTwinNormal_Speed,
    Distance: *mut wire_MeasureTwinNormal_Distance,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MeasureTwinNormal_Speed {
    field0: *mut wire_speed_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MeasureTwinNormal_Distance {
    field0: *mut wire_distance_twin_normal,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_measure_twin_sync {
    tag: i32,
    kind: *mut MeasureTwinSyncKind,
}

#[repr(C)]
pub union MeasureTwinSyncKind {
    Speed: *mut wire_MeasureTwinSync_Speed,
    Distance: *mut wire_MeasureTwinSync_Distance,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MeasureTwinSync_Speed {
    field0: *mut wire_speed_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_MeasureTwinSync_Distance {
    field0: *mut wire_distance_twin_sync,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_message_id_twin_normal {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_message_id_twin_sync {
    field0: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_nested_struct_twin_normal {
    tree_node: wire_my_tree_node_twin_normal,
    weekday: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_nested_struct_twin_sync {
    tree_node: wire_my_tree_node_twin_sync,
    weekday: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_size {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_size_freezed_twin_normal {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_size_freezed_twin_sync {
    width: i32,
    height: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_struct {
    content: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_tree_node_twin_normal {
    value_i32: i32,
    value_vec_u8: *mut wire_list_prim_u_8,
    value_boolean: bool,
    children: *mut wire_list_my_tree_node_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_my_tree_node_twin_sync {
    value_i32: i32,
    value_vec_u8: *mut wire_list_prim_u_8,
    value_boolean: bool,
    children: *mut wire_list_my_tree_node_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_new_type_int_twin_normal {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_new_type_int_twin_sync {
    field0: i64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_note_twin_normal {
    day: *mut i32,
    body: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_note_twin_sync {
    day: *mut i32,
    body: *mut wire_list_prim_u_8,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_numbers {
    field0: *mut wire_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opaque_nested_twin_normal {
    first: wire_RustOpaque_hide_data,
    second: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opaque_nested_twin_sync {
    first: wire_RustOpaque_hide_data,
    second: wire_RustOpaque_hide_data,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opt_vecs_twin_normal {
    i32: *mut wire_list_opt_box_autoadd_i_32,
    enums: *mut wire_list_opt_box_autoadd_weekdays_twin_normal,
    strings: *mut wire_list_opt_String,
    buffers: *mut wire_list_opt_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_opt_vecs_twin_sync {
    i32: *mut wire_list_opt_box_autoadd_i_32,
    enums: *mut wire_list_opt_box_autoadd_weekdays_twin_sync,
    strings: *mut wire_list_opt_String,
    buffers: *mut wire_list_opt_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_record_string_i_32 {
    field0: *mut wire_list_prim_u_8,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_sequences {
    field0: *mut wire_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_some_struct_twin_normal {
    value: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_some_struct_twin_sync {
    value: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_speed_twin_normal {
    tag: i32,
    kind: *mut SpeedTwinNormalKind,
}

#[repr(C)]
pub union SpeedTwinNormalKind {
    Unknown: *mut wire_SpeedTwinNormal_Unknown,
    GPS: *mut wire_SpeedTwinNormal_GPS,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SpeedTwinNormal_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SpeedTwinNormal_GPS {
    field0: f64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_speed_twin_sync {
    tag: i32,
    kind: *mut SpeedTwinSyncKind,
}

#[repr(C)]
pub union SpeedTwinSyncKind {
    Unknown: *mut wire_SpeedTwinSync_Unknown,
    GPS: *mut wire_SpeedTwinSync_GPS,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SpeedTwinSync_Unknown {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SpeedTwinSync_GPS {
    field0: f64,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_normal {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_comments_twin_sync {
    field_with_comments: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_enum_twin_normal {
    abc1: wire_abc_twin_normal,
    abc2: wire_abc_twin_normal,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_enum_twin_sync {
    abc1: wire_abc_twin_sync,
    abc2: wire_abc_twin_sync,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_one_field_twin_normal {
    a: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_one_field_twin_sync {
    a: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_two_field_twin_normal {
    a: i32,
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_two_field_twin_sync {
    a: i32,
    b: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field_twin_normal {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_struct_with_zero_field_twin_sync {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_sum_with_twin_normal {
    x: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_sum_with_twin_sync {
    x: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_test_id_twin_normal {
    field0: *mut wire_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_test_id_twin_sync {
    field0: *mut wire_list_prim_i_32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_one_field_twin_normal {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_one_field_twin_sync {
    field0: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_two_field_twin_normal {
    field0: i32,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_tuple_struct_with_two_field_twin_sync {
    field0: i32,
    field1: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_user_id_twin_normal {
    value: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_user_id_twin_sync {
    value: u32,
}

// Section: impl_new_with_nullptr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}
impl NewWithNullPtr for wire_DartOpaque {
    fn new_with_null_ptr() -> Self {
        Self { port: 0, handle: 0 }
    }
}
impl NewWithNullPtr for wire_RustOpaque_MutexHideData {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_RwLockHideData {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_box_dynDartDebugTwinNormal {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_box_dynDartDebugTwinSync {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_hide_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_i_32 {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_non_clone_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_RustOpaque_non_send_hide_data {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_a_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_a_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_a_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_a_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_abc_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_abc_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_abc_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_abc_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_application_env {
    fn new_with_null_ptr() -> Self {
        Self {
            vars: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_application_env {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_application_env_var {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_application_env_var {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_application_settings {
    fn new_with_null_ptr() -> Self {
        Self {
            name: core::ptr::null_mut(),
            version: core::ptr::null_mut(),
            mode: Default::default(),
            env: core::ptr::null_mut(),
            env_optional: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_application_settings {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_attribute_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_attribute_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_attribute_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            key: core::ptr::null_mut(),
            value: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_attribute_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_b_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            b: Default::default(),
        }
    }
}
impl Default for wire_b_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_b_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            b: Default::default(),
        }
    }
}
impl Default for wire_b_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_blob_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_blob_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_blob_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_blob_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_c_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            c: Default::default(),
        }
    }
}
impl Default for wire_c_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_c_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            c: Default::default(),
        }
    }
}
impl Default for wire_c_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_concatenate_with_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_concatenate_with_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_concatenate_with_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_concatenate_with_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_inner_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_inner_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_inner_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_inner_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_outer_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_outer_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_nested_error_outer_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_nested_error_outer_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_struct_error_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_error_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_struct_error_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_error_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_struct_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            message: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_custom_struct_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            message: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_custom_struct_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_customized_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            final_field: core::ptr::null_mut(),
            non_final_field: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_customized_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_customized_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            final_field: core::ptr::null_mut(),
            non_final_field: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_customized_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_dart_opaque_nested_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_DartOpaque::new_with_null_ptr(),
            second: wire_DartOpaque::new_with_null_ptr(),
        }
    }
}
impl Default for wire_dart_opaque_nested_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_dart_opaque_nested_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_DartOpaque::new_with_null_ptr(),
            second: wire_DartOpaque::new_with_null_ptr(),
        }
    }
}
impl Default for wire_dart_opaque_nested_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_distance_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_distance_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_distance_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_distance_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_empty_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_empty_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_empty_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_empty_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_dart_opaque_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_dart_opaque_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_dart_opaque_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_dart_opaque_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_opaque_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_opaque_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_opaque_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_opaque_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_mixed_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_mixed_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_mixed_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_mixed_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_struct_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_struct_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_struct_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_struct_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_tuple_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_tuple_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_enum_with_item_tuple_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_enum_with_item_tuple_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_event_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            address: core::ptr::null_mut(),
            payload: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_event_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_event_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            address: core::ptr::null_mut(),
            payload: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_event_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_exotic_optionals_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: core::ptr::null_mut(),
            int64: core::ptr::null_mut(),
            float64: core::ptr::null_mut(),
            boolean: core::ptr::null_mut(),
            zerocopy: core::ptr::null_mut(),
            int8list: core::ptr::null_mut(),
            uint8list: core::ptr::null_mut(),
            int32list: core::ptr::null_mut(),
            float32list: core::ptr::null_mut(),
            float64list: core::ptr::null_mut(),
            attributes: core::ptr::null_mut(),
            attributes_nullable: core::ptr::null_mut(),
            nullable_attributes: core::ptr::null_mut(),
            newtypeint: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_exotic_optionals_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_exotic_optionals_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            int32: core::ptr::null_mut(),
            int64: core::ptr::null_mut(),
            float64: core::ptr::null_mut(),
            boolean: core::ptr::null_mut(),
            zerocopy: core::ptr::null_mut(),
            int8list: core::ptr::null_mut(),
            uint8list: core::ptr::null_mut(),
            int32list: core::ptr::null_mut(),
            float32list: core::ptr::null_mut(),
            float64list: core::ptr::null_mut(),
            attributes: core::ptr::null_mut(),
            attributes_nullable: core::ptr::null_mut(),
            nullable_attributes: core::ptr::null_mut(),
            newtypeint: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_exotic_optionals_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feature_chrono_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            utc: Default::default(),
            local: Default::default(),
            duration: Default::default(),
            naive: Default::default(),
        }
    }
}
impl Default for wire_feature_chrono_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feature_chrono_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            utc: Default::default(),
            local: Default::default(),
            duration: Default::default(),
            naive: Default::default(),
        }
    }
}
impl Default for wire_feature_chrono_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feature_uuid_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            one: core::ptr::null_mut(),
            many: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feature_uuid_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feature_uuid_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            one: core::ptr::null_mut(),
            many: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feature_uuid_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feed_id_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feed_id_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_feed_id_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_feed_id_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_kitchen_sink_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_kitchen_sink_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_kitchen_sink_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_kitchen_sink_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_macro_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            data: Default::default(),
        }
    }
}
impl Default for wire_macro_struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_measure_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_measure_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_measure_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_measure_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_message_id_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_message_id_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_message_id_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_message_id_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_nested_struct_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tree_node: Default::default(),
            weekday: Default::default(),
        }
    }
}
impl Default for wire_my_nested_struct_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_nested_struct_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tree_node: Default::default(),
            weekday: Default::default(),
        }
    }
}
impl Default for wire_my_nested_struct_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_size {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}
impl Default for wire_my_size {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_size_freezed_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}
impl Default for wire_my_size_freezed_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_size_freezed_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}
impl Default for wire_my_size_freezed_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_struct {
    fn new_with_null_ptr() -> Self {
        Self {
            content: Default::default(),
        }
    }
}
impl Default for wire_my_struct {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_tree_node_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: core::ptr::null_mut(),
            value_boolean: Default::default(),
            children: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_my_tree_node_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_my_tree_node_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            value_i32: Default::default(),
            value_vec_u8: core::ptr::null_mut(),
            value_boolean: Default::default(),
            children: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_my_tree_node_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_new_type_int_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_new_type_int_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_new_type_int_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_new_type_int_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_note_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            day: core::ptr::null_mut(),
            body: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_note_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_note_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            day: core::ptr::null_mut(),
            body: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_note_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_numbers {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_numbers {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opaque_nested_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_RustOpaque_hide_data::new_with_null_ptr(),
            second: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }
    }
}
impl Default for wire_opaque_nested_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opaque_nested_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            first: wire_RustOpaque_hide_data::new_with_null_ptr(),
            second: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }
    }
}
impl Default for wire_opaque_nested_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opt_vecs_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            i32: core::ptr::null_mut(),
            enums: core::ptr::null_mut(),
            strings: core::ptr::null_mut(),
            buffers: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_opt_vecs_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_opt_vecs_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            i32: core::ptr::null_mut(),
            enums: core::ptr::null_mut(),
            strings: core::ptr::null_mut(),
            buffers: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_opt_vecs_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_record_string_i_32 {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_record_string_i_32 {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_sequences {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_sequences {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_some_struct_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}
impl Default for wire_some_struct_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_some_struct_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}
impl Default for wire_some_struct_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_speed_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_speed_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_speed_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_speed_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_comments_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field_with_comments: Default::default(),
        }
    }
}
impl Default for wire_struct_with_comments_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_enum_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            abc1: Default::default(),
            abc2: Default::default(),
        }
    }
}
impl Default for wire_struct_with_enum_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_enum_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            abc1: Default::default(),
            abc2: Default::default(),
        }
    }
}
impl Default for wire_struct_with_enum_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_one_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
        }
    }
}
impl Default for wire_struct_with_one_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_one_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
        }
    }
}
impl Default for wire_struct_with_one_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_two_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
impl Default for wire_struct_with_two_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_two_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            a: Default::default(),
            b: Default::default(),
        }
    }
}
impl Default for wire_struct_with_two_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_struct_with_zero_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {}
    }
}
impl Default for wire_struct_with_zero_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_sum_with_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}
impl Default for wire_sum_with_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_sum_with_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            x: Default::default(),
        }
    }
}
impl Default for wire_sum_with_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_test_id_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_test_id_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_test_id_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: core::ptr::null_mut(),
        }
    }
}
impl Default for wire_test_id_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_one_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_one_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_one_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_one_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_two_field_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_two_field_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_tuple_struct_with_two_field_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}
impl Default for wire_tuple_struct_with_two_field_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_user_id_twin_normal {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}
impl Default for wire_user_id_twin_normal {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}
impl NewWithNullPtr for wire_user_id_twin_sync {
    fn new_with_null_ptr() -> Self {
        Self {
            value: Default::default(),
        }
    }
}
impl Default for wire_user_id_twin_sync {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

#[no_mangle]
pub extern "C" fn wire_boxed_blob_twin_normal(port_: i64, blob: *mut wire_list_prim_u_8) {
    wire_boxed_blob_twin_normal_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_func_test_id_twin_normal(port_: i64, id: *mut wire_test_id_twin_normal) {
    wire_func_test_id_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_get_array_twin_normal(port_: i64) {
    wire_get_array_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_complex_array_twin_normal(port_: i64) {
    wire_get_complex_array_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_last_number_twin_normal(port_: i64, array: *mut wire_list_prim_f_64) {
    wire_last_number_twin_normal_impl(port_, array)
}

#[no_mangle]
pub extern "C" fn wire_nested_id_twin_normal(port_: i64, id: *mut wire_list_test_id_twin_normal) {
    wire_nested_id_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_new_msgid_twin_normal(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_new_msgid_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_feed_id_twin_normal(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_return_boxed_feed_id_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_raw_feed_id_twin_normal(
    port_: i64,
    id: *mut wire_feed_id_twin_normal,
) {
    wire_return_boxed_raw_feed_id_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_use_boxed_blob_twin_normal(port_: i64, blob: *mut wire_blob_twin_normal) {
    wire_use_boxed_blob_twin_normal_impl(port_, blob)
}

#[no_mangle]
pub extern "C" fn wire_use_msgid_twin_normal(port_: i64, id: *mut wire_message_id_twin_normal) {
    wire_use_msgid_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_handle_customized_struct_twin_normal(
    port_: i64,
    val: *mut wire_customized_twin_normal,
) {
    wire_handle_customized_struct_twin_normal_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id_twin_normal(
    port_: i64,
    user_id: *mut wire_user_id_twin_normal,
) {
    wire_next_user_id_twin_normal_impl(port_, user_id)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_input_bytes_twin_normal(
    port_: i64,
    bytes: *mut wire_list_prim_u_8,
) {
    wire_benchmark_input_bytes_twin_normal_impl(port_, bytes)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_output_bytes_twin_normal(port_: i64, size: i32) {
    wire_benchmark_output_bytes_twin_normal_impl(port_, size)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_void_twin_normal(port_: i64) {
    wire_benchmark_void_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_datetime_local_twin_normal(port_: i64, d: i64) {
    wire_datetime_local_twin_normal_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_datetime_utc_twin_normal(port_: i64, d: i64) {
    wire_datetime_utc_twin_normal_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_duration_twin_normal(port_: i64, d: i64) {
    wire_duration_twin_normal_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_handle_durations_twin_normal(
    port_: i64,
    durations: *mut wire_list_prim_i_64,
    since: i64,
) {
    wire_handle_durations_twin_normal_impl(port_, durations, since)
}

#[no_mangle]
pub extern "C" fn wire_handle_timestamps_twin_normal(
    port_: i64,
    timestamps: *mut wire_list_prim_i_64,
    epoch: i64,
) {
    wire_handle_timestamps_twin_normal_impl(port_, timestamps, epoch)
}

#[no_mangle]
pub extern "C" fn wire_how_long_does_it_take_twin_normal(
    port_: i64,
    mine: *mut wire_feature_chrono_twin_normal,
) {
    wire_how_long_does_it_take_twin_normal_impl(port_, mine)
}

#[no_mangle]
pub extern "C" fn wire_naivedatetime_twin_normal(port_: i64, d: i64) {
    wire_naivedatetime_twin_normal_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_optional_empty_datetime_utc_twin_normal(port_: i64, d: *mut i64) {
    wire_optional_empty_datetime_utc_twin_normal_impl(port_, d)
}

#[no_mangle]
pub extern "C" fn wire_test_chrono_twin_normal(port_: i64) {
    wire_test_chrono_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_precise_chrono_twin_normal(port_: i64) {
    wire_test_precise_chrono_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: i64,
    that: *mut wire_struct_with_comments_twin_normal,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(port_: i64) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_normal(port_: i64) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_normal(port_: i64) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_dart_dynamic_twin_normal(port_: i64) {
    wire_return_dart_dynamic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_async_accept_dart_opaque_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_async_accept_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_enum_dart_opaque_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_create_enum_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_dart_opaque_twin_normal(
    port_: i64,
    opaque1: wire_DartOpaque,
    opaque2: wire_DartOpaque,
) {
    wire_create_nested_dart_opaque_twin_normal_impl(port_, opaque1, opaque2)
}

#[no_mangle]
pub extern "C" fn wire_drop_static_dart_opaque_twin_normal(port_: i64) {
    wire_drop_static_dart_opaque_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_enum_dart_opaque_twin_normal(
    port_: i64,
    opaque: *mut wire_enum_dart_opaque_twin_normal,
) {
    wire_get_enum_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_get_nested_dart_opaque_twin_normal(
    port_: i64,
    opaque: *mut wire_dart_opaque_nested_twin_normal,
) {
    wire_get_nested_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array_get_twin_normal(
    port_: i64,
    opaque: *mut wire_list_DartOpaque,
) {
    wire_loop_back_array_get_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_array_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option_get_twin_normal(port_: i64, opaque: *mut wire_DartOpaque) {
    wire_loop_back_option_get_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_option_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec_get_twin_normal(
    port_: i64,
    opaque: *mut wire_list_DartOpaque,
) {
    wire_loop_back_vec_get_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_loop_back_vec_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_panic_unwrap_dart_opaque_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_panic_unwrap_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_set_static_dart_opaque_twin_normal(port_: i64, opaque: wire_DartOpaque) {
    wire_set_static_dart_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_return_non_droppable_dart_opaque_twin_normal(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_non_droppable_dart_opaque_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_sync_accept_dart_opaque_twin_normal(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_accept_dart_opaque_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_sync_loopback_twin_normal(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_loopback_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_sync_option_dart_opaque_twin_normal(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_dart_opaque_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_sync_option_loopback_twin_normal(
    opaque: *mut wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_loopback_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_unwrap_dart_opaque_twin_normal(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_dart_opaque_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_normal(port_: i64, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_mixed_twin_normal,
) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_struct_twin_normal,
) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_normal(
    port_: i64,
    arg: *mut wire_enum_with_item_tuple_twin_normal,
) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_parameter_twin_normal(port_: i64, weekday: i32) {
    wire_handle_enum_parameter_twin_normal_impl(port_, weekday)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_struct_twin_normal(
    port_: i64,
    val: *mut wire_kitchen_sink_twin_normal,
) {
    wire_handle_enum_struct_twin_normal_impl(port_, val)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_enum_twin_normal(port_: i64, input: *mut wire_list_prim_u_8) {
    wire_handle_return_enum_twin_normal_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_multiply_by_ten_twin_normal(
    port_: i64,
    measure: *mut wire_measure_twin_normal,
) {
    wire_multiply_by_ten_twin_normal_impl(port_, measure)
}

#[no_mangle]
pub extern "C" fn wire_print_note_twin_normal(port_: i64, note: *mut wire_note_twin_normal) {
    wire_print_note_twin_normal_impl(port_, note)
}

#[no_mangle]
pub extern "C" fn wire_EventTwinNormal_as_string_twin_normal(
    port_: i64,
    that: *mut wire_event_twin_normal,
) {
    wire_EventTwinNormal_as_string_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_close_event_listener_twin_normal(port_: i64) {
    wire_close_event_listener_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_event_twin_normal(
    port_: i64,
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
) {
    wire_create_event_twin_normal_impl(port_, address, payload)
}

#[no_mangle]
pub extern "C" fn wire_register_event_listener_twin_normal(port_: i64) {
    wire_register_event_listener_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinNormal_new_twin_normal(
    port_: i64,
    message: *mut wire_list_prim_u_8,
) {
    wire_CustomStructTwinNormal_new_twin_normal_impl(port_, message)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal(
    port_: i64,
    that: *mut wire_custom_struct_twin_normal,
) {
    wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal(
    port_: i64,
    that: *mut wire_custom_struct_twin_normal,
) {
    wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal(
    port_: i64,
) {
    wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal(
    port_: i64,
) {
    wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinNormal_new_twin_normal(port_: i64, value: u32) {
    wire_SomeStructTwinNormal_new_twin_normal_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal(
    port_: i64,
    that: *mut wire_some_struct_twin_normal,
) {
    wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal(
    port_: i64,
    that: *mut wire_some_struct_twin_normal,
) {
    wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal(port_: i64) {
    wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal(port_: i64) {
    wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_panic_twin_normal(port_: i64) {
    wire_custom_enum_error_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_error_twin_normal(port_: i64) {
    wire_custom_enum_error_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_ok_twin_normal(port_: i64, arg: u32) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_nested_error_return_error_twin_normal(
    port_: i64,
    arg: *mut wire_custom_nested_error_outer_twin_normal,
) {
    wire_custom_nested_error_return_error_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_struct_error_return_error_twin_normal(
    port_: i64,
    arg: *mut wire_custom_struct_error_twin_normal,
) {
    wire_custom_struct_error_return_error_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_normal(port_: i64) {
    wire_func_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_type_fallible_panic_twin_normal(port_: i64) {
    wire_func_type_fallible_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_type_infallible_panic_twin_normal(port_: i64) {
    wire_func_type_infallible_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_panic_with_custom_result_twin_normal(port_: i64) {
    wire_panic_with_custom_result_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_1_twin_normal(port_: i64) {
    wire_return_custom_nested_error_1_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_1_variant1_twin_normal(port_: i64) {
    wire_return_custom_nested_error_1_variant1_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_2_twin_normal(port_: i64) {
    wire_return_custom_nested_error_2_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_custom_struct_error_twin_normal(port_: i64) {
    wire_return_custom_struct_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_custom_struct_ok_twin_normal(port_: i64) {
    wire_return_custom_struct_ok_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_err_custom_error_twin_normal(port_: i64) {
    wire_return_err_custom_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_return_error_variant_twin_normal(port_: i64, variant: u32) {
    wire_return_error_variant_twin_normal_impl(port_, variant)
}

#[no_mangle]
pub extern "C" fn wire_return_ok_custom_error_twin_normal(port_: i64) {
    wire_return_ok_custom_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_stream_sink_throw_anyhow_twin_normal(port_: i64) {
    wire_stream_sink_throw_anyhow_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_sync_return_custom_struct_error_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_return_custom_struct_error_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_throw_anyhow_twin_normal(port_: i64) {
    wire_throw_anyhow_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_call_new_module_system_twin_normal(port_: i64) {
    wire_call_new_module_system_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_call_old_module_system_twin_normal(port_: i64) {
    wire_call_old_module_system_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_enum_twin_normal(port_: i64, my_enum: i32) {
    wire_use_imported_enum_twin_normal_impl(port_, my_enum)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_struct_twin_normal(port_: i64, my_struct: *mut wire_my_struct) {
    wire_use_imported_struct_twin_normal_impl(port_, my_struct)
}

#[no_mangle]
pub extern "C" fn wire_another_macro_struct_twin_normal(port_: i64) {
    wire_another_macro_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_macro_struct_twin_normal(port_: i64, arg: *mut wire_macro_struct) {
    wire_func_macro_struct_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal(
    port_: i64,
    a: *mut wire_list_prim_u_8,
    b: *mut wire_list_prim_u_8,
) {
    wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_concatenate_twin_normal(
    port_: i64,
    that: *mut wire_concatenate_with_twin_normal,
    b: *mut wire_list_prim_u_8,
) {
    wire_ConcatenateWithTwinNormal_concatenate_twin_normal_impl(port_, that, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal(
    port_: i64,
) {
    wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal(
    port_: i64,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal(
    port_: i64,
    that: *mut wire_concatenate_with_twin_normal,
) {
    wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal(
    port_: i64,
    that: *mut wire_concatenate_with_twin_normal,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal_impl(port_, that, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinNormal_new_twin_normal(
    port_: i64,
    a: *mut wire_list_prim_u_8,
) {
    wire_ConcatenateWithTwinNormal_new_twin_normal_impl(port_, a)
}

#[no_mangle]
pub extern "C" fn wire_SumWithTwinNormal_sum_twin_normal(
    port_: i64,
    that: *mut wire_sum_with_twin_normal,
    y: u32,
    z: u32,
) {
    wire_SumWithTwinNormal_sum_twin_normal_impl(port_, that, y, z)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_array_twin_normal(port_: i64, a: u32, b: u32, c: u32) {
    wire_get_sum_array_twin_normal_impl(port_, a, b, c)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_struct_twin_normal(port_: i64) {
    wire_get_sum_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_app_settings_stream_twin_normal(port_: i64) {
    wire_app_settings_stream_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_app_settings_vec_stream_twin_normal(port_: i64) {
    wire_app_settings_vec_stream_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_first_number_twin_normal(port_: i64, nums: *mut wire_numbers) {
    wire_first_number_twin_normal_impl(port_, nums)
}

#[no_mangle]
pub extern "C" fn wire_first_sequence_twin_normal(port_: i64, seqs: *mut wire_sequences) {
    wire_first_sequence_twin_normal_impl(port_, seqs)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings_twin_normal(port_: i64) {
    wire_get_app_settings_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_fallible_app_settings_twin_normal(port_: i64) {
    wire_get_fallible_app_settings_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_get_message_twin_normal(port_: i64) {
    wire_get_message_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded_twin_normal(
    port_: i64,
    app_settings: *mut wire_application_settings,
) {
    wire_is_app_embedded_twin_normal_impl(port_, app_settings)
}

#[no_mangle]
pub extern "C" fn wire_mirror_struct_stream_twin_normal(port_: i64) {
    wire_mirror_struct_stream_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_mirror_tuple_stream_twin_normal(port_: i64) {
    wire_mirror_tuple_stream_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_repeat_number_twin_normal(port_: i64, num: i32, times: usize) {
    wire_repeat_number_twin_normal_impl(port_, num, times)
}

#[no_mangle]
pub extern "C" fn wire_repeat_sequence_twin_normal(port_: i64, seq: i32, times: usize) {
    wire_repeat_sequence_twin_normal_impl(port_, seq, times)
}

#[no_mangle]
pub extern "C" fn wire_test_contains_mirrored_sub_struct_twin_normal(port_: i64) {
    wire_test_contains_mirrored_sub_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_fallible_of_raw_string_mirrored_twin_normal(port_: i64) {
    wire_test_fallible_of_raw_string_mirrored_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_nested_enums_mirrored_twin_normal(port_: i64) {
    wire_test_list_of_nested_enums_mirrored_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_raw_nested_string_mirrored_twin_normal(port_: i64) {
    wire_test_list_of_raw_nested_string_mirrored_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_nested_raw_string_mirrored_twin_normal(port_: i64) {
    wire_test_nested_raw_string_mirrored_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_enum_mirrored_twin_normal(port_: i64, nested: bool) {
    wire_test_raw_string_enum_mirrored_twin_normal_impl(port_, nested)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_mirrored_twin_normal(port_: i64) {
    wire_test_raw_string_mirrored_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_big_buffers_twin_normal(port_: i64) {
    wire_handle_big_buffers_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct_twin_normal(
    port_: i64,
    s: *mut wire_my_tree_node_twin_normal,
) {
    wire_handle_complex_struct_twin_normal_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_struct_twin_normal(
    port_: i64,
    s: *mut wire_my_nested_struct_twin_normal,
) {
    wire_handle_nested_struct_twin_normal_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_twin_normal(port_: i64, s: *mut wire_list_prim_u_8) {
    wire_handle_string_twin_normal_impl(port_, s)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct_sync_freezed_twin_normal(
    arg: *mut wire_my_size_freezed_twin_normal,
    boxed: *mut wire_my_size_freezed_twin_normal,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_sync_freezed_twin_normal_impl(arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct_twin_normal(
    port_: i64,
    arg: *mut wire_my_size,
    boxed: *mut wire_my_size,
) {
    wire_handle_struct_twin_normal_impl(port_, arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8_twin_normal(port_: i64, v: *mut wire_list_prim_u_8) {
    wire_handle_vec_u8_twin_normal_impl(port_, v)
}

#[no_mangle]
pub extern "C" fn wire_list_of_primitive_enums_twin_normal(
    port_: i64,
    weekdays: *mut wire_list_weekdays_twin_normal,
) {
    wire_list_of_primitive_enums_twin_normal_impl(port_, weekdays)
}

#[no_mangle]
pub extern "C" fn wire_test_abc_enum_twin_normal(port_: i64, abc: *mut wire_abc_twin_normal) {
    wire_test_abc_enum_twin_normal_impl(port_, abc)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_with_enum_twin_normal(
    port_: i64,
    se: *mut wire_struct_with_enum_twin_normal,
) {
    wire_test_struct_with_enum_twin_normal_impl(port_, se)
}

#[no_mangle]
pub extern "C" fn wire_empty_struct_twin_normal(port_: i64, empty: *mut wire_empty_twin_normal) {
    wire_empty_struct_twin_normal_impl(port_, empty)
}

#[no_mangle]
pub extern "C" fn wire_func_return_unit_twin_normal(port_: i64) {
    wire_func_return_unit_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_string_twin_normal(port_: i64, arg: *mut wire_list_prim_u_8) {
    wire_func_string_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct_twin_normal(port_: i64, l: *mut wire_list_my_size) {
    wire_handle_list_of_struct_twin_normal_impl(port_, l)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_list_twin_normal(port_: i64, names: *mut wire_StringList) {
    wire_handle_string_list_twin_normal_impl(port_, names)
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype_twin_normal(
    port_: i64,
    arg: *mut wire_new_type_int_twin_normal,
) {
    wire_handle_newtype_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_increment_boxed_optional_twin_normal(port_: i64, opt: *mut f64) {
    wire_handle_increment_boxed_optional_twin_normal_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments_twin_normal(
    port_: i64,
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_exotic_optionals_twin_normal,
) {
    wire_handle_option_box_arguments_twin_normal_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment_twin_normal(
    port_: i64,
    opt: *mut wire_exotic_optionals_twin_normal,
) {
    wire_handle_optional_increment_twin_normal_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return_twin_normal(port_: i64, left: f64, right: f64) {
    wire_handle_optional_return_twin_normal_impl(port_, left, right)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct_twin_normal(
    port_: i64,
    document: *mut wire_list_prim_u_8,
) {
    wire_handle_optional_struct_twin_normal_impl(port_, document)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_opts_twin_normal(
    port_: i64,
    opt: *mut wire_opt_vecs_twin_normal,
) {
    wire_handle_vec_of_opts_twin_normal_impl(port_, opt)
}

#[no_mangle]
pub extern "C" fn wire_sync_option_null_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_sync_option_null_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_option_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_primitive_optional_types_twin_normal(
    port_: i64,
    my_i32: *mut i32,
    my_i64: *mut i64,
    my_f64: *mut f64,
    my_bool: *mut bool,
) {
    wire_primitive_optional_types_twin_normal_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_primitive_twin_normal(port_: i64, n: i32) {
    wire_handle_vec_of_primitive_twin_normal_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_vec_of_primitive_twin_normal(port_: i64, n: i32) {
    wire_handle_zero_copy_vec_of_primitive_twin_normal_impl(port_, n)
}

#[no_mangle]
pub extern "C" fn wire_primitive_types_twin_normal(
    port_: i64,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    wire_primitive_types_twin_normal_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_primitive_u32_twin_normal(port_: i64, my_u32: u32) {
    wire_primitive_u32_twin_normal_impl(port_, my_u32)
}

#[no_mangle]
pub extern "C" fn wire_boxed_blob_twin_sync(
    blob: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_boxed_blob_twin_sync_impl(blob)
}

#[no_mangle]
pub extern "C" fn wire_func_test_id_twin_sync(
    id: *mut wire_test_id_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_test_id_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_get_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_array_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_complex_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_get_complex_array_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_last_number_twin_sync(
    array: *mut wire_list_prim_f_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_last_number_twin_sync_impl(array)
}

#[no_mangle]
pub extern "C" fn wire_nested_id_twin_sync(
    id: *mut wire_list_test_id_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_nested_id_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_new_msgid_twin_sync(
    id: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_new_msgid_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_feed_id_twin_sync(
    id: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_boxed_feed_id_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_return_boxed_raw_feed_id_twin_sync(
    id: *mut wire_feed_id_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_boxed_raw_feed_id_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_use_boxed_blob_twin_sync(
    blob: *mut wire_blob_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_boxed_blob_twin_sync_impl(blob)
}

#[no_mangle]
pub extern "C" fn wire_use_msgid_twin_sync(
    id: *mut wire_message_id_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_msgid_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_handle_customized_struct_twin_sync(
    val: *mut wire_customized_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_customized_struct_twin_sync_impl(val)
}

#[no_mangle]
pub extern "C" fn wire_next_user_id_twin_sync(
    user_id: *mut wire_user_id_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_next_user_id_twin_sync_impl(user_id)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_input_bytes_twin_sync(
    bytes: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_input_bytes_twin_sync_impl(bytes)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_output_bytes_twin_sync(
    size: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_output_bytes_twin_sync_impl(size)
}

#[no_mangle]
pub extern "C" fn wire_benchmark_void_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_void_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_datetime_local_twin_sync(
    d: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_datetime_local_twin_sync_impl(d)
}

#[no_mangle]
pub extern "C" fn wire_datetime_utc_twin_sync(
    d: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_datetime_utc_twin_sync_impl(d)
}

#[no_mangle]
pub extern "C" fn wire_duration_twin_sync(d: i64) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_duration_twin_sync_impl(d)
}

#[no_mangle]
pub extern "C" fn wire_handle_durations_twin_sync(
    durations: *mut wire_list_prim_i_64,
    since: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_durations_twin_sync_impl(durations, since)
}

#[no_mangle]
pub extern "C" fn wire_handle_timestamps_twin_sync(
    timestamps: *mut wire_list_prim_i_64,
    epoch: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_timestamps_twin_sync_impl(timestamps, epoch)
}

#[no_mangle]
pub extern "C" fn wire_how_long_does_it_take_twin_sync(
    mine: *mut wire_feature_chrono_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_how_long_does_it_take_twin_sync_impl(mine)
}

#[no_mangle]
pub extern "C" fn wire_naivedatetime_twin_sync(
    d: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_naivedatetime_twin_sync_impl(d)
}

#[no_mangle]
pub extern "C" fn wire_optional_empty_datetime_utc_twin_sync(
    d: *mut i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_optional_empty_datetime_utc_twin_sync_impl(d)
}

#[no_mangle]
pub extern "C" fn wire_test_chrono_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_chrono_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_precise_chrono_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_precise_chrono_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: *mut wire_struct_with_comments_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_slash_star_star_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_dart_dynamic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_dart_dynamic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_async_accept_dart_opaque_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_async_accept_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_enum_dart_opaque_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_enum_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_dart_opaque_twin_sync(
    opaque1: wire_DartOpaque,
    opaque2: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_nested_dart_opaque_twin_sync_impl(opaque1, opaque2)
}

#[no_mangle]
pub extern "C" fn wire_drop_static_dart_opaque_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_drop_static_dart_opaque_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_enum_dart_opaque_twin_sync(
    opaque: *mut wire_enum_dart_opaque_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_enum_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_get_nested_dart_opaque_twin_sync(
    opaque: *mut wire_dart_opaque_nested_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_nested_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array_get_twin_sync(
    opaque: *mut wire_list_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_array_get_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_array_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_array_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option_get_twin_sync(
    opaque: *mut wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_option_get_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_option_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_option_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec_get_twin_sync(
    opaque: *mut wire_list_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_vec_get_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_loop_back_vec_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_vec_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_panic_unwrap_dart_opaque_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_panic_unwrap_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_set_static_dart_opaque_twin_sync(
    opaque: wire_DartOpaque,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_set_static_dart_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_simple_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_mixed_twin_sync(
    arg: *mut wire_enum_with_item_mixed_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_struct_twin_sync(
    arg: *mut wire_enum_with_item_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_enum_with_item_tuple_twin_sync(
    arg: *mut wire_enum_with_item_tuple_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_parameter_twin_sync(
    weekday: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_enum_parameter_twin_sync_impl(weekday)
}

#[no_mangle]
pub extern "C" fn wire_handle_enum_struct_twin_sync(
    val: *mut wire_kitchen_sink_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_enum_struct_twin_sync_impl(val)
}

#[no_mangle]
pub extern "C" fn wire_handle_return_enum_twin_sync(
    input: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_return_enum_twin_sync_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_multiply_by_ten_twin_sync(
    measure: *mut wire_measure_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_multiply_by_ten_twin_sync_impl(measure)
}

#[no_mangle]
pub extern "C" fn wire_print_note_twin_sync(
    note: *mut wire_note_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_print_note_twin_sync_impl(note)
}

#[no_mangle]
pub extern "C" fn wire_EventTwinSync_as_string_twin_sync(
    that: *mut wire_event_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_EventTwinSync_as_string_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_close_event_listener_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_close_event_listener_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_create_event_twin_sync(
    address: *mut wire_list_prim_u_8,
    payload: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_event_twin_sync_impl(address, payload)
}

#[no_mangle]
pub extern "C" fn wire_register_event_listener_twin_sync(port_: i64) {
    wire_register_event_listener_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinSync_new_twin_sync(
    message: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_new_twin_sync_impl(message)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync(
    that: *mut wire_custom_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync(
    that: *mut wire_custom_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinSync_new_twin_sync(
    value: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_new_twin_sync_impl(value)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync(
    that: *mut wire_some_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync(
    that: *mut wire_some_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync_impl(that)
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_custom_enum_error_return_ok_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_nested_error_return_error_twin_sync(
    arg: *mut wire_custom_nested_error_outer_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_custom_struct_error_return_error_twin_sync(
    arg: *mut wire_custom_struct_error_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_return_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_func_return_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_fallible_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_type_infallible_panic_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_panic_with_custom_result_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_panic_with_custom_result_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_1_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_nested_error_1_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_1_variant1_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_nested_error_1_variant1_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_custom_nested_error_2_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_nested_error_2_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_custom_struct_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_struct_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_custom_struct_ok_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_struct_ok_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_err_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_err_custom_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_return_error_variant_twin_sync(
    variant: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_error_variant_twin_sync_impl(variant)
}

#[no_mangle]
pub extern "C" fn wire_return_ok_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_ok_custom_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_stream_sink_throw_anyhow_twin_sync(port_: i64) {
    wire_stream_sink_throw_anyhow_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_sync_return_custom_struct_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_return_custom_struct_error_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_throw_anyhow_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_throw_anyhow_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_call_new_module_system_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_call_new_module_system_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_call_old_module_system_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_call_old_module_system_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_use_imported_enum_twin_sync(
    my_enum: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_imported_enum_twin_sync_impl(my_enum)
}

#[no_mangle]
pub extern "C" fn wire_use_imported_struct_twin_sync(
    my_struct: *mut wire_my_struct,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_imported_struct_twin_sync_impl(my_struct)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_concatenate_static_twin_sync(
    a: *mut wire_list_prim_u_8,
    b: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_concatenate_static_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_concatenate_twin_sync(
    that: *mut wire_concatenate_with_twin_sync,
    b: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_concatenate_twin_sync_impl(that, b)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync(
    port_: i64,
) {
    wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync(
    port_: i64,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync(
    port_: i64,
    that: *mut wire_concatenate_with_twin_sync,
) {
    wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync_impl(port_, that)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync(
    port_: i64,
    that: *mut wire_concatenate_with_twin_sync,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync_impl(port_, that, key, max)
}

#[no_mangle]
pub extern "C" fn wire_ConcatenateWithTwinSync_new_twin_sync(
    a: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_new_twin_sync_impl(a)
}

#[no_mangle]
pub extern "C" fn wire_SumWithTwinSync_sum_twin_sync(
    that: *mut wire_sum_with_twin_sync,
    y: u32,
    z: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SumWithTwinSync_sum_twin_sync_impl(that, y, z)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_array_twin_sync(
    a: u32,
    b: u32,
    c: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_sum_array_twin_sync_impl(a, b, c)
}

#[no_mangle]
pub extern "C" fn wire_get_sum_struct_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_sum_struct_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_app_settings_stream_twin_sync(port_: i64) {
    wire_app_settings_stream_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_app_settings_vec_stream_twin_sync(port_: i64) {
    wire_app_settings_vec_stream_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_first_number_twin_sync(
    nums: *mut wire_numbers,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_first_number_twin_sync_impl(nums)
}

#[no_mangle]
pub extern "C" fn wire_first_sequence_twin_sync(
    seqs: *mut wire_sequences,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_first_sequence_twin_sync_impl(seqs)
}

#[no_mangle]
pub extern "C" fn wire_get_app_settings_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_get_app_settings_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_fallible_app_settings_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_fallible_app_settings_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_get_message_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_message_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_is_app_embedded_twin_sync(
    app_settings: *mut wire_application_settings,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_is_app_embedded_twin_sync_impl(app_settings)
}

#[no_mangle]
pub extern "C" fn wire_mirror_struct_stream_twin_sync(port_: i64) {
    wire_mirror_struct_stream_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_mirror_tuple_stream_twin_sync(port_: i64) {
    wire_mirror_tuple_stream_twin_sync_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_repeat_number_twin_sync(
    num: i32,
    times: usize,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_repeat_number_twin_sync_impl(num, times)
}

#[no_mangle]
pub extern "C" fn wire_repeat_sequence_twin_sync(
    seq: i32,
    times: usize,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_repeat_sequence_twin_sync_impl(seq, times)
}

#[no_mangle]
pub extern "C" fn wire_test_contains_mirrored_sub_struct_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_contains_mirrored_sub_struct_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_fallible_of_raw_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_fallible_of_raw_string_mirrored_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_nested_enums_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_list_of_nested_enums_mirrored_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_list_of_raw_nested_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_list_of_raw_nested_string_mirrored_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_nested_raw_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_nested_raw_string_mirrored_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_enum_mirrored_twin_sync(
    nested: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_raw_string_enum_mirrored_twin_sync_impl(nested)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_raw_string_mirrored_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_handle_big_buffers_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_handle_big_buffers_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_handle_complex_struct_twin_sync(
    s: *mut wire_my_tree_node_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_complex_struct_twin_sync_impl(s)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_struct_twin_sync(
    s: *mut wire_my_nested_struct_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_nested_struct_twin_sync_impl(s)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_twin_sync(
    s: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_string_twin_sync_impl(s)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct_sync_freezed_twin_sync(
    arg: *mut wire_my_size_freezed_twin_sync,
    boxed: *mut wire_my_size_freezed_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_sync_freezed_twin_sync_impl(arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_struct_twin_sync(
    arg: *mut wire_my_size,
    boxed: *mut wire_my_size,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_twin_sync_impl(arg, boxed)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_u8_twin_sync(
    v: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_u8_twin_sync_impl(v)
}

#[no_mangle]
pub extern "C" fn wire_list_of_primitive_enums_twin_sync(
    weekdays: *mut wire_list_weekdays_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_list_of_primitive_enums_twin_sync_impl(weekdays)
}

#[no_mangle]
pub extern "C" fn wire_test_abc_enum_twin_sync(
    abc: *mut wire_abc_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_abc_enum_twin_sync_impl(abc)
}

#[no_mangle]
pub extern "C" fn wire_test_struct_with_enum_twin_sync(
    se: *mut wire_struct_with_enum_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_struct_with_enum_twin_sync_impl(se)
}

#[no_mangle]
pub extern "C" fn wire_empty_struct_twin_sync(
    empty: *mut wire_empty_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_empty_struct_twin_sync_impl(empty)
}

#[no_mangle]
pub extern "C" fn wire_func_return_unit_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_func_return_unit_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_func_string_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_string_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_list_of_struct_twin_sync(
    l: *mut wire_list_my_size,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_list_of_struct_twin_sync_impl(l)
}

#[no_mangle]
pub extern "C" fn wire_handle_string_list_twin_sync(
    names: *mut wire_StringList,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_string_list_twin_sync_impl(names)
}

#[no_mangle]
pub extern "C" fn wire_handle_newtype_twin_sync(
    arg: *mut wire_new_type_int_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_newtype_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: i64,
    arg: *mut bool,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_normal(port_: i64, arg: *mut f32) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_normal(port_: i64, arg: *mut f64) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_normal(port_: i64, arg: *mut i16) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_normal(port_: i64, arg: *mut i32) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_normal(port_: i64, arg: *mut i64) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_normal(port_: i64, arg: *mut i8) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_normal(port_: i64, arg: *mut u16) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_normal(port_: i64, arg: *mut u32) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_normal(port_: i64, arg: *mut u64) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_normal(port_: i64, arg: *mut u8) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_primitive_optional_types_twin_sync(
    my_i32: *mut i32,
    my_i64: *mut i64,
    my_f64: *mut f64,
    my_bool: *mut bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_optional_types_twin_sync_impl(my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: *mut bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: *mut f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: *mut f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: *mut i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: *mut i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: *mut i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: *mut i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: *mut u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: *mut u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: *mut u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: *mut u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_increment_boxed_optional_twin_sync(
    opt: *mut f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_increment_boxed_optional_twin_sync_impl(opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_option_box_arguments_twin_sync(
    i8box: *mut i8,
    u8box: *mut u8,
    i32box: *mut i32,
    i64box: *mut i64,
    f64box: *mut f64,
    boolbox: *mut bool,
    structbox: *mut wire_exotic_optionals_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_option_box_arguments_twin_sync_impl(
        i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_increment_twin_sync(
    opt: *mut wire_exotic_optionals_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_increment_twin_sync_impl(opt)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_return_twin_sync(
    left: f64,
    right: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_return_twin_sync_impl(left, right)
}

#[no_mangle]
pub extern "C" fn wire_handle_optional_struct_twin_sync(
    document: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_struct_twin_sync_impl(document)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_opts_twin_sync(
    opt: *mut wire_opt_vecs_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_of_opts_twin_sync_impl(opt)
}

#[no_mangle]
pub extern "C" fn wire_sync_option_null_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_sync_option_null_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_option_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_normal(port_: i64, arg: bool) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_normal(port_: i64, arg: f32) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_normal(port_: i64, arg: f64) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_normal(port_: i64, arg: i16) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_normal(port_: i64, arg: i32) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_normal(port_: i64, arg: i64) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_normal(port_: i64, arg: i8) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_normal(port_: i64, arg: u16) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_normal(port_: i64, arg: u32) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_normal(port_: i64, arg: u64) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_normal(port_: i64, arg: u8) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_normal(
    port_: i64,
    arg: *mut wire_list_bool,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_32,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_f_64,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_16,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_32,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_64,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_i_8,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_16,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_32,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_64,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_normal(
    port_: i64,
    arg: *mut wire_list_prim_u_8,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_handle_vec_of_primitive_twin_sync(
    n: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_of_primitive_twin_sync_impl(n)
}

#[no_mangle]
pub extern "C" fn wire_handle_zero_copy_vec_of_primitive_twin_sync(
    n: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_zero_copy_vec_of_primitive_twin_sync_impl(n)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_bool_twin_sync(
    arg: *mut wire_list_bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f32_twin_sync(
    arg: *mut wire_list_prim_f_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_f64_twin_sync(
    arg: *mut wire_list_prim_f_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i16_twin_sync(
    arg: *mut wire_list_prim_i_16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i32_twin_sync(
    arg: *mut wire_list_prim_i_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i64_twin_sync(
    arg: *mut wire_list_prim_i_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_i8_twin_sync(
    arg: *mut wire_list_prim_i_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u16_twin_sync(
    arg: *mut wire_list_prim_u_16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u32_twin_sync(
    arg: *mut wire_list_prim_u_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u64_twin_sync(
    arg: *mut wire_list_prim_u_64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_list_type_u8_twin_sync(
    arg: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_primitive_types_twin_sync(
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_types_twin_sync_impl(my_i32, my_i64, my_f64, my_bool)
}

#[no_mangle]
pub extern "C" fn wire_primitive_u32_twin_sync(
    my_u32: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_u32_twin_sync_impl(my_u32)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_bool_twin_sync(
    arg: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f32_twin_sync(
    arg: f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_f64_twin_sync(
    arg: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i16_twin_sync(
    arg: i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i32_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i64_twin_sync(
    arg: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_i8_twin_sync(
    arg: i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u16_twin_sync(
    arg: u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u32_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u64_twin_sync(
    arg: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_example_primitive_type_u8_twin_sync(
    arg: u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_test_more_than_just_one_raw_string_struct_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_more_than_just_one_raw_string_struct_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_item_struct_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_raw_string_item_struct_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_create_array_opaque_enum_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_array_opaque_enum_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_create_nested_opaque_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_nested_opaque_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_create_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_opaque_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_create_option_opaque_twin_sync(
    opaque: *mut wire_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_option_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_sync_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_create_sync_opaque_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_frb_generator_test_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_frb_generator_test_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_opaque_array_run_twin_sync(
    data: *mut wire_list_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_array_run_twin_sync_impl(data)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_array_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec_run_twin_sync(
    data: *mut wire_list_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_vec_run_twin_sync_impl(data)
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_vec_twin_sync_impl()
}

#[no_mangle]
pub extern "C" fn wire_run_enum_opaque_twin_sync(
    opaque: *mut wire_enum_opaque_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_enum_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_nested_opaque_twin_sync(
    opaque: *mut wire_opaque_nested_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_nested_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_non_clone_twin_sync(
    clone: wire_RustOpaque_non_clone_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_non_clone_twin_sync_impl(clone)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_twin_sync(
    opaque: wire_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_with_delay_twin_sync(
    opaque: wire_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_opaque_with_delay_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_unwrap_rust_opaque_twin_sync(
    opaque: wire_RustOpaque_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_rust_opaque_twin_sync_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_sync(
    a: i32,
    b: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_sync(
    arg: *mut wire_struct_with_one_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_sync(
    arg: *mut wire_struct_with_two_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_sync(
    arg: *mut wire_struct_with_zero_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_sync(
    arg: *mut wire_tuple_struct_with_one_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_sync(
    arg: *mut wire_tuple_struct_with_two_field_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_two_field_twin_sync_impl(arg)
}

#[no_mangle]
pub extern "C" fn wire_test_tuple_2_twin_sync(
    value: *mut wire_list_record_string_i_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_tuple_2_twin_sync_impl(value)
}

#[no_mangle]
pub extern "C" fn wire_test_tuple_twin_sync(
    value: *mut wire_record_string_i_32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_tuple_twin_sync_impl(value)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_alias_id_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_alias_id_twin_sync_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_alias_model_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_alias_model_twin_sync_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_nest_alias_id_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_nest_alias_id_twin_sync_impl(input)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_uuids_twin_sync(
    ids: *mut wire_feature_uuid_twin_sync,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_nested_uuids_twin_sync_impl(ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuid_twin_sync(
    id: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_uuid_twin_sync_impl(id)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuids_twin_sync(
    ids: *mut wire_list_prim_u_8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_uuids_twin_sync_impl(ids)
}

#[no_mangle]
pub extern "C" fn wire_test_more_than_just_one_raw_string_struct_twin_normal(port_: i64) {
    wire_test_more_than_just_one_raw_string_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_test_raw_string_item_struct_twin_normal(port_: i64) {
    wire_test_raw_string_item_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_array_opaque_enum_twin_normal(port_: i64) {
    wire_create_array_opaque_enum_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_nested_opaque_twin_normal(port_: i64) {
    wire_create_nested_opaque_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_opaque_twin_normal(port_: i64) {
    wire_create_opaque_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_option_opaque_twin_normal(
    port_: i64,
    opaque: *mut wire_RustOpaque_hide_data,
) {
    wire_create_option_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_create_sync_opaque_twin_normal(port_: i64) {
    wire_create_sync_opaque_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_frb_generator_test_twin_normal(port_: i64) {
    wire_frb_generator_test_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array_run_twin_normal(
    port_: i64,
    data: *mut wire_list_RustOpaque_hide_data,
) {
    wire_opaque_array_run_twin_normal_impl(port_, data)
}

#[no_mangle]
pub extern "C" fn wire_opaque_array_twin_normal(port_: i64) {
    wire_opaque_array_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec_run_twin_normal(
    port_: i64,
    data: *mut wire_list_RustOpaque_hide_data,
) {
    wire_opaque_vec_run_twin_normal_impl(port_, data)
}

#[no_mangle]
pub extern "C" fn wire_opaque_vec_twin_normal(port_: i64) {
    wire_opaque_vec_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_run_enum_opaque_twin_normal(
    port_: i64,
    opaque: *mut wire_enum_opaque_twin_normal,
) {
    wire_run_enum_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_nested_opaque_twin_normal(
    port_: i64,
    opaque: *mut wire_opaque_nested_twin_normal,
) {
    wire_run_nested_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_non_clone_twin_normal(
    port_: i64,
    clone: wire_RustOpaque_non_clone_data,
) {
    wire_run_non_clone_twin_normal_impl(port_, clone)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_twin_normal(port_: i64, opaque: wire_RustOpaque_hide_data) {
    wire_run_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_run_opaque_with_delay_twin_normal(
    port_: i64,
    opaque: wire_RustOpaque_hide_data,
) {
    wire_run_opaque_with_delay_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_unwrap_rust_opaque_twin_normal(
    port_: i64,
    opaque: wire_RustOpaque_hide_data,
) {
    wire_unwrap_rust_opaque_twin_normal_impl(port_, opaque)
}

#[no_mangle]
pub extern "C" fn wire_frb_sync_generator_test_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_frb_sync_generator_test_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_create_non_clone_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_non_clone_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_create_opaque_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_opaque_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_create_sync_opaque_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_sync_opaque_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_option_rust_opaque_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_rust_opaque_twin_normal_impl()
}

#[no_mangle]
pub extern "C" fn wire_sync_run_opaque_twin_normal(
    opaque: wire_RustOpaque_non_send_hide_data,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_run_opaque_twin_normal_impl(opaque)
}

#[no_mangle]
pub extern "C" fn wire_simple_adder_twin_normal(port_: i64, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_realistic_twin_normal(port_: i64, arg: *mut wire_list_prim_u_8) {
    wire_func_stream_realistic_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_return_error_twin_normal(port_: i64) {
    wire_func_stream_return_error_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_return_panic_twin_normal(port_: i64) {
    wire_func_stream_return_panic_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_func_stream_sink_arg_position_twin_normal(port_: i64, a: u32, b: u32) {
    wire_func_stream_sink_arg_position_twin_normal_impl(port_, a, b)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_of_struct_twin_normal(port_: i64) {
    wire_handle_stream_of_struct_twin_normal_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_1_twin_normal(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_1_twin_normal_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_2_twin_normal(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_2_twin_normal_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_handle_stream_sink_at_3_twin_normal(port_: i64, key: u32, max: u32) {
    wire_handle_stream_sink_at_3_twin_normal_impl(port_, key, max)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_one_field_twin_normal,
) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_two_field_twin_normal,
) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_struct_with_zero_field_twin_normal(
    port_: i64,
    arg: *mut wire_struct_with_zero_field_twin_normal,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_one_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_one_field_twin_normal,
) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_func_tuple_struct_with_two_field_twin_normal(
    port_: i64,
    arg: *mut wire_tuple_struct_with_two_field_twin_normal,
) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[no_mangle]
pub extern "C" fn wire_test_tuple_2_twin_normal(
    port_: i64,
    value: *mut wire_list_record_string_i_32,
) {
    wire_test_tuple_2_twin_normal_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_test_tuple_twin_normal(port_: i64, value: *mut wire_record_string_i_32) {
    wire_test_tuple_twin_normal_impl(port_, value)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_alias_id_twin_normal(port_: i64, input: u64) {
    wire_handle_type_alias_id_twin_normal_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_alias_model_twin_normal(port_: i64, input: u64) {
    wire_handle_type_alias_model_twin_normal_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_type_nest_alias_id_twin_normal(port_: i64, input: u64) {
    wire_handle_type_nest_alias_id_twin_normal_impl(port_, input)
}

#[no_mangle]
pub extern "C" fn wire_handle_nested_uuids_twin_normal(
    port_: i64,
    ids: *mut wire_feature_uuid_twin_normal,
) {
    wire_handle_nested_uuids_twin_normal_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuid_twin_normal(port_: i64, id: *mut wire_list_prim_u_8) {
    wire_handle_uuid_twin_normal_impl(port_, id)
}

#[no_mangle]
pub extern "C" fn wire_handle_uuids_twin_normal(port_: i64, ids: *mut wire_list_prim_u_8) {
    wire_handle_uuids_twin_normal_impl(port_, ids)
}

#[no_mangle]
pub extern "C" fn new_DartOpaque() -> wire_DartOpaque {
    wire_DartOpaque::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_MutexHideData() -> wire_RustOpaque_MutexHideData {
    wire_RustOpaque_MutexHideData::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_RwLockHideData() -> wire_RustOpaque_RwLockHideData {
    wire_RustOpaque_RwLockHideData::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_box_dynDartDebugTwinNormal(
) -> wire_RustOpaque_box_dynDartDebugTwinNormal {
    wire_RustOpaque_box_dynDartDebugTwinNormal::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_box_dynDartDebugTwinSync(
) -> wire_RustOpaque_box_dynDartDebugTwinSync {
    wire_RustOpaque_box_dynDartDebugTwinSync::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_hide_data() -> wire_RustOpaque_hide_data {
    wire_RustOpaque_hide_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_i_32() -> wire_RustOpaque_i_32 {
    wire_RustOpaque_i_32::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_non_clone_data() -> wire_RustOpaque_non_clone_data {
    wire_RustOpaque_non_clone_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_RustOpaque_non_send_hide_data() -> wire_RustOpaque_non_send_hide_data {
    wire_RustOpaque_non_send_hide_data::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_StringList(len: i32) -> *mut wire_StringList {
    let wrap = wire_StringList {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <*mut wire_list_prim_u_8>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_box_application_env() -> *mut wire_application_env {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_env::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_Chrono_Utc(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_DartOpaque() -> *mut wire_DartOpaque {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_DartOpaque::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_RustOpaque_hide_data() -> *mut wire_RustOpaque_hide_data {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_RustOpaque_hide_data::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_a_twin_normal() -> *mut wire_a_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_a_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_a_twin_sync() -> *mut wire_a_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_a_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_abc_twin_normal() -> *mut wire_abc_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_abc_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_abc_twin_sync() -> *mut wire_abc_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_abc_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_env() -> *mut wire_application_env {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_env::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_application_settings() -> *mut wire_application_settings {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_application_settings::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute_twin_normal() -> *mut wire_attribute_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_attribute_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_attribute_twin_sync() -> *mut wire_attribute_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_attribute_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_b_twin_normal() -> *mut wire_b_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_b_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_b_twin_sync() -> *mut wire_b_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_b_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_bool(value: bool) -> *mut bool {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_c_twin_normal() -> *mut wire_c_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_c_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_c_twin_sync() -> *mut wire_c_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_c_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_concatenate_with_twin_normal(
) -> *mut wire_concatenate_with_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_concatenate_with_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_concatenate_with_twin_sync(
) -> *mut wire_concatenate_with_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_concatenate_with_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_normal(
) -> *mut wire_custom_nested_error_inner_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_inner_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_inner_twin_sync(
) -> *mut wire_custom_nested_error_inner_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_inner_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_normal(
) -> *mut wire_custom_nested_error_outer_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_outer_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_nested_error_outer_twin_sync(
) -> *mut wire_custom_nested_error_outer_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_nested_error_outer_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_normal(
) -> *mut wire_custom_struct_error_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_struct_error_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_error_twin_sync(
) -> *mut wire_custom_struct_error_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_struct_error_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_twin_normal() -> *mut wire_custom_struct_twin_normal
{
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_custom_struct_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_custom_struct_twin_sync() -> *mut wire_custom_struct_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_custom_struct_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_customized_twin_normal() -> *mut wire_customized_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_customized_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_customized_twin_sync() -> *mut wire_customized_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_customized_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_dart_opaque_nested_twin_normal(
) -> *mut wire_dart_opaque_nested_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_dart_opaque_nested_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_dart_opaque_nested_twin_sync(
) -> *mut wire_dart_opaque_nested_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_dart_opaque_nested_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_empty_twin_normal() -> *mut wire_empty_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_empty_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_empty_twin_sync() -> *mut wire_empty_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_empty_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_dart_opaque_twin_normal(
) -> *mut wire_enum_dart_opaque_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_dart_opaque_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_dart_opaque_twin_sync(
) -> *mut wire_enum_dart_opaque_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_dart_opaque_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_opaque_twin_normal() -> *mut wire_enum_opaque_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_enum_opaque_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_opaque_twin_sync() -> *mut wire_enum_opaque_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_enum_opaque_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_normal(
) -> *mut wire_enum_with_item_mixed_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_mixed_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_mixed_twin_sync(
) -> *mut wire_enum_with_item_mixed_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_mixed_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_normal(
) -> *mut wire_enum_with_item_struct_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_struct_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_struct_twin_sync(
) -> *mut wire_enum_with_item_struct_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_struct_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_normal(
) -> *mut wire_enum_with_item_tuple_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_tuple_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_enum_with_item_tuple_twin_sync(
) -> *mut wire_enum_with_item_tuple_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_enum_with_item_tuple_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_event_twin_normal() -> *mut wire_event_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_event_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_event_twin_sync() -> *mut wire_event_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_event_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals_twin_normal(
) -> *mut wire_exotic_optionals_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_exotic_optionals_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_exotic_optionals_twin_sync(
) -> *mut wire_exotic_optionals_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_exotic_optionals_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_32(value: f32) -> *mut f32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f_64(value: f64) -> *mut f64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_chrono_twin_normal(
) -> *mut wire_feature_chrono_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_feature_chrono_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_chrono_twin_sync() -> *mut wire_feature_chrono_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_feature_chrono_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_uuid_twin_normal() -> *mut wire_feature_uuid_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_feature_uuid_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feature_uuid_twin_sync() -> *mut wire_feature_uuid_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feature_uuid_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feed_id_twin_normal() -> *mut wire_feed_id_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feed_id_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_feed_id_twin_sync() -> *mut wire_feed_id_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_feed_id_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_16(value: i16) -> *mut i16 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_32(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_64(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_i_8(value: i8) -> *mut i8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kitchen_sink_twin_normal() -> *mut wire_kitchen_sink_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_kitchen_sink_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_kitchen_sink_twin_sync() -> *mut wire_kitchen_sink_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_kitchen_sink_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_macro_struct() -> *mut wire_macro_struct {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_macro_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_measure_twin_normal() -> *mut wire_measure_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_measure_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_measure_twin_sync() -> *mut wire_measure_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_measure_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_message_id_twin_normal() -> *mut wire_message_id_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_message_id_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_message_id_twin_sync() -> *mut wire_message_id_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_message_id_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_nested_struct_twin_normal(
) -> *mut wire_my_nested_struct_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_nested_struct_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_nested_struct_twin_sync(
) -> *mut wire_my_nested_struct_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_nested_struct_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size() -> *mut wire_my_size {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_size::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size_freezed_twin_normal(
) -> *mut wire_my_size_freezed_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_size_freezed_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_size_freezed_twin_sync() -> *mut wire_my_size_freezed_twin_sync
{
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_size_freezed_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_struct() -> *mut wire_my_struct {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_struct::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node_twin_normal() -> *mut wire_my_tree_node_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_tree_node_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_my_tree_node_twin_sync() -> *mut wire_my_tree_node_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_tree_node_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int_twin_normal() -> *mut wire_new_type_int_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_new_type_int_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_new_type_int_twin_sync() -> *mut wire_new_type_int_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_new_type_int_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note_twin_normal() -> *mut wire_note_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_note_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_note_twin_sync() -> *mut wire_note_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_note_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_numbers() -> *mut wire_numbers {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_numbers::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opaque_nested_twin_normal() -> *mut wire_opaque_nested_twin_normal
{
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_opaque_nested_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opaque_nested_twin_sync() -> *mut wire_opaque_nested_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_opaque_nested_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opt_vecs_twin_normal() -> *mut wire_opt_vecs_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_opt_vecs_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_opt_vecs_twin_sync() -> *mut wire_opt_vecs_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_opt_vecs_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_record_string_i_32() -> *mut wire_record_string_i_32 {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_record_string_i_32::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sequences() -> *mut wire_sequences {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_sequences::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_some_struct_twin_normal() -> *mut wire_some_struct_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_some_struct_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_some_struct_twin_sync() -> *mut wire_some_struct_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_some_struct_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_normal(
) -> *mut wire_struct_with_comments_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_comments_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_comments_twin_sync(
) -> *mut wire_struct_with_comments_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_comments_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_enum_twin_normal(
) -> *mut wire_struct_with_enum_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_enum_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_enum_twin_sync(
) -> *mut wire_struct_with_enum_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_enum_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_normal(
) -> *mut wire_struct_with_one_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_one_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_one_field_twin_sync(
) -> *mut wire_struct_with_one_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_one_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_normal(
) -> *mut wire_struct_with_two_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_two_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_two_field_twin_sync(
) -> *mut wire_struct_with_two_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_two_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_normal(
) -> *mut wire_struct_with_zero_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_zero_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_struct_with_zero_field_twin_sync(
) -> *mut wire_struct_with_zero_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_struct_with_zero_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with_twin_normal() -> *mut wire_sum_with_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_sum_with_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sum_with_twin_sync() -> *mut wire_sum_with_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_sum_with_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_test_id_twin_normal() -> *mut wire_test_id_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_test_id_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_test_id_twin_sync() -> *mut wire_test_id_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_test_id_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_normal(
) -> *mut wire_tuple_struct_with_one_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_one_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_one_field_twin_sync(
) -> *mut wire_tuple_struct_with_one_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_one_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_normal(
) -> *mut wire_tuple_struct_with_two_field_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_two_field_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_tuple_struct_with_two_field_twin_sync(
) -> *mut wire_tuple_struct_with_two_field_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_tuple_struct_with_two_field_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_16(value: u16) -> *mut u16 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_32(value: u32) -> *mut u32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_64(value: u64) -> *mut u64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u_8(value: u8) -> *mut u8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id_twin_normal() -> *mut wire_user_id_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_user_id_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_id_twin_sync() -> *mut wire_user_id_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_user_id_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_weekdays_twin_normal(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_weekdays_twin_sync(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_blob_twin_normal() -> *mut wire_blob_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_blob_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_blob_twin_sync() -> *mut wire_blob_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_blob_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_bool(value: bool) -> *mut bool {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_distance_twin_normal() -> *mut wire_distance_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_distance_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_distance_twin_sync() -> *mut wire_distance_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_distance_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals_twin_normal() -> *mut wire_exotic_optionals_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_exotic_optionals_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_exotic_optionals_twin_sync() -> *mut wire_exotic_optionals_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_exotic_optionals_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_f_64(value: f64) -> *mut f64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_32(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_64(value: i64) -> *mut i64 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_i_8(value: i8) -> *mut i8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_kitchen_sink_twin_normal() -> *mut wire_kitchen_sink_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_kitchen_sink_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_kitchen_sink_twin_sync() -> *mut wire_kitchen_sink_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_kitchen_sink_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_my_size() -> *mut wire_my_size {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_my_size::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_my_size_freezed_twin_normal() -> *mut wire_my_size_freezed_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_size_freezed_twin_normal::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_my_size_freezed_twin_sync() -> *mut wire_my_size_freezed_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(
        wire_my_size_freezed_twin_sync::new_with_null_ptr(),
    )
}

#[no_mangle]
pub extern "C" fn new_box_speed_twin_normal() -> *mut wire_speed_twin_normal {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_speed_twin_normal::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_speed_twin_sync() -> *mut wire_speed_twin_sync {
    flutter_rust_bridge::support::new_leak_box_ptr(wire_speed_twin_sync::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_u_8(value: u8) -> *mut u8 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_weekdays_twin_normal(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_weekdays_twin_sync(value: i32) -> *mut i32 {
    flutter_rust_bridge::support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_list_DartOpaque(len: i32) -> *mut wire_list_DartOpaque {
    let wrap = wire_list_DartOpaque {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_DartOpaque>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_RustOpaque_hide_data(len: i32) -> *mut wire_list_RustOpaque_hide_data {
    let wrap = wire_list_RustOpaque_hide_data {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_RustOpaque_hide_data>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_application_env_var(len: i32) -> *mut wire_list_application_env_var {
    let wrap = wire_list_application_env_var {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_application_env_var>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_attribute_twin_normal(len: i32) -> *mut wire_list_attribute_twin_normal {
    let wrap = wire_list_attribute_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_attribute_twin_normal>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_attribute_twin_sync(len: i32) -> *mut wire_list_attribute_twin_sync {
    let wrap = wire_list_attribute_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_attribute_twin_sync>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_bool(len: i32) -> *mut wire_list_bool {
    let wrap = wire_list_bool {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_size(len: i32) -> *mut wire_list_my_size {
    let wrap = wire_list_my_size {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_my_size>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node_twin_normal(
    len: i32,
) -> *mut wire_list_my_tree_node_twin_normal {
    let wrap = wire_list_my_tree_node_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_my_tree_node_twin_normal>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_my_tree_node_twin_sync(
    len: i32,
) -> *mut wire_list_my_tree_node_twin_sync {
    let wrap = wire_list_my_tree_node_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_my_tree_node_twin_sync>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_String(len: i32) -> *mut wire_list_opt_String {
    let wrap = wire_list_opt_String {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute_twin_normal(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute_twin_normal {
    let wrap = wire_list_opt_box_autoadd_attribute_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_attribute_twin_sync(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_attribute_twin_sync {
    let wrap = wire_list_opt_box_autoadd_attribute_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_i_32(len: i32) -> *mut wire_list_opt_box_autoadd_i_32 {
    let wrap = wire_list_opt_box_autoadd_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_weekdays_twin_normal(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_weekdays_twin_normal {
    let wrap = wire_list_opt_box_autoadd_weekdays_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_box_autoadd_weekdays_twin_sync(
    len: i32,
) -> *mut wire_list_opt_box_autoadd_weekdays_twin_sync {
    let wrap = wire_list_opt_box_autoadd_weekdays_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_opt_list_prim_i_32(len: i32) -> *mut wire_list_opt_list_prim_i_32 {
    let wrap = wire_list_opt_list_prim_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(core::ptr::null_mut(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_32(len: i32) -> *mut wire_list_prim_f_32 {
    let ans = wire_list_prim_f_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_f_64(len: i32) -> *mut wire_list_prim_f_64 {
    let ans = wire_list_prim_f_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_16(len: i32) -> *mut wire_list_prim_i_16 {
    let ans = wire_list_prim_i_16 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_32(len: i32) -> *mut wire_list_prim_i_32 {
    let ans = wire_list_prim_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_64(len: i32) -> *mut wire_list_prim_i_64 {
    let ans = wire_list_prim_i_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_i_8(len: i32) -> *mut wire_list_prim_i_8 {
    let ans = wire_list_prim_i_8 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_16(len: i32) -> *mut wire_list_prim_u_16 {
    let ans = wire_list_prim_u_16 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_32(len: i32) -> *mut wire_list_prim_u_32 {
    let ans = wire_list_prim_u_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_64(len: i32) -> *mut wire_list_prim_u_64 {
    let ans = wire_list_prim_u_64 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_prim_u_8(len: i32) -> *mut wire_list_prim_u_8 {
    let ans = wire_list_prim_u_8 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_record_string_i_32(len: i32) -> *mut wire_list_record_string_i_32 {
    let wrap = wire_list_record_string_i_32 {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_record_string_i_32>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_test_id_twin_normal(len: i32) -> *mut wire_list_test_id_twin_normal {
    let wrap = wire_list_test_id_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_test_id_twin_normal>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_test_id_twin_sync(len: i32) -> *mut wire_list_test_id_twin_sync {
    let wrap = wire_list_test_id_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(
            <wire_test_id_twin_sync>::new_with_null_ptr(),
            len,
        ),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_weekdays_twin_normal(len: i32) -> *mut wire_list_weekdays_twin_normal {
    let wrap = wire_list_weekdays_twin_normal {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_weekdays_twin_sync(len: i32) -> *mut wire_list_weekdays_twin_sync {
    let wrap = wire_list_weekdays_twin_sync {
        ptr: flutter_rust_bridge::support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    flutter_rust_bridge::support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_MutexHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_MutexHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_RwLockHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_RwLockHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_box_dynDartDebugTwinNormal(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinNormal>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_box_dynDartDebugTwinNormal(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinNormal>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_box_dynDartDebugTwinSync(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinSync>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_box_dynDartDebugTwinSync(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinSync>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_frb_opaque_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_frb_opaque_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_frb_opaque_sync_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_frb_opaque_sync_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<i32>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_i_32(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<i32>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_non_clone_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_non_clone_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_RustOpaque_non_send_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RustOpaque_non_send_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinNormal_A() -> *mut AbcTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinNormalKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinNormal_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinNormal_B() -> *mut AbcTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinNormal_C() -> *mut AbcTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinNormalKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinNormal_C {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinNormal_JustInt() -> *mut AbcTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinNormalKind {
        JustInt: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinNormal_JustInt {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinSync_A() -> *mut AbcTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinSyncKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinSync_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinSync_B() -> *mut AbcTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinSync_C() -> *mut AbcTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinSyncKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinSync_C {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AbcTwinSync_JustInt() -> *mut AbcTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(AbcTwinSyncKind {
        JustInt: flutter_rust_bridge::support::new_leak_box_ptr(wire_AbcTwinSync_JustInt {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Three(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Three: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinNormal_Three {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinNormal_Four(
) -> *mut CustomNestedErrorInnerTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinNormalKind {
        Four: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinNormal_Four {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Three(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Three: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinSync_Three {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorInnerTwinSync_Four(
) -> *mut CustomNestedErrorInnerTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorInnerTwinSyncKind {
        Four: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorInnerTwinSync_Four {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_One(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        One: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinNormal_One {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinNormal_Two(
) -> *mut CustomNestedErrorOuterTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinNormalKind {
        Two: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinNormal_Two {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_One(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        One: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinSync_One {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_CustomNestedErrorOuterTwinSync_Two(
) -> *mut CustomNestedErrorOuterTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(CustomNestedErrorOuterTwinSyncKind {
        Two: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_CustomNestedErrorOuterTwinSync_Two {
                field0: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DistanceTwinNormal_Map() -> *mut DistanceTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(DistanceTwinNormalKind {
        Map: flutter_rust_bridge::support::new_leak_box_ptr(wire_DistanceTwinNormal_Map {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DistanceTwinSync_Map() -> *mut DistanceTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(DistanceTwinSyncKind {
        Map: flutter_rust_bridge::support::new_leak_box_ptr(wire_DistanceTwinSync_Map {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaqueTwinNormal_Primitive() -> *mut EnumDartOpaqueTwinNormalKind
{
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueTwinNormalKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumDartOpaqueTwinNormal_Primitive {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaqueTwinNormal_Opaque() -> *mut EnumDartOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueTwinNormalKind {
        Opaque: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumDartOpaqueTwinNormal_Opaque {
                field0: wire_DartOpaque::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaqueTwinSync_Primitive() -> *mut EnumDartOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueTwinSyncKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumDartOpaqueTwinSync_Primitive {
                field0: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumDartOpaqueTwinSync_Opaque() -> *mut EnumDartOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumDartOpaqueTwinSyncKind {
        Opaque: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumDartOpaqueTwinSync_Opaque {
                field0: wire_DartOpaque::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinNormal_Struct() -> *mut EnumOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinNormalKind {
        Struct: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinNormal_Struct {
            field0: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinNormal_Primitive() -> *mut EnumOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinNormalKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumOpaqueTwinNormal_Primitive {
                field0: wire_RustOpaque_i_32::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinNormal_TraitObj() -> *mut EnumOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinNormalKind {
        TraitObj: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumOpaqueTwinNormal_TraitObj {
                field0: wire_RustOpaque_box_dynDartDebugTwinNormal::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinNormal_Mutex() -> *mut EnumOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinNormalKind {
        Mutex: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinNormal_Mutex {
            field0: wire_RustOpaque_MutexHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinNormal_RwLock() -> *mut EnumOpaqueTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinNormalKind {
        RwLock: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinNormal_RwLock {
            field0: wire_RustOpaque_RwLockHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinSync_Struct() -> *mut EnumOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinSyncKind {
        Struct: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinSync_Struct {
            field0: wire_RustOpaque_hide_data::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinSync_Primitive() -> *mut EnumOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinSyncKind {
        Primitive: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumOpaqueTwinSync_Primitive {
                field0: wire_RustOpaque_i_32::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinSync_TraitObj() -> *mut EnumOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinSyncKind {
        TraitObj: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_EnumOpaqueTwinSync_TraitObj {
                field0: wire_RustOpaque_box_dynDartDebugTwinSync::new_with_null_ptr(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinSync_Mutex() -> *mut EnumOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinSyncKind {
        Mutex: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinSync_Mutex {
            field0: wire_RustOpaque_MutexHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumOpaqueTwinSync_RwLock() -> *mut EnumOpaqueTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumOpaqueTwinSyncKind {
        RwLock: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumOpaqueTwinSync_RwLock {
            field0: wire_RustOpaque_RwLockHideData::new_with_null_ptr(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_B() -> *mut EnumWithItemMixedTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinNormal_C() -> *mut EnumWithItemMixedTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinNormalKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinNormal_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_B() -> *mut EnumWithItemMixedTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemMixedTwinSync_C() -> *mut EnumWithItemMixedTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemMixedTwinSyncKind {
        C: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemMixedTwinSync_C {
            c_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_A() -> *mut EnumWithItemStructTwinNormalKind
{
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinNormal_B() -> *mut EnumWithItemStructTwinNormalKind
{
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinNormal_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_A() -> *mut EnumWithItemStructTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_A {
            a_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemStructTwinSync_B() -> *mut EnumWithItemStructTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemStructTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemStructTwinSync_B {
            b_field: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_A() -> *mut EnumWithItemTupleTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinNormal_B() -> *mut EnumWithItemTupleTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinNormalKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinNormal_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_A() -> *mut EnumWithItemTupleTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        A: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_A {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_EnumWithItemTupleTwinSync_B() -> *mut EnumWithItemTupleTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(EnumWithItemTupleTwinSyncKind {
        B: flutter_rust_bridge::support::new_leak_box_ptr(wire_EnumWithItemTupleTwinSync_B {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinNormal_Primitives() -> *mut KitchenSinkTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinNormalKind {
        Primitives: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_KitchenSinkTwinNormal_Primitives {
                int32: Default::default(),
                float64: Default::default(),
                boolean: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinNormal_Nested() -> *mut KitchenSinkTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinNormalKind {
        Nested: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinNormal_Nested {
            field0: Default::default(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinNormal_Optional() -> *mut KitchenSinkTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinNormalKind {
        Optional: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_KitchenSinkTwinNormal_Optional {
                field0: core::ptr::null_mut(),
                field1: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinNormal_Buffer() -> *mut KitchenSinkTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinNormalKind {
        Buffer: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinNormal_Buffer {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinNormal_Enums() -> *mut KitchenSinkTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinNormalKind {
        Enums: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinNormal_Enums {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinSync_Primitives() -> *mut KitchenSinkTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinSyncKind {
        Primitives: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_KitchenSinkTwinSync_Primitives {
                int32: Default::default(),
                float64: Default::default(),
                boolean: Default::default(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinSync_Nested() -> *mut KitchenSinkTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinSyncKind {
        Nested: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinSync_Nested {
            field0: Default::default(),
            field1: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinSync_Optional() -> *mut KitchenSinkTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinSyncKind {
        Optional: flutter_rust_bridge::support::new_leak_box_ptr(
            wire_KitchenSinkTwinSync_Optional {
                field0: core::ptr::null_mut(),
                field1: core::ptr::null_mut(),
            },
        ),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinSync_Buffer() -> *mut KitchenSinkTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinSyncKind {
        Buffer: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinSync_Buffer {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_KitchenSinkTwinSync_Enums() -> *mut KitchenSinkTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(KitchenSinkTwinSyncKind {
        Enums: flutter_rust_bridge::support::new_leak_box_ptr(wire_KitchenSinkTwinSync_Enums {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MeasureTwinNormal_Speed() -> *mut MeasureTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureTwinNormalKind {
        Speed: flutter_rust_bridge::support::new_leak_box_ptr(wire_MeasureTwinNormal_Speed {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MeasureTwinNormal_Distance() -> *mut MeasureTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureTwinNormalKind {
        Distance: flutter_rust_bridge::support::new_leak_box_ptr(wire_MeasureTwinNormal_Distance {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MeasureTwinSync_Speed() -> *mut MeasureTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureTwinSyncKind {
        Speed: flutter_rust_bridge::support::new_leak_box_ptr(wire_MeasureTwinSync_Speed {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_MeasureTwinSync_Distance() -> *mut MeasureTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(MeasureTwinSyncKind {
        Distance: flutter_rust_bridge::support::new_leak_box_ptr(wire_MeasureTwinSync_Distance {
            field0: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_SpeedTwinNormal_GPS() -> *mut SpeedTwinNormalKind {
    flutter_rust_bridge::support::new_leak_box_ptr(SpeedTwinNormalKind {
        GPS: flutter_rust_bridge::support::new_leak_box_ptr(wire_SpeedTwinNormal_GPS {
            field0: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_SpeedTwinSync_GPS() -> *mut SpeedTwinSyncKind {
    flutter_rust_bridge::support::new_leak_box_ptr(SpeedTwinSyncKind {
        GPS: flutter_rust_bridge::support::new_leak_box_ptr(wire_SpeedTwinSync_GPS {
            field0: Default::default(),
        }),
    })
}
