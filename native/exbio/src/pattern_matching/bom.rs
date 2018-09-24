use rustler::{Env, Term, NifResult, Encoder};
use rustler::schedule::SchedulerFlags;
use bio::pattern_matching::bom::BOM;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

rustler_export_nifs! {
    "Elixir.ExBio.PatternMatching.Bom",
    [("bom", 2, bom, SchedulerFlags::DirtyCpu)],
    Some(on_load)
}

fn on_load<'a>(_env: Env<'a>, _load_info: Term<'a>) -> bool {
    true
}

fn bom<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = try!(args[0].decode());
    let pattern: String = try!(args[1].decode());

    let bom = BOM::new(pattern.as_bytes());
    let occ: Vec<usize> = bom.find_all(text.as_bytes()).collect();

    Ok((atoms::ok(), occ).encode(env))
}
