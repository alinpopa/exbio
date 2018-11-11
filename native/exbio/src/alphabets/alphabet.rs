use bio::alphabets::Alphabet;
use bio::alphabets::{dna, protein, rna};
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

#[derive(NifUnitEnum)]
pub enum Alphabets {
    Dna,
    Rna,
    Protein,
}

#[derive(NifUnitEnum)]
pub enum AlphabetType {
    Default,
    Iupac,
    N,
}

pub struct AlphabetRef {
    pub alphabet: RwLock<Alphabet>,
}

pub fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let text: String = args[0].decode()?;
    let text = text.into_bytes();
    let alphabet = Alphabet::new(&text);
    let resource = ResourceArc::new(AlphabetRef {
        alphabet: RwLock::new(alphabet),
    });
    Ok((atoms::ok(), resource).encode(env))
}

pub fn insert<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let mut alphabet = resource.alphabet.write().unwrap();
    let a: u8 = args[1].decode()?;
    alphabet.insert(a);
    Ok((atoms::ok(), &resource).encode(env))
}

pub fn is_word<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let text: String = args[1].decode()?;
    let text = text.into_bytes();
    let is_word = alphabet.is_word(&text);
    Ok(is_word.encode(env))
}

pub fn max_symbol<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let result = alphabet.max_symbol();
    Ok((atoms::ok(), result).encode(env))
}

pub fn len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let len = alphabet.len();
    Ok(len.encode(env))
}

pub fn is_empty<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let is_empty = alphabet.is_empty();
    Ok(is_empty.encode(env))
}

pub fn alphabet<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabets: Alphabets = args[0].decode()?;
    let alphabet = match alphabets {
        Alphabets::Dna => dna_alphabet(args[1].decode()?),
        Alphabets::Rna => rna_alphabet(args[1].decode()?),
        Alphabets::Protein => protein::alphabet(),
    };
    let resource = ResourceArc::new(AlphabetRef {
        alphabet: RwLock::new(alphabet),
    });
    Ok((atoms::ok(), resource).encode(env))
}

pub fn complement<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabets: Alphabets = args[0].decode()?;
    let char: u8 = args[1].decode()?;
    match alphabets {
        Alphabets::Dna => Ok((atoms::ok(), dna::complement(char)).encode(env)),
        Alphabets::Rna => Ok((atoms::ok(), rna::complement(char)).encode(env)),
        Alphabets::Protein => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn revcomp<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let alphabets: Alphabets = args[0].decode()?;
    let text: String = args[1].decode()?;
    let text = text.into_bytes();
    match alphabets {
        Alphabets::Dna => Ok((
            atoms::ok(),
            String::from_utf8(dna::revcomp(&text)).expect(""),
        )
            .encode(env)),
        Alphabets::Rna => Ok((
            atoms::ok(),
            String::from_utf8(rna::revcomp(&text)).expect(""),
        )
            .encode(env)),
        Alphabets::Protein => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

fn dna_alphabet(alphabet_type: AlphabetType) -> Alphabet {
    match alphabet_type {
        AlphabetType::Default => dna::alphabet(),
        AlphabetType::Iupac => dna::iupac_alphabet(),
        AlphabetType::N => dna::n_alphabet(),
    }
}

fn rna_alphabet(alphabet_type: AlphabetType) -> Alphabet {
    match alphabet_type {
        AlphabetType::Default => rna::alphabet(),
        AlphabetType::Iupac => rna::iupac_alphabet(),
        AlphabetType::N => rna::n_alphabet(),
    }
}
