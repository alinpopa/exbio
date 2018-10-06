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
    true
}
