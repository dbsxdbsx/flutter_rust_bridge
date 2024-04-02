// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.29.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_auto_opaque_twin_moi.freezed.dart';

// The type `HelloOneStructTwinMoi` is not used by any `pub` functions, thus it is ignored.
// The type `HelloTwoEnumTwinMoi` is not used by any `pub` functions, thus it is ignored.

Future<void> rustAutoOpaqueArgOwnTwinMoi(
        {required NonCloneSimpleTwinMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnTwinMoi(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgBorrowTwinMoi(
        {required NonCloneSimpleTwinMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgBorrowTwinMoi(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueArgMutBorrowTwinMoi(
        {required NonCloneSimpleTwinMoi arg,
        required int expect,
        required int adder,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueArgMutBorrowTwinMoi(
        arg: arg, expect: expect, adder: adder, hint: hint);

Future<NonCloneSimpleTwinMoi> rustAutoOpaqueReturnOwnTwinMoi(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueReturnOwnTwinMoi(initial: initial, hint: hint);

Future<NonCloneSimpleTwinMoi> rustAutoOpaqueArgOwnAndReturnOwnTwinMoi(
        {required NonCloneSimpleTwinMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgOwnAndReturnOwnTwinMoi(arg: arg, hint: hint);

Future<void> rustAutoOpaqueTwoArgsTwinMoi(
        {required NonCloneSimpleTwinMoi a,
        required NonCloneSimpleTwinMoi b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTwoArgsTwinMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueNormalAndOpaqueArgTwinMoi(
        {required NonCloneSimpleTwinMoi a, required String b, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueNormalAndOpaqueArgTwinMoi(a: a, b: b, hint: hint);

/// "+" inside the type signature
Future<void> rustAutoOpaquePlusSignArgTwinMoi(
        {required BoxMyTraitTwinMoi arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignArgTwinMoi(arg: arg, hint: hint);

Future<BoxMyTraitTwinMoi> rustAutoOpaquePlusSignReturnTwinMoi({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaquePlusSignReturnTwinMoi(hint: hint);

Future<void> rustAutoOpaqueCallableArgTwinMoi(
        {required BoxFnStringString arg, dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableArgTwinMoi(arg: arg, hint: hint);

Future<BoxFnStringString> rustAutoOpaqueCallableReturnTwinMoi({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueCallableReturnTwinMoi(hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgOwnTwinMoi(
        {required BoxHelloTraitTwinMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgOwnTwinMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgBorrowTwinMoi(
        {required BoxHelloTraitTwinMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgBorrowTwinMoi(
        arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueTraitObjectArgMutBorrowTwinMoi(
        {required BoxHelloTraitTwinMoi arg,
        required String expect,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueTraitObjectArgMutBorrowTwinMoi(
        arg: arg, expect: expect, hint: hint);

Future<BoxHelloTraitTwinMoi> rustAutoOpaqueTraitObjectReturnOwnOneTwinMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnOneTwinMoi(hint: hint);

Future<BoxHelloTraitTwinMoi> rustAutoOpaqueTraitObjectReturnOwnTwoTwinMoi(
        {dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueTraitObjectReturnOwnTwoTwinMoi(hint: hint);

Future<void> rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinMoi(
        {required StructWithGoodAndOpaqueFieldTwinMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueStructWithGoodAndOpaqueFieldArgOwnTwinMoi(
            arg: arg, hint: hint);

Future<StructWithGoodAndOpaqueFieldTwinMoi>
    rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinMoi(
            {dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueStructWithGoodAndOpaqueFieldReturnOwnTwinMoi(
                hint: hint);

Future<void> rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinMoi(
        {required EnumWithGoodAndOpaqueTwinMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueEnumWithGoodAndOpaqueArgOwnTwinMoi(arg: arg, hint: hint);

Future<EnumWithGoodAndOpaqueTwinMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnGoodTwinMoi(
                hint: hint);

Future<EnumWithGoodAndOpaqueTwinMoi>
    rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueEnumWithGoodAndOpaqueReturnOwnOpaqueTwinMoi(
                hint: hint);

Future<void> rustAutoOpaqueDummyTwinMoi(
        {required StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi a,
        required EnumWithGoodAndOpaqueWithoutOptionTwinMoi b,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueDummyTwinMoi(a: a, b: b, hint: hint);

Future<void> rustAutoOpaqueEnumArgBorrowTwinMoi(
        {required NonCloneSimpleEnumTwinMoi arg, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueEnumArgBorrowTwinMoi(arg: arg, hint: hint);

Future<NonCloneSimpleEnumTwinMoi> rustAutoOpaqueEnumReturnOwnTwinMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueEnumReturnOwnTwinMoi(hint: hint);

Stream<NonCloneSimpleTwinMoi> rustAutoOpaqueStreamSinkTwinMoi({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueStreamSinkTwinMoi(hint: hint);

Future<void> rustAutoOpaqueArgVecOwnTwinMoi(
        {required List<NonCloneSimpleTwinMoi> arg,
        required List<int> expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueArgVecOwnTwinMoi(arg: arg, expect: expect, hint: hint);

Future<List<NonCloneSimpleTwinMoi>> rustAutoOpaqueReturnVecOwnTwinMoi(
        {dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnVecOwnTwinMoi(hint: hint);

Future<void> rustAutoOpaqueExplicitArgTwinMoi(
        {required NonCloneSimpleTwinMoi arg,
        required int expect,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitArgTwinMoi(arg: arg, expect: expect, hint: hint);

Future<void> rustAutoOpaqueExplicitStructTwinMoi(
        {required StructWithExplicitAutoOpaqueFieldTwinMoi arg,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitStructTwinMoi(arg: arg, hint: hint);

Future<NonCloneSimpleTwinMoi> rustAutoOpaqueExplicitReturnTwinMoi(
        {required int initial, dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueExplicitReturnTwinMoi(initial: initial, hint: hint);

Future<(OpaqueOneTwinMoi, OpaqueTwoTwinMoi)>
    rustAutoOpaqueReturnOpaqueOneAndTwoTwinMoi({dynamic hint}) =>
        RustLib.instance.api
            .rustAutoOpaqueReturnOpaqueOneAndTwoTwinMoi(hint: hint);

Future<OpaqueTwoTwinMoi> rustAutoOpaqueReturnOpaqueTwoTwinMoi({dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueReturnOpaqueTwoTwinMoi(hint: hint);

Future<int> rustAutoOpaqueBorrowAndMutBorrowTwinMoi(
        {required NonCloneSimpleTwinMoi borrow,
        required NonCloneSimpleTwinMoi mutBorrow,
        dynamic hint}) =>
    RustLib.instance.api.rustAutoOpaqueBorrowAndMutBorrowTwinMoi(
        borrow: borrow, mutBorrow: mutBorrow, hint: hint);

Future<int> rustAutoOpaqueBorrowAndBorrowTwinMoi(
        {required NonCloneSimpleTwinMoi a,
        required NonCloneSimpleTwinMoi b,
        dynamic hint}) =>
    RustLib.instance.api
        .rustAutoOpaqueBorrowAndBorrowTwinMoi(a: a, b: b, hint: hint);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn Fn (String) -> String + Send + Sync >>>
@sealed
class BoxFnStringString extends RustOpaque {
  BoxFnStringString.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxFnStringString.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_BoxFnStringString,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_BoxFnStringString,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxFnStringStringPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn HelloTraitTwinMoi >>>
@sealed
class BoxHelloTraitTwinMoi extends RustOpaque {
  BoxHelloTraitTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxHelloTraitTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_BoxHelloTraitTwinMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxHelloTraitTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxHelloTraitTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<Box < dyn MyTraitTwinMoi + Send + Sync >>>
@sealed
class BoxMyTraitTwinMoi extends RustOpaque {
  BoxMyTraitTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  BoxMyTraitTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_BoxMyTraitTwinMoi,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_BoxMyTraitTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<EnumWithGoodAndOpaqueWithoutOptionTwinMoi>>
@sealed
class EnumWithGoodAndOpaqueWithoutOptionTwinMoi extends RustOpaque {
  EnumWithGoodAndOpaqueWithoutOptionTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  EnumWithGoodAndOpaqueWithoutOptionTwinMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_EnumWithGoodAndOpaqueWithoutOptionTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleEnumTwinMoi>>
@sealed
class NonCloneSimpleEnumTwinMoi extends RustOpaque {
  NonCloneSimpleEnumTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleEnumTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_NonCloneSimpleEnumTwinMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_NonCloneSimpleEnumTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<NonCloneSimpleTwinMoi>>
@sealed
class NonCloneSimpleTwinMoi extends RustOpaque {
  NonCloneSimpleTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  NonCloneSimpleTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib
        .instance.api.rust_arc_increment_strong_count_NonCloneSimpleTwinMoi,
    rustArcDecrementStrongCount: RustLib
        .instance.api.rust_arc_decrement_strong_count_NonCloneSimpleTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_NonCloneSimpleTwinMoiPtr,
  );

  Future<void> instanceMethodArgBorrowTwinMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiInstanceMethodArgBorrowTwinMoi(
        that: this,
        hint: hint,
      );

  Future<void> instanceMethodArgMutBorrowTwinMoi({dynamic hint}) =>
      RustLib.instance.api
          .nonCloneSimpleTwinMoiInstanceMethodArgMutBorrowTwinMoi(
        that: this,
        hint: hint,
      );

  Future<void> instanceMethodArgOwnTwinMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiInstanceMethodArgOwnTwinMoi(
        that: this,
        hint: hint,
      );

  Future<int> get instanceMethodGetterTwinMoi =>
      RustLib.instance.api.nonCloneSimpleTwinMoiInstanceMethodGetterTwinMoi(
        that: this,
      );

  Future<NonCloneSimpleTwinMoi> instanceMethodReturnOwnTwinMoi(
          {dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiInstanceMethodReturnOwnTwinMoi(
        that: this,
        hint: hint,
      );

  /// named constructor
  static Future<NonCloneSimpleTwinMoi> newCustomNameTwinMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiNewCustomNameTwinMoi(
        hint: hint,
      );

  /// unnamed constructor
  static Future<NonCloneSimpleTwinMoi> newTwinMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiNewTwinMoi(
        hint: hint,
      );

  /// constructor with Result
  static Future<NonCloneSimpleTwinMoi> newWithResultTwinMoi({dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiNewWithResultTwinMoi(
        hint: hint,
      );

  static Future<void> staticMethodArgBorrowTwinMoi(
          {required NonCloneSimpleTwinMoi arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiStaticMethodArgBorrowTwinMoi(
        arg: arg,
        hint: hint,
      );

  static Future<void> staticMethodArgMutBorrowTwinMoi(
          {required NonCloneSimpleTwinMoi arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiStaticMethodArgMutBorrowTwinMoi(
        arg: arg,
        hint: hint,
      );

  static Future<void> staticMethodArgOwnTwinMoi(
          {required NonCloneSimpleTwinMoi arg, dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiStaticMethodArgOwnTwinMoi(
        arg: arg,
        hint: hint,
      );

  static Future<NonCloneSimpleTwinMoi> staticMethodReturnOwnTwinMoi(
          {dynamic hint}) =>
      RustLib.instance.api.nonCloneSimpleTwinMoiStaticMethodReturnOwnTwinMoi(
        hint: hint,
      );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueOneTwinMoi>>
@sealed
class OpaqueOneTwinMoi extends RustOpaque {
  OpaqueOneTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueOneTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_OpaqueOneTwinMoi,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_OpaqueOneTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueOneTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<OpaqueTwoTwinMoi>>
@sealed
class OpaqueTwoTwinMoi extends RustOpaque {
  OpaqueTwoTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  OpaqueTwoTwinMoi.sseDecode(int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_OpaqueTwoTwinMoi,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib
        .instance.api.rust_arc_decrement_strong_count_OpaqueTwoTwinMoiPtr,
  );
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::rust_async::RwLock<StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi>>
@sealed
class StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi extends RustOpaque {
  StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi.dcoDecode(List<dynamic> wire)
      : super.dcoDecode(wire, _kStaticData);

  StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi.sseDecode(
      int ptr, int externalSizeOnNative)
      : super.sseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount: RustLib.instance.api
        .rust_arc_increment_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi,
    rustArcDecrementStrongCount: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinMoi,
    rustArcDecrementStrongCountPtr: RustLib.instance.api
        .rust_arc_decrement_strong_count_StructWithGoodAndOpaqueFieldWithoutOptionTwinMoiPtr,
  );
}

@freezed
sealed class EnumWithGoodAndOpaqueTwinMoi with _$EnumWithGoodAndOpaqueTwinMoi {
  const factory EnumWithGoodAndOpaqueTwinMoi.good(
    String field0,
  ) = EnumWithGoodAndOpaqueTwinMoi_Good;
  const factory EnumWithGoodAndOpaqueTwinMoi.opaque(
    NonCloneSimpleTwinMoi field0,
  ) = EnumWithGoodAndOpaqueTwinMoi_Opaque;
}

class StructWithExplicitAutoOpaqueFieldTwinMoi {
  final NonCloneSimpleTwinMoi autoOpaque;
  final int normal;

  const StructWithExplicitAutoOpaqueFieldTwinMoi({
    required this.autoOpaque,
    required this.normal,
  });

  @override
  int get hashCode => autoOpaque.hashCode ^ normal.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithExplicitAutoOpaqueFieldTwinMoi &&
          runtimeType == other.runtimeType &&
          autoOpaque == other.autoOpaque &&
          normal == other.normal;
}

class StructWithGoodAndOpaqueFieldTwinMoi {
  final String good;
  final NonCloneSimpleTwinMoi opaque;
  final NonCloneSimpleTwinMoi? optionOpaque;

  const StructWithGoodAndOpaqueFieldTwinMoi({
    required this.good,
    required this.opaque,
    this.optionOpaque,
  });

  @override
  int get hashCode => good.hashCode ^ opaque.hashCode ^ optionOpaque.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithGoodAndOpaqueFieldTwinMoi &&
          runtimeType == other.runtimeType &&
          good == other.good &&
          opaque == other.opaque &&
          optionOpaque == other.optionOpaque;
}
