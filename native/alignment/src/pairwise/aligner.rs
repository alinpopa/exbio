use bio::alignment::pairwise::Aligner as BioAligner;
use bio::utils::TextSlice;
use pairwise::matchfunc::MatchFunc;
use pairwise::scoring::ScoringRef;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::panic;
use std::sync::RwLock;
use types_lib;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

#[derive(NifUnitEnum, Clone)]
pub enum Op {
    Custom,
    Global,
    Semiglobal,
    Local,
}

pub struct Aligner {
    aligner: RwLock<BioAligner<MatchFunc>>,
}

pub fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let gap_open: i32 = args[0].decode()?;
    let gap_extend: i32 = args[1].decode()?;
    let resource: ResourceArc<MatchFunc> = args[2].decode()?;
    let match_func = MatchFunc {
        left: resource.left,
        fun: resource.fun,
        right: resource.right,
    };
    let result = panic::catch_unwind(|| BioAligner::new(gap_open, gap_extend, match_func));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource.encode(env)).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn with_capacity<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let m: usize = args[0].decode()?;
    let n: usize = args[1].decode()?;
    let gap_open: i32 = args[2].decode()?;
    let gap_extend: i32 = args[3].decode()?;
    let resource: ResourceArc<MatchFunc> = args[4].decode()?;
    let match_func = MatchFunc {
        left: resource.left,
        fun: resource.fun,
        right: resource.right,
    };
    let result =
        panic::catch_unwind(|| BioAligner::with_capacity(m, n, gap_open, gap_extend, match_func));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource.encode(env)).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn with_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ScoringRef> = args[0].decode()?;
    let scoring = resource.scoring.clone();
    let result = panic::catch_unwind(|| BioAligner::with_scoring(scoring));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource.encode(env)).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn with_capacity_and_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let m: usize = args[0].decode()?;
    let n: usize = args[1].decode()?;
    let resource: ResourceArc<ScoringRef> = args[2].decode()?;
    let scoring = resource.scoring.clone();
    let result = panic::catch_unwind(|| BioAligner::with_capacity_and_scoring(m, n, scoring));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource.encode(env)).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn apply<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let op: Op = args[1].decode()?;
    let x: String = args[2].decode()?;
    let y: String = args[3].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let alignment = match op {
        Op::Custom => aligner.custom(x, y),
        Op::Global => aligner.global(x, y),
        Op::Semiglobal => aligner.semiglobal(x, y),
        Op::Local => aligner.local(x, y),
    };
    let alignment = types_lib::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}
