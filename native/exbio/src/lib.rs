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

        ("match_func", 3, alignment::pairwise::matchfunc::matchfunc),
        ("match_func_apply", 3, alignment::pairwise::matchfunc::apply),

        ("alignment_pairwise_aligner_new", 3, alignment::pairwise::aligner::new),
        ("alignment_pairwise_aligner_with_capacity", 5, alignment::pairwise::aligner::with_capacity),
        ("alignment_pairwise_aligner_with_scoring", 1, alignment::pairwise::aligner::with_scoring),
        ("alignment_pairwise_aligner_with_capacity_and_scoring", 3, alignment::pairwise::aligner::with_capacity_and_scoring),
        ("alignment_pairwise_aligner_apply", 4, alignment::pairwise::aligner::apply,
         SchedulerFlags::DirtyCpu),

        ("alignment_pairwise_banded_aligner_new", 5, alignment::pairwise::banded::aligner::new),
        ("alignment_pairwise_banded_aligner_with_capacity", 7, alignment::pairwise::banded::aligner::with_capacity),
        ("alignment_pairwise_banded_aligner_with_scoring", 3, alignment::pairwise::banded::aligner::with_scoring),
        ("alignment_pairwise_banded_aligner_with_capacity_and_scoring", 5, alignment::pairwise::banded::aligner::with_capacity_and_scoring),
        ("alignment_pairwise_banded_aligner_apply_with_prehash", 5, alignment::pairwise::banded::aligner::apply_with_prehash,
         SchedulerFlags::DirtyCpu),
        // TODO: "alignment_pairwise_banded_aligner_custom_with_matches", 4, alignment::pairwise::banded::aligner::custom_with_matches),
        // TODO: "alignment_pairwise_banded_aligner_custom_with_expanded_matches", 4, alignment::pairwise::banded::aligner::custom_with_expanded_matches),
        // TODO: "alignment_pairwise_banded_aligner_semiglobal_with_prehash", 4, alignment::pairwise::banded::aligner::semiglobal_with_prehash),
        ("alignment_pairwise_banded_aligner_apply", 4, alignment::pairwise::banded::aligner::apply,
         SchedulerFlags::DirtyCpu),

        ("alignment_pairwise_scoring_new", 3, alignment::pairwise::scoring::new),
        ("alignment_pairwise_scoring_from_scores", 4, alignment::pairwise::scoring::from_scores),
        ("alignment_pairwise_scoring_from_scoring", 1, alignment::pairwise::scoring::from_scoring),
        ("alignment_pairwise_scoring_to_scoring", 1, alignment::pairwise::scoring::to_scoring),
        ("alignment_pairwise_scoring_xclip", 2, alignment::pairwise::scoring::xclip),
        ("alignment_pairwise_scoring_yclip", 2, alignment::pairwise::scoring::yclip),

        ("alignment_pairwise_tracebackcell_new", 0, alignment::pairwise::tracebackcell::new),
        ("alignment_pairwise_tracebackcell_set_bits", 3, alignment::pairwise::tracebackcell::set_bits),
        ("alignment_pairwise_tracebackcell_get_bits", 2, alignment::pairwise::tracebackcell::get_bits),

        ("scores_fun", 1, scores::fun),
        ("scores_apply", 3, scores::apply),

        ("types_alignment_new", 1, types::alignment::new),
        ("types_alignment_pretty", 3, types::alignment::pretty),
        ("types_alignment_cigar", 2, types::alignment::cigar),
        ("types_alignment_path", 1, types::alignment::path),
        ("types_alignment_filter_clip_operations", 1, types::alignment::filter_clip_operations),
        ("types_alignment_y_aln_len", 1, types::alignment::y_aln_len),
        ("types_alignment_x_aln_len", 1, types::alignment::x_aln_len),
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(scores::FnScore, env);
    resource_struct_init!(alignment::pairwise::matchfunc::MatchFunc, env);
    resource_struct_init!(alignment::pairwise::aligner::Aligner, env);
    resource_struct_init!(alignment::pairwise::banded::aligner::Aligner, env);
    resource_struct_init!(alignment::pairwise::scoring::ScoringRef, env);
    resource_struct_init!(alignment::pairwise::tracebackcell::TracebackCell, env);
    resource_struct_init!(types::alignment::AlignmentRef, env);
    true
}
