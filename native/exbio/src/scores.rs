use bio;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::panic;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

pub struct FnScore {
    fun: fn(u8, u8) -> i32,
}

pub fn blosum62<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let score_fn = FnScore {
        fun: bio::scores::blosum62,
    };
    let resource = ResourceArc::new(score_fn);
    Ok(resource.encode(env))
}

pub fn pam120<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let score_fn = FnScore {
        fun: bio::scores::pam120,
    };
    let resource = ResourceArc::new(score_fn);
    Ok(resource.encode(env))
}

pub fn pam200<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let score_fn = FnScore {
        fun: bio::scores::pam200,
    };
    let resource = ResourceArc::new(score_fn);
    Ok(resource.encode(env))
}

pub fn pam250<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let score_fn = FnScore {
        fun: bio::scores::pam250,
    };
    let resource = ResourceArc::new(score_fn);
    Ok(resource.encode(env))
}

pub fn pam40<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let score_fn = FnScore {
        fun: bio::scores::pam40,
    };
    let resource = ResourceArc::new(score_fn);
    Ok(resource.encode(env))
}

pub fn apply<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<FnScore> = args[0].decode()?;
    let a: u8 = args[1].decode()?;
    let b: u8 = args[2].decode()?;
    let f: fn(u8, u8) -> i32 = resource.fun;
    let result = panic::catch_unwind(|| f(a, b));
    match result {
        Ok(r) => Ok((atoms::ok(), r).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}
