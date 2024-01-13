// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `basic_map_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_map_twin_sync_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_map', () {
    addTestsIdentityFunctionCall(exampleBasicMapTypeI8TwinSyncSse, [
      {},
      {42: 0},
      {42: -128},
      {42: 127}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeI16TwinSyncSse, [
      {},
      {42: 0},
      {42: -32768},
      {42: 32767}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeI32TwinSyncSse, [
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeI64TwinSyncSse, [
      {},
      {42: 0},
      {42: -9007199254740992},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeU8TwinSyncSse, [
      {},
      {42: 0},
      {42: 255}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeU16TwinSyncSse, [
      {},
      {42: 0},
      {42: 65535}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeU32TwinSyncSse, [
      {},
      {42: 0},
      {42: 4294967295}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeU64TwinSyncSse, [
      {},
      {42: 0},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeIsizeTwinSyncSse, [
      {},
      {42: 0},
      {42: -2147483648},
      {42: 2147483647},
      {42: -9007199254740992},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeUsizeTwinSyncSse, [
      {},
      {42: 0},
      {42: 4294967295},
      {42: 9007199254740992}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeF32TwinSyncSse, [
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeF64TwinSyncSse, [
      {},
      {42: 0},
      {42: -42.5},
      {42: 123456}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeBoolTwinSyncSse, [
      {},
      {42: false},
      {42: true}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeStringTwinSyncSse, [
      {},
      {42: ""},
      {42: "hello"},
      {42: "😂"}
    ]);
    addTestsIdentityFunctionCall(exampleBasicMapTypeBytesTwinSyncSse, [
      {},
      {42: Uint8List.fromList([])},
      {
        42: Uint8List.fromList([255, 0])
      },
      {
        42: Uint8List.fromList([10, 20, 30, 40])
      }
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicPrimitiveEnumTwinSyncSseTwinSyncSse, [
      {},
      {42: BasicPrimitiveEnumTwinSyncSse.apple},
      {42: BasicPrimitiveEnumTwinSyncSse.orange}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicGeneralEnumTwinSyncSseTwinSyncSse, [
      {},
      {42: BasicGeneralEnumTwinSyncSse.apple(field: "one")},
      {42: BasicGeneralEnumTwinSyncSse.orange()}
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicMapTypeBasicStructTwinSyncSseTwinSyncSse, [
      {},
      {42: BasicStructTwinSyncSse(apple: null, orange: null)},
      {42: BasicStructTwinSyncSse(apple: "one", orange: 42)}
    ]);
  });
}
