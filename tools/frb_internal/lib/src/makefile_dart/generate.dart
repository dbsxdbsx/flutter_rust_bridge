import 'package:args/command_runner.dart';
import 'package:build_cli_annotations/build_cli_annotations.dart';
import 'package:flutter_rust_bridge_internal/src/frb_example_pure_dart_generator/generator.dart'
    as frb_example_pure_dart_generator;
import 'package:flutter_rust_bridge_internal/src/makefile_dart/misc.dart';
import 'package:flutter_rust_bridge_internal/src/utils/makefile_dart_infra.dart';

part 'generate.g.dart';

List<Command<void>> createCommands() {
  return [
    SimpleConfigCommand('generate', generate, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-internal', generateInternal, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
    SimpleConfigCommand(
        'generate-run-frb-codegen', generateRunFrbCodegen, _$populateGenerateConfigParser, _$parseGenerateConfigResult),
  ];
}

@CliOptions()
class GenerateConfig {
  @CliOption(defaultsTo: false)
  final bool setExitIfChanged;

  const GenerateConfig({
    required this.setExitIfChanged,
  });
}

Future<void> generate(GenerateConfig config) async {
  await generateInternal(config);
  await generateRunFrbCodegen(config);
}

Future<void> generateInternal(GenerateConfig config) async {
  await generateInternalFrbExamplePureDart(config);
  await generateInternalDartSource(config);
  await generateInternalRust(config);
  await generateInternalBookHelp(config);
  await generateInternalBuildRunner(config);
}

Future<void> generateInternalFrbExamplePureDart(GenerateConfig config) async {
  await frb_example_pure_dart_generator.generate();
  await _maybeSetExitIfChanged(config);
}

Future<void> generateInternalDartSource(GenerateConfig config) async {
  await exec('''
    #!/usr/bin/env bash
    set -euxo pipefail

    git clone --depth 1 --filter=blob:none --sparse --branch stable https://github.com/dart-lang/sdk.git
    cd sdk
    git sparse-checkout set runtime/include
    cd ..
    cp -rf ./sdk/runtime/include/* ./frb_rust/src/dart_api/
    rm -r sdk
  ''');
  await _maybeSetExitIfChanged(config);
}

Future<void> generateInternalRust(GenerateConfig config) async {
  await exec('cd frb_codegen && cargo run -- internal-generate');
  await _maybeSetExitIfChanged(config);
}

Future<void> generateInternalBookHelp(GenerateConfig config) async {
  for (final cmd in [
    null,
    'generate',
    'create',
    'integrate',
    'build-web',
  ]) {
    await exec('cd frb_codegen && cargo run -- $cmd --help > book/src/generated/${cmd ?? 'main'}.txt');
  }
  await _maybeSetExitIfChanged(config);
}

Future<void> generateInternalBuildRunner(GenerateConfig config) async {
  for (final package in kDartNonExamplePackages) {
    await exec('cd $package && dart run build_runner build --delete-conflicting-outputs');
  }
  await _maybeSetExitIfChanged(config);
}

Future<void> generateRunFrbCodegen(GenerateConfig config) async {
  for (final package in kDartExamplePackages) {
    await generateRunFrbCodegenCommandGenerate(config, package);
  }
}

/// Run flutter_rust_bridge_codegen's `generate` subcommand
Future<void> generateRunFrbCodegenCommandGenerate(GenerateConfig config, String package) async {
  await exec('cd $package && cargo run --manifest-path ${exec.pwd}/frb_codegen/Cargo.toml -- generate');
  await _maybeSetExitIfChanged(config);
}

Future<void> _maybeSetExitIfChanged(GenerateConfig config) async {
  if (config.setExitIfChanged) {
    await exec('git diff --exit-code');
  }
}
