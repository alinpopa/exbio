use alignment::pairwise::matchfunc::MatchFunc;
use bio::alignment::pairwise::Aligner as BioAligner;
use bio::utils::TextSlice;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::panic;
use std::sync::RwLock;
use types::alignment;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
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

pub fn custom<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let x: String = try!(args[1].decode());
    let y: String = try!(args[2].decode());
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let alignment = aligner.custom(x, y);
    let alignment = alignment::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}

pub fn semiglobal<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let x: String = try!(args[1].decode());
    let y: String = try!(args[2].decode());
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let alignment = aligner.semiglobal(x, y);
    let alignment = alignment::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}
