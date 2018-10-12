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

#[derive(NifUnitEnum, Clone)]
pub enum FunType {
    Blosum62,
    Pam120,
    Pam200,
    Pam250,
    Pam40,
}

pub struct FnScore {
    fun: fn(u8, u8) -> i32,
}

pub fn fun<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let fun_type: FunType = args[0].decode()?;
    let score_fn = match fun_type {
        FunType::Blosum62 => FnScore {
            fun: bio::scores::blosum62,
        },
        FunType::Pam120 => FnScore {
            fun: bio::scores::pam120,
        },
        FunType::Pam200 => FnScore {
            fun: bio::scores::pam200,
        },
        FunType::Pam250 => FnScore {
            fun: bio::scores::pam250,
        },
        FunType::Pam40 => FnScore {
            fun: bio::scores::pam40,
        },
    };
    let resource = ResourceArc::new(score_fn);
    Ok((atoms::ok(), resource).encode(env))
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
