use bio::alphabets::Alphabet;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;

mod atoms {
    rustler_atoms! {
        atom ok;
    }
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
