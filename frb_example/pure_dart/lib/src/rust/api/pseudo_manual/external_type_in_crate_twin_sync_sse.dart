// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.29.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/new_module_system/sub_module.dart';
import '../../auxiliary/old_module_system/sub_module.dart';
import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

bool useImportedStructTwinSyncSse({required MyStruct myStruct, dynamic hint}) =>
    RustLib.instance.api
        .useImportedStructTwinSyncSse(myStruct: myStruct, hint: hint);

bool useImportedEnumTwinSyncSse({required MyEnum myEnum, dynamic hint}) =>
    RustLib.instance.api.useImportedEnumTwinSyncSse(myEnum: myEnum, hint: hint);

OldSimpleStruct callOldModuleSystemTwinSyncSse({dynamic hint}) =>
    RustLib.instance.api.callOldModuleSystemTwinSyncSse(hint: hint);

NewSimpleStruct callNewModuleSystemTwinSyncSse({dynamic hint}) =>
    RustLib.instance.api.callNewModuleSystemTwinSyncSse(hint: hint);
