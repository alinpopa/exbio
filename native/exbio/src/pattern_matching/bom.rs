use bio::pattern_matching::bom::BOM;
use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

pub fn bom<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;
    let pattern: String = args[1].decode()?;

    let bom = BOM::new(pattern.as_bytes());
    let occ: Vec<usize> = bom.find_all(text.as_bytes()).collect();

    Ok((atoms::ok(), occ).encode(env))
}
