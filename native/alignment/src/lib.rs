#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate bio;
extern crate bio_types;
extern crate types_lib;

use rustler::schedule::SchedulerFlags;
use rustler::{Env, Term};

mod distance;
mod pairwise;

rustler_export_nifs! {
    "Elixir.ExBio.Nif.Alignment",
    [
        ("dist_hdist", 2, distance::hamming,
         SchedulerFlags::DirtyCpu),
        ("dist_ldist", 2, distance::levenshtein,
         SchedulerFlags::DirtyCpu),

        ("match_func", 3, pairwise::matchfunc::matchfunc),
        ("match_func_apply", 3, pairwise::matchfunc::apply),

        ("pairwise_aligner_new", 3, pairwise::aligner::new),
        ("pairwise_aligner_with_capacity", 5, pairwise::aligner::with_capacity),
        ("pairwise_aligner_with_scoring", 1, pairwise::aligner::with_scoring),
        ("pairwise_aligner_with_capacity_and_scoring", 3, pairwise::aligner::with_capacity_and_scoring),
        ("pairwise_aligner_apply", 4, pairwise::aligner::apply,
         SchedulerFlags::DirtyCpu),

        ("pairwise_banded_aligner_new", 5, pairwise::banded::aligner::new),
        ("pairwise_banded_aligner_with_capacity", 7, pairwise::banded::aligner::with_capacity),
        ("pairwise_banded_aligner_with_scoring", 3, pairwise::banded::aligner::with_scoring),
        ("pairwise_banded_aligner_with_capacity_and_scoring", 5, pairwise::banded::aligner::with_capacity_and_scoring),
        ("pairwise_banded_aligner_apply_with_prehash", 5, pairwise::banded::aligner::apply_with_prehash,
         SchedulerFlags::DirtyCpu),
        // TODO: "pairwise_banded_aligner_custom_with_matches", 4, pairwise::banded::aligner::custom_with_matches),
        // TODO: "pairwise_banded_aligner_custom_with_expanded_matches", 4, pairwise::banded::aligner::custom_with_expanded_matches),
        // TODO: "pairwise_banded_aligner_semiglobal_with_prehash", 4, pairwise::banded::aligner::semiglobal_with_prehash),
        ("pairwise_banded_aligner_apply", 4, pairwise::banded::aligner::apply,
         SchedulerFlags::DirtyCpu),

        ("pairwise_scoring_new", 3, pairwise::scoring::new),
        ("pairwise_scoring_from_scores", 4, pairwise::scoring::from_scores),
        ("pairwise_scoring_from_scoring", 1, pairwise::scoring::from_scoring),
        ("pairwise_scoring_to_scoring", 1, pairwise::scoring::to_scoring),
        ("pairwise_scoring_xclip", 2, pairwise::scoring::xclip),
        ("pairwise_scoring_yclip", 2, pairwise::scoring::yclip),

        ("pairwise_tracebackcell_new", 0, pairwise::tracebackcell::new),
        ("pairwise_tracebackcell_set_bits", 3, pairwise::tracebackcell::set_bits),
        ("pairwise_tracebackcell_get_bits", 2, pairwise::tracebackcell::get_bits),
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(pairwise::matchfunc::MatchFunc, env);
    resource_struct_init!(pairwise::aligner::Aligner, env);
    resource_struct_init!(pairwise::banded::aligner::Aligner, env);
    resource_struct_init!(pairwise::scoring::ScoringRef, env);
    resource_struct_init!(pairwise::tracebackcell::TracebackCell, env);
    true
}
