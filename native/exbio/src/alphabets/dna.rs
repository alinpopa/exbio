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

#[derive(NifUnitEnum)]
pub enum AlphabetType {
    Default,
    Iupac,
    N,
}

pub fn alphabet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabet_type: AlphabetType = args[0].decode()?;
    let alphabet = match alphabet_type {
        AlphabetType::Default => dna::alphabet(),
        AlphabetType::Iupac => dna::iupac_alphabet(),
        AlphabetType::N => dna::n_alphabet(),
    };
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

pub fn revcomp<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;
    let text = text.into_bytes();
    let complement = dna::revcomp(&text);
    let complement = String::from_utf8(complement).expect("");
    Ok(complement.encode(env))
}
