use alphabets::alphabet;
use bio::alphabets::dna;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

pub fn alphabet<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabet = dna::alphabet();
    let resource = ResourceArc::new(alphabet::AlphabetRef {
        alphabet: RwLock::new(alphabet),
    });
    Ok((atoms::ok(), resource).encode(env))
}
