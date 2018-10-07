use bio::alignment::pairwise::MatchFunc as BioMatchFunc;
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

pub struct MatchFunc {
    pub left: i32,
    pub fun: fn(u8, u8) -> bool,
    pub right: i32,
}

impl BioMatchFunc for MatchFunc {
    fn score(&self, a: u8, b: u8) -> i32 {
        let f: fn(u8, u8) -> bool = self.fun;
        if f(a, b) {
            self.left
        } else {
            self.right
        }
    }
}

fn run<'a>(env: Env<'a>, args: &[Term<'a>], func: fn(u8, u8) -> bool) -> NifResult<Term<'a>> {
    let left: i32 = args[0].decode()?;
    let right: i32 = args[1].decode()?;
    let match_func = MatchFunc {
        left: left,
        fun: func,
        right: right,
    };
    let resource = ResourceArc::new(match_func);
    Ok(resource.encode(env))
}

pub fn eq<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a == b)
}

pub fn ne<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a != b)
}

pub fn lt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a < b)
}

pub fn lte<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a <= b)
}

pub fn gt<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a > b)
}

pub fn gte<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    run(env, args, |a: u8, b: u8| a >= b)
}

pub fn apply<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<MatchFunc> = args[0].decode()?;
    let a: u8 = args[1].decode()?;
    let b: u8 = args[2].decode()?;
    let f: fn(u8, u8) -> bool = resource.fun;
    let result = panic::catch_unwind(|| {
        if f(a, b) {
            resource.left
        } else {
            resource.right
        }
    });
    match result {
        Ok(r) => Ok((atoms::ok(), r).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}
