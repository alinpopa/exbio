use bio::alphabets::Alphabet;
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
    Ok((atoms::ok(), is_word).encode(env))
}

pub fn len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let len = alphabet.len();
    Ok((atoms::ok(), len).encode(env))
}
