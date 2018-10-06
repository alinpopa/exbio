#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate bio;

use rustler::schedule::SchedulerFlags;
use rustler::{Env, Term};

mod alignment;
mod pattern_matching;
mod scores;

rustler_export_nifs! {
    "Elixir.ExBio.Nif.RustBio",
    [
        ("pm_bom", 2, pattern_matching::bom::bom, SchedulerFlags::DirtyCpu),

        ("alignment_dist_hdist", 2, alignment::distance::hamming, SchedulerFlags::DirtyCpu),
        ("alignment_dist_ldist", 2, alignment::distance::levenshtein, SchedulerFlags::DirtyCpu),

        ("match_func_eq", 0, alignment::pairwise::matchfunc::eq),
        ("match_func_ne", 0, alignment::pairwise::matchfunc::ne),
        ("match_func_lt", 0, alignment::pairwise::matchfunc::lt),
        ("match_func_lte", 0,  alignment::pairwise::matchfunc::lte),
        ("match_func_gt", 0, alignment::pairwise::matchfunc::gt),
        ("match_func_gte", 0, alignment::pairwise::matchfunc::gte),
        ("match_func_apply", 4, alignment::pairwise::matchfunc::apply),

        ("scores_blosum62", 0, scores::blosum62),
        ("scores_pam120", 0, scores::pam120),
        ("scores_pam200", 0, scores::pam200),
        ("scores_pam250", 0, scores::pam250),
        ("scores_pam40", 0, scores::pam40),
        ("scores_apply", 3, scores::apply),
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(scores::FnScore, env);
    resource_struct_init!(alignment::pairwise::matchfunc::MatchFunc, env);
    true
}
