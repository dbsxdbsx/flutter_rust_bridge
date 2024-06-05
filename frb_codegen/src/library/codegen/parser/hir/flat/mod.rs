mod exporter;
pub(crate) mod parser;
pub(crate) mod transformer;

use crate::codegen::dumper::Dumper;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::hir::tree::pack::HirTreePack;
use crate::codegen::parser::hir::internal_config::ParserHirInternalConfig;
use crate::codegen::ConfigDumpContent::Hir;

pub(crate) fn parse(
    config: &ParserHirInternalConfig,
    hir_tree: HirTreePack,
    dumper: &Dumper,
) -> anyhow::Result<HirFlatPack> {
    let pack = parser::pack::parse_pack(config, hir_tree)?;
    dump(dumper, "1_parse_pack", &pack)?;

    let pack = transformer::function_interest_module_transformer::transform(pack, config)?;
    dump(dumper, "2_function_interest_module_transformer", &pack)?;

    let pack = transformer::remove_not_defined_trait_transformer::transform(pack)?;
    dump(dumper, "3_remove_not_defined_trait_transformer", &pack)?;

    let pack = transformer::copy_trait_def_to_impl_transformer::transform(pack)?;
    dump(dumper, "4_copy_trait_def_to_impl_transformer", &pack)?;

    let pack = transformer::merge_duplicate_transformer::transform(pack)?;
    dump(dumper, "5_merge_duplicate_transformer", &pack)?;

    let pack = transformer::resolve_type_alias_transformer::transform(pack)?;
    dump(dumper, "6_resolve_type_alias_transformer", &pack)?;

    Ok(pack)
}

fn dump(dumper: &Dumper, name: &str, pack: &HirFlatPack) -> anyhow::Result<()> {
    dumper.dump(Hir, &format!("hir_flat/{name}.json"), pack)
}
