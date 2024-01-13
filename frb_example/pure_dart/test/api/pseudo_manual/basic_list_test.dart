// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/basic_list.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import '../../test_utils.dart';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('basic_list', () {
    addTestsIdentityFunctionCall(exampleBasicListTypeI8TwinNormal, [
      Int8List.fromList([]),
      Int8List.fromList([0]),
      Int8List.fromList([-128]),
      Int8List.fromList([127])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeI16TwinNormal, [
      Int16List.fromList([]),
      Int16List.fromList([0]),
      Int16List.fromList([-32768]),
      Int16List.fromList([32767])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeI32TwinNormal, [
      Int32List.fromList([]),
      Int32List.fromList([0]),
      Int32List.fromList([-2147483648]),
      Int32List.fromList([2147483647])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeI64TwinNormal, [
      Int64List.fromList([]),
      Int64List.fromList([0]),
      Int64List.fromList([-9007199254740992]),
      Int64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeU8TwinNormal, [
      Uint8List.fromList([]),
      Uint8List.fromList([0]),
      Uint8List.fromList([255])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeU16TwinNormal, [
      Uint16List.fromList([]),
      Uint16List.fromList([0]),
      Uint16List.fromList([65535])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeU32TwinNormal, [
      Uint32List.fromList([]),
      Uint32List.fromList([0]),
      Uint32List.fromList([4294967295])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeU64TwinNormal, [
      Uint64List.fromList([]),
      Uint64List.fromList([0]),
      Uint64List.fromList([9007199254740992])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeF32TwinNormal, [
      Float32List.fromList([]),
      Float32List.fromList([0]),
      Float32List.fromList([-42.5]),
      Float32List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeF64TwinNormal, [
      Float64List.fromList([]),
      Float64List.fromList([0]),
      Float64List.fromList([-42.5]),
      Float64List.fromList([123456])
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeBoolTwinNormal, [
      <bool>[],
      <bool>[false],
      <bool>[true]
    ]);
    addTestsIdentityFunctionCall(exampleBasicListTypeStringTwinNormal, [
      null.fromList([]),
      null.fromList([""]),
      null.fromList(["hello"]),
      null.fromList(["😂"])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicPrimitiveEnumTwinNormalTwinNormal, [
      null.fromList([]),
      null.fromList([BasicPrimitiveEnumTwinNormal.apple]),
      null.fromList([BasicPrimitiveEnumTwinNormal.orange])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicGeneralEnumTwinNormalTwinNormal, [
      null.fromList([]),
      null.fromList([BasicGeneralEnumTwinNormal.apple(field: "one")]),
      null.fromList([BasicGeneralEnumTwinNormal.orange()])
    ]);
    addTestsIdentityFunctionCall(
        exampleBasicListTypeBasicStructTwinNormalTwinNormal, [
      null.fromList([]),
      null.fromList([BasicStructTwinNormal(apple: null, orange: null)]),
      null.fromList([BasicStructTwinNormal(apple: "one", orange: 42)])
    ]);
  });
}
