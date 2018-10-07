#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate bio;
extern crate bio_types;

use rustler::schedule::SchedulerFlags;
use rustler::{Env, Term};

mod alignment;
mod pattern_matching;
mod scores;
mod types;

rustler_export_nifs! {
    "Elixir.ExBio.Nif.RustBio",
    [
        ("pm_bom", 2, pattern_matching::bom::bom,
         SchedulerFlags::DirtyCpu),

        ("alignment_dist_hdist", 2, alignment::distance::hamming,
         SchedulerFlags::DirtyCpu),
        ("alignment_dist_ldist", 2, alignment::distance::levenshtein,
         SchedulerFlags::DirtyCpu),

        ("match_func_eq", 2, alignment::pairwise::matchfunc::eq),
        ("match_func_ne", 2, alignment::pairwise::matchfunc::ne),
        ("match_func_lt", 2, alignment::pairwise::matchfunc::lt),
        ("match_func_lte", 2,  alignment::pairwise::matchfunc::lte),
        ("match_func_gt", 2, alignment::pairwise::matchfunc::gt),
        ("match_func_gte", 2, alignment::pairwise::matchfunc::gte),
        ("match_func_apply", 3, alignment::pairwise::matchfunc::apply),

        ("alignment_pairwise_aligner_new", 3, alignment::pairwise::aligner::new),
        ("alignment_pairwise_aligner_with_capacity", 5, alignment::pairwise::aligner::with_capacity),
        ("alignment_pairwise_aligner_custom", 3, alignment::pairwise::aligner::custom,
         SchedulerFlags::DirtyCpu),
        ("alignment_pairwise_aligner_semiglobal", 3, alignment::pairwise::aligner::semiglobal,
         SchedulerFlags::DirtyCpu),

        ("alignment_pairwise_scoring_new", 3, alignment::pairwise::scoring::new),
        ("alignment_pairwise_scoring_from_scores", 4, alignment::pairwise::scoring::from_scores),
        ("alignment_pairwise_scoring_from_scoring", 1, alignment::pairwise::scoring::from_scoring),
        ("alignment_pairwise_scoring_xclip", 2, alignment::pairwise::scoring::xclip),
        ("alignment_pairwise_scoring_yclip", 2, alignment::pairwise::scoring::yclip),

        ("scores_blosum62", 0, scores::blosum62),
        ("scores_pam120", 0, scores::pam120),
        ("scores_pam200", 0, scores::pam200),
        ("scores_pam250", 0, scores::pam250),
        ("scores_pam40", 0, scores::pam40),
        ("scores_apply", 3, scores::apply),

        ("types_alignment_new", 1, types::alignment::new),
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(scores::FnScore, env);
    resource_struct_init!(alignment::pairwise::matchfunc::MatchFunc, env);
    resource_struct_init!(alignment::pairwise::aligner::Aligner, env);
    resource_struct_init!(types::alignment::AlignmentRef, env);
    true
}
