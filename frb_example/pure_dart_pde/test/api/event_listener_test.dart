// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:frb_example_pure_dart_pde/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    unawaited(expectLater(registerEventListenerTwinNormal(),
        emits(EventTwinNormal(address: 'foo', payload: 'bar'))));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEventTwinNormal(address: 'foo', payload: 'bar');
    await closeEventListenerTwinNormal();
  });

  // #1836
  test('when send event before async gap, should receive it', () async {
    final logs = <String>[];

    final stream = registerEventListenerTwinNormal();
    stream.listen((event) => logs.add(event.address));

    // main call to test #1836
    createEventSyncTwinNormal(address: 'one', payload: '');

    await Future.delayed(Duration.zero);
    createEventSyncTwinNormal(address: 'two', payload: '');

    await closeEventListenerTwinNormal();

    expect(logs, ['one', 'two']);
  });

  // #1836
  test('when Rust send event after Dart close stream', () async {
    final stream = registerEventListenerTwinNormal();
    await Future.delayed(Duration.zero);
    final subscription = stream.listen((_) {});
    await Future.delayed(Duration.zero);
    unawaited(subscription.cancel());
    createEventSyncTwinNormal(address: '1', payload: '');
  });
}
