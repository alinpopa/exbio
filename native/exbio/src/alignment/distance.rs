use bio::alignment::distance::{hamming as hdist, levenshtein as ldist};
use bio::utils::TextSlice;
use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

pub fn hamming<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let x: String = try!(args[0].decode());
    let y: String = try!(args[1].decode());

    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();

    if x.len() != y.len() {
        return Ok((atoms::error(), atoms::invalid_args()).encode(env));
    }
    Ok((atoms::ok(), hdist(x, y)).encode(env))
}

pub fn levenshtein<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let x: String = try!(args[0].decode());
    let y: String = try!(args[1].decode());

    let z: u32 = ldist(x.as_bytes(), y.as_bytes());
    Ok((atoms::ok(), z).encode(env))
}
