// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.38.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field

// Static analysis wrongly picks the IO variant, thus ignore this
// ignore_for_file: argument_type_not_assignable

import 'api/minimal.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';

abstract class RustLibApiImplPlatform extends BaseApiImpl<RustLibWire> {
  RustLibApiImplPlatform({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @protected
  int dco_decode_CastedPrimitive_u_64(dynamic raw);

  @protected
  int dco_decode_i_32(dynamic raw);

  @protected
  BigInt dco_decode_u_64(dynamic raw);

  @protected
  void dco_decode_unit(dynamic raw);

  @protected
  int sse_decode_CastedPrimitive_u_64(SseDeserializer deserializer);

  @protected
  int sse_decode_i_32(SseDeserializer deserializer);

  @protected
  BigInt sse_decode_u_64(SseDeserializer deserializer);

  @protected
  void sse_decode_unit(SseDeserializer deserializer);

  @protected
  bool sse_decode_bool(SseDeserializer deserializer);

  @protected
  Object cst_encode_CastedPrimitive_u_64(int raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    throw UnimplementedError(
        'Not implemented in this codec, please use the other one');
  }

  @protected
  Object cst_encode_u_64(BigInt raw) {
    // Codec=Cst (C-struct based), see doc to use other codecs
    return castNativeBigInt(raw);
  }

  @protected
  int cst_encode_i_32(int raw);

  @protected
  void cst_encode_unit(void raw);

  @protected
  void sse_encode_CastedPrimitive_u_64(int self, SseSerializer serializer);

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer);

  @protected
  void sse_encode_u_64(BigInt self, SseSerializer serializer);

  @protected
  void sse_encode_unit(void self, SseSerializer serializer);

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer);
}

// Section: wire_class

class RustLibWire implements BaseWire {
  RustLibWire.fromExternalLibrary(ExternalLibrary lib);

  void wire__crate__api__minimal__f(NativePortType port_, Object arg) =>
      wasmModule.wire__crate__api__minimal__f(port_, arg);

  void wire__crate__api__minimal__init_app(NativePortType port_) =>
      wasmModule.wire__crate__api__minimal__init_app(port_);

  void wire__crate__api__minimal__minimal_adder(
          NativePortType port_, int a, int b) =>
      wasmModule.wire__crate__api__minimal__minimal_adder(port_, a, b);
}

@JS('wasm_bindgen')
external RustLibWasmModule get wasmModule;

@JS()
@anonymous
class RustLibWasmModule {
  external void wire__crate__api__minimal__f(NativePortType port_, Object arg);

  external void wire__crate__api__minimal__init_app(NativePortType port_);

  external void wire__crate__api__minimal__minimal_adder(
      NativePortType port_, int a, int b);
}
