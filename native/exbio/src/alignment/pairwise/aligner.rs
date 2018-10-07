use alignment::pairwise::matchfunc::MatchFunc;
use bio::alignment::pairwise::Aligner as BioAligner;
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

pub struct Aligner {
    pub aligner: BioAligner<MatchFunc>,
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
            let resource = ResourceArc::new(Aligner { aligner });
            Ok((atoms::ok(), resource.encode(env)).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}
