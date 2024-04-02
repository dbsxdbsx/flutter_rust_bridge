// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.29.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception_twin_sse.freezed.dart';

Future<int> funcReturnErrorTwinSse({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinSse(hint: hint);

Future<int> funcTypeFalliblePanicTwinSse({dynamic hint}) =>
    RustLib.instance.api.funcTypeFalliblePanicTwinSse(hint: hint);

Future<int> funcTypeInfalliblePanicTwinSse({dynamic hint}) =>
    RustLib.instance.api.funcTypeInfalliblePanicTwinSse(hint: hint);

Future<int> customEnumErrorReturnOkTwinSse({required int arg, dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnOkTwinSse(arg: arg, hint: hint);

Future<void> customEnumErrorPanicTwinSse({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorPanicTwinSse(hint: hint);

Future<int> customEnumErrorReturnErrorTwinSse({dynamic hint}) =>
    RustLib.instance.api.customEnumErrorReturnErrorTwinSse(hint: hint);

Future<void> customNestedErrorReturnErrorTwinSse(
        {required CustomNestedErrorOuterTwinSse arg, dynamic hint}) =>
    RustLib.instance.api
        .customNestedErrorReturnErrorTwinSse(arg: arg, hint: hint);

Future<void> customStructErrorReturnErrorTwinSse(
        {required CustomStructErrorTwinSse arg, dynamic hint}) =>
    RustLib.instance.api
        .customStructErrorReturnErrorTwinSse(arg: arg, hint: hint);

Future<int> returnErrCustomErrorTwinSse({dynamic hint}) =>
    RustLib.instance.api.returnErrCustomErrorTwinSse(hint: hint);

Future<int> returnOkCustomErrorTwinSse({dynamic hint}) =>
    RustLib.instance.api.returnOkCustomErrorTwinSse(hint: hint);

Future<int> returnErrorVariantTwinSse({required int variant, dynamic hint}) =>
    RustLib.instance.api
        .returnErrorVariantTwinSse(variant: variant, hint: hint);

Future<void> returnCustomNestedError1TwinSse({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1TwinSse(hint: hint);

Future<void> returnCustomNestedError1Variant1TwinSse({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError1Variant1TwinSse(hint: hint);

Future<void> returnCustomNestedError2TwinSse({dynamic hint}) =>
    RustLib.instance.api.returnCustomNestedError2TwinSse(hint: hint);

Future<void> returnCustomStructErrorTwinSse({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructErrorTwinSse(hint: hint);

Future<int> returnCustomStructOkTwinSse({dynamic hint}) =>
    RustLib.instance.api.returnCustomStructOkTwinSse(hint: hint);

Future<void> throwAnyhowTwinSse({dynamic hint}) =>
    RustLib.instance.api.throwAnyhowTwinSse(hint: hint);

Future<void> panicWithCustomResultTwinSse({dynamic hint}) =>
    RustLib.instance.api.panicWithCustomResultTwinSse(hint: hint);

Stream<String> streamSinkThrowAnyhowTwinSse({dynamic hint}) =>
    RustLib.instance.api.streamSinkThrowAnyhowTwinSse(hint: hint);

@freezed
sealed class CustomEnumErrorTwinSse
    with _$CustomEnumErrorTwinSse
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSse.one({
    required String message,
    required String backtrace,
  }) = CustomEnumErrorTwinSse_One;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinSse.two({
    required int message,
    required String backtrace,
  }) = CustomEnumErrorTwinSse_Two;
}

@freezed
sealed class CustomErrorTwinSse
    with _$CustomErrorTwinSse
    implements FrbException {
  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinSse.error0({
    required String e,
    required String backtrace,
  }) = CustomErrorTwinSse_Error0;
  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinSse.error1({
    required int e,
    required String backtrace,
  }) = CustomErrorTwinSse_Error1;
}

@freezed
sealed class CustomNestedError1TwinSse
    with _$CustomNestedError1TwinSse
    implements FrbException {
  const factory CustomNestedError1TwinSse.customNested1(
    String field0,
  ) = CustomNestedError1TwinSse_CustomNested1;
  const factory CustomNestedError1TwinSse.errorNested(
    CustomNestedError2TwinSse field0,
  ) = CustomNestedError1TwinSse_ErrorNested;
}

@freezed
sealed class CustomNestedError2TwinSse with _$CustomNestedError2TwinSse {
  const factory CustomNestedError2TwinSse.customNested2(
    String field0,
  ) = CustomNestedError2TwinSse_CustomNested2;
  const factory CustomNestedError2TwinSse.customNested2Number(
    int field0,
  ) = CustomNestedError2TwinSse_CustomNested2Number;
}

@freezed
sealed class CustomNestedErrorInnerTwinSse
    with _$CustomNestedErrorInnerTwinSse {
  const factory CustomNestedErrorInnerTwinSse.three(
    String field0,
  ) = CustomNestedErrorInnerTwinSse_Three;
  const factory CustomNestedErrorInnerTwinSse.four(
    int field0,
  ) = CustomNestedErrorInnerTwinSse_Four;
}

@freezed
sealed class CustomNestedErrorOuterTwinSse
    with _$CustomNestedErrorOuterTwinSse {
  const factory CustomNestedErrorOuterTwinSse.one(
    String field0,
  ) = CustomNestedErrorOuterTwinSse_One;
  const factory CustomNestedErrorOuterTwinSse.two(
    CustomNestedErrorInnerTwinSse field0,
  ) = CustomNestedErrorOuterTwinSse_Two;
}

class CustomStructErrorAnotherTwinSse implements FrbException {
  final String message;

  const CustomStructErrorAnotherTwinSse({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorAnotherTwinSse &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class CustomStructErrorTwinSse {
  final String a;

  const CustomStructErrorTwinSse({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorTwinSse &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class CustomStructTwinSse {
  final String message;

  const CustomStructTwinSse({
    required this.message,
  });

  static Future<CustomStructTwinSse> newTwinSse(
          {required String message, dynamic hint}) =>
      RustLib.instance.api.customStructTwinSseNewTwinSse(
        message: message,
        hint: hint,
      );

  Future<void> nonstaticReturnCustomStructErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSseNonstaticReturnCustomStructErrorTwinSse(
        that: this,
        hint: hint,
      );

  Future<int> nonstaticReturnCustomStructOkTwinSse({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSseNonstaticReturnCustomStructOkTwinSse(
        that: this,
        hint: hint,
      );

  static Future<void> staticReturnCustomStructErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api
          .customStructTwinSseStaticReturnCustomStructErrorTwinSse(
        hint: hint,
      );

  static Future<int> staticReturnCustomStructOkTwinSse({dynamic hint}) =>
      RustLib.instance.api.customStructTwinSseStaticReturnCustomStructOkTwinSse(
        hint: hint,
      );

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructTwinSse &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class SomeStructTwinSse {
  final int value;

  const SomeStructTwinSse({
    required this.value,
  });

  static Future<SomeStructTwinSse> newTwinSse(
          {required int value, dynamic hint}) =>
      RustLib.instance.api.someStructTwinSseNewTwinSse(
        value: value,
        hint: hint,
      );

  Future<int> nonStaticReturnErrCustomErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api
          .someStructTwinSseNonStaticReturnErrCustomErrorTwinSse(
        that: this,
        hint: hint,
      );

  Future<int> nonStaticReturnOkCustomErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api.someStructTwinSseNonStaticReturnOkCustomErrorTwinSse(
        that: this,
        hint: hint,
      );

  static Future<int> staticReturnErrCustomErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api.someStructTwinSseStaticReturnErrCustomErrorTwinSse(
        hint: hint,
      );

  static Future<int> staticReturnOkCustomErrorTwinSse({dynamic hint}) =>
      RustLib.instance.api.someStructTwinSseStaticReturnOkCustomErrorTwinSse(
        hint: hint,
      );

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SomeStructTwinSse &&
          runtimeType == other.runtimeType &&
          value == other.value;
}
