#[macro_use] extern crate rustler;
#[macro_use] extern crate lazy_static;
extern crate bio;

use rustler::{Env, Term};
use rustler::schedule::SchedulerFlags;

mod pattern_matching;
mod alignment;

rustler_export_nifs! {
    "Elixir.ExBio.Nif.RustBio",
    [
        ("pm_bom", 2, pattern_matching::bom::bom, SchedulerFlags::DirtyCpu),
        ("alignment_dist_hdist", 2, alignment::distance::hamming, SchedulerFlags::DirtyCpu),
        ("alignment_dist_ldist", 2, alignment::distance::levenshtein, SchedulerFlags::DirtyCpu),
    ],
    Some(on_load)
}

fn on_load<'a>(_env: Env<'a>, _load_info: Term<'a>) -> bool {
    true
}
