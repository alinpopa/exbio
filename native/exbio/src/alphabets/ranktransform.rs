use bio::alphabets::RankTransform;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;
use alphabets::alphabet::AlphabetRef;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

pub struct RankTransformRef {
    pub rt: RwLock<RankTransform>,
}

pub fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlphabetRef> = args[0].decode()?;
    let alphabet = resource.alphabet.read().unwrap();
    let resource = ResourceArc::new(RankTransformRef {
        rt: RwLock::new(RankTransform::new(&alphabet)),
    });
    Ok((atoms::ok(), &resource).encode(env))
}
