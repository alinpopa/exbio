use rustler::{Encoder, Env, NifResult, Term};
use rustler::resource::ResourceArc;
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
        let (left, right) = left_right;
        if a == b {left} else {right}
    }

    pub fn ne(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        let (left, right) = left_right;
        if a != b {left} else {right}
    }

    pub fn lt(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        let (left, right) = left_right;
        if a < b {left} else {right}
    }

    pub fn lte(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        let (left, right) = left_right;
        if a <= b {left} else {right}
    }

    pub fn gt(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        let (left, right) = left_right;
        if a > b {left} else {right}
    }

    pub fn gte(a: u8, b: u8, left_right: (i32, i32)) -> i32 {
        let (left, right) = left_right;
        if a >= b {left} else {right}
    }
}

pub struct MatchFunc {
    fun: fn(u8, u8, (i32, i32)) -> i32,
}

pub fn eq<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::eq,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn ne<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::ne,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn lt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::lt,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn lte<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::lte,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn gt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::gt,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
}

pub fn gte<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let func = MatchFunc {
        fun: funcs::gte,
    };
    let resource = ResourceArc::new(func);
    Ok(resource.encode(env))
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
