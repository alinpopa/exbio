use alignment::pairwise::matchfunc::MatchFunc;
use alignment::pairwise::scoring::ScoringRef;
use bio::alignment::pairwise::banded::Aligner as BioAligner;
use bio::alignment::sparse::{hash_kmers, HashMapFx};
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

#[derive(NifUnitEnum, Clone)]
pub enum Op {
    Custom,
    Global,
    Semiglobal,
    Local,
}

#[derive(NifUnitEnum, Clone)]
pub enum KmerHashType {
    Y,
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
    let k: usize = args[3].decode()?;
    let w: usize = args[4].decode()?;
    let result = panic::catch_unwind(|| BioAligner::new(gap_open, gap_extend, match_func, k, w));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource).encode(env))
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
    let k: usize = args[5].decode()?;
    let w: usize = args[6].decode()?;
    let result = panic::catch_unwind(|| {
        BioAligner::with_capacity(m, n, gap_open, gap_extend, match_func, k, w)
    });
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn with_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ScoringRef> = args[0].decode()?;
    let k: usize = args[1].decode()?;
    let w: usize = args[2].decode()?;
    let scoring = resource.scoring.clone();
    let result = panic::catch_unwind(|| BioAligner::with_scoring(scoring, k, w));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn with_capacity_and_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let m: usize = args[0].decode()?;
    let n: usize = args[1].decode()?;
    let resource: ResourceArc<ScoringRef> = args[2].decode()?;
    let k: usize = args[3].decode()?;
    let w: usize = args[4].decode()?;
    let scoring = resource.scoring.clone();
    let result = panic::catch_unwind(|| BioAligner::with_capacity_and_scoring(m, n, scoring, k, w));
    match result {
        Ok(aligner) => {
            let resource = ResourceArc::new(Aligner {
                aligner: RwLock::new(aligner),
            });
            Ok((atoms::ok(), resource).encode(env))
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
    let alignment = alignment::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}

pub fn apply_with_prehash<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let op: Op = args[1].decode()?;
    let x: String = args[2].decode()?;
    let y: String = args[3].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let (kmer_hash_type, k): (KmerHashType, usize) = args[4].decode()?;
    let y_kmer_hash = get_kmer_hash(y, k, kmer_hash_type);
    let result = match op {
        Op::Custom => {
            let alignment = aligner.custom_with_prehash(x, y, &y_kmer_hash);
            let alignment = alignment::from_bio(alignment);
            (atoms::ok(), alignment).encode(env)
        }
        Op::Semiglobal => {
            let alignment = aligner.semiglobal_with_prehash(x, y, &y_kmer_hash);
            let alignment = alignment::from_bio(alignment);
            (atoms::ok(), alignment).encode(env)
        }
        _ => (atoms::error(), atoms::invalid_args()).encode(env),
    };
    Ok(result)
}

pub fn custom_with_matches<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let x: String = args[1].decode()?;
    let y: String = args[2].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let matches: Vec<(u32, u32)> = args[3].decode()?;
    let matches = matches.as_slice();
    let alignment = aligner.custom_with_matches(x, y, matches);
    let alignment = alignment::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}

pub fn custom_with_expanded_matches<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<Aligner> = args[0].decode()?;
    let mut aligner = resource.aligner.write().unwrap();
    let x: String = args[1].decode()?;
    let y: String = args[2].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let matches: Vec<(u32, u32)> = args[3].decode()?;
    let allowed_mismatches: Option<usize> = args[4].decode()?;
    let use_lcskpp_union: bool = args[5].decode()?;
    let alignment =
        aligner.custom_with_expanded_matches(x, y, matches, allowed_mismatches, use_lcskpp_union);
    let alignment = alignment::from_bio(alignment);
    Ok((atoms::ok(), alignment).encode(env))
}

fn get_kmer_hash<'a>(
    text_slice: &'a [u8],
    k: usize,
    kmer_hash_type: KmerHashType,
) -> HashMapFx<&'a [u8], Vec<u32>> {
    match kmer_hash_type {
        KmerHashType::Y => hash_kmers(text_slice, k),
    }
}
