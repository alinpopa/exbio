use alphabets::alphabet;
use bio::alphabets::dna;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
}

pub fn alphabet<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabet = dna::alphabet();
    let resource = ResourceArc::new(alphabet::AlphabetRef {
        alphabet: RwLock::new(alphabet),
    });
    Ok((atoms::ok(), resource).encode(env))
}

pub fn complement<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let char: u8 = args[0].decode()?;
    let complement = dna::complement(char);
    Ok(complement.encode(env))
}
