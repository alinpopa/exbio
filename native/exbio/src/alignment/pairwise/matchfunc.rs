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

mod funcs {
    pub fn eq(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a == b {
            left_right.0
        } else {
            left_right.1
        }
    }

    pub fn ne(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a != b {
            left_right.0
        } else {
            left_right.1
        }
    }

    pub fn lt(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a < b {
            left_right.0
        } else {
            left_right.1
        }
    }

    pub fn lte(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a <= b {
            left_right.0
        } else {
            left_right.1
        }
    }

    pub fn gt(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a > b {
            left_right.0
        } else {
            left_right.1
        }
    }

    pub fn gte(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        if a >= b {
            left_right.0
        } else {
            left_right.1
        }
    }
}

pub struct MatchFunc {
    fun: fn(u8, u8, (i32, i32)) -> i32,
}

fn run<'a>(env: Env<'a>, func: MatchFunc) -> NifResult<Term<'a>> {
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn eq<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::eq })
}

pub fn ne<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::ne })
}

pub fn lt<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::lt })
}

pub fn lte<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::lte })
}

pub fn gt<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::gt })
}

pub fn gte<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, MatchFunc { fun: funcs::gte })
}

pub fn apply<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<MatchFunc> = args[0].decode()?;
    let a: u8 = args[1].decode()?;
    let b: u8 = args[2].decode()?;
    let c: (i32, i32) = args[3].decode()?;

    let f: fn(u8, u8, (i32, i32)) -> i32 = resource.fun;
    let result = panic::catch_unwind(|| f(a, b, c));
    match result {
        Ok(r) => Ok((atoms::ok(), r).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}
