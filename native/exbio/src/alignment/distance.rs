use rustler::{Env, Term, NifResult, Encoder};
use bio::alignment::distance::{hamming as hdist, levenshtein as ldist};

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

pub fn hamming<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let x: String = try!(args[0].decode());
    let y: String = try!(args[1].decode());

    let z: u64 = hdist(x.as_bytes(), y.as_bytes());
    Ok((atoms::ok(), z).encode(env))
}
pub fn levenshtein<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let x: String = try!(args[0].decode());
    let y: String = try!(args[1].decode());

    let z: u32 = ldist(x.as_bytes(), y.as_bytes());
    Ok((atoms::ok(), z).encode(env))
}
