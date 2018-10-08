use alignment::pairwise::matchfunc::MatchFunc;
use bio::alignment::pairwise::{MatchFunc as BioMatchFunc, Scoring as BioScoring};
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
pub enum MatchScore {
    Some,
    None,
}

#[derive(NifStruct, Clone)]
#[module = "ExBio.Alignment.Pairwise.Scoring"]
pub struct Scoring {
    pub gap_open: i32,
    pub gap_extend: i32,
    pub match_scores: (MatchScore, (i32, i32)),
    pub xclip_prefix: i32,
    pub xclip_suffix: i32,
    pub yclip_prefix: i32,
    pub yclip_suffix: i32,
    pub match_func: ResourceArc<MatchFunc>,
}

pub struct ScoringRef {
    pub scoring: BioScoring<MatchFunc>,
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

    let result = panic::catch_unwind(|| {
        let scoring = BioScoring::new(gap_open, gap_extend, match_func);
        ResourceArc::new(ScoringRef { scoring })
    });
    match result {
        Ok(scoring) => Ok((atoms::ok(), scoring).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn from_scores<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let gap_open: i32 = args[0].decode()?;
    let gap_extend: i32 = args[1].decode()?;
    let match_score: i32 = args[2].decode()?;
    let mismatch_score: i32 = args[3].decode()?;
    let result = panic::catch_unwind(|| {
        let scoring = BioScoring::from_scores(gap_open, gap_extend, match_score, mismatch_score);
        let match_func = MatchFunc {
            left: match_score,
            fun: |a: u8, b: u8| a == b,
            right: mismatch_score,
        };
        let scoring = BioScoring {
            gap_open: scoring.gap_open,
            gap_extend: scoring.gap_extend,
            match_fn: match_func,
            match_scores: scoring.match_scores,
            xclip_prefix: scoring.xclip_prefix,
            xclip_suffix: scoring.xclip_suffix,
            yclip_prefix: scoring.yclip_prefix,
            yclip_suffix: scoring.yclip_suffix,
        };
        ResourceArc::new(ScoringRef { scoring })
    });
    match result {
        Ok(scoring) => Ok((atoms::ok(), scoring).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn from_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let scoring: Scoring = args[0].decode()?;
    let result = panic::catch_unwind(|| {
        let scoring = scoring.clone();
        let scoring = to_bio_scoring(scoring);
        ResourceArc::new(ScoringRef { scoring })
    });
    match result {
        Ok(scoring) => Ok((atoms::ok(), scoring).encode(env)),
        Err(_) => Ok((atoms::error(), atoms::invalid_args()).encode(env)),
    }
}

pub fn to_scoring<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ScoringRef> = args[0].decode()?;
    let scoring = resource.scoring.clone();
    let match_fn = scoring.match_fn;
    let resource = ResourceArc::new(match_fn);
    let scoring = from_bio_scoring(scoring, resource);
    Ok((atoms::ok(), scoring).encode(env))
}

pub fn xclip<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ScoringRef> = args[0].decode()?;
    let penalty: i32 = args[1].decode()?;
    let scoring = resource.scoring.clone();
    let scoring = scoring.xclip(penalty);
    let resource = ResourceArc::new(ScoringRef { scoring });
    Ok((atoms::ok(), resource).encode(env))
}

pub fn yclip<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<ScoringRef> = args[0].decode()?;
    let penalty: i32 = args[1].decode()?;
    let scoring = resource.scoring.clone();
    let scoring = scoring.yclip(penalty);
    let resource = ResourceArc::new(ScoringRef { scoring });
    Ok((atoms::ok(), resource).encode(env))
}

fn get_match_fn(scoring: Scoring) -> MatchFunc {
    MatchFunc {
        left: scoring.match_func.left,
        fun: scoring.match_func.fun,
        right: scoring.match_func.right,
    }
}

fn to_bio_scoring(scoring: Scoring) -> BioScoring<MatchFunc> {
    let match_scores = match scoring.match_scores {
        (MatchScore::Some, (x, y)) => Some((x, y)),
        (MatchScore::None, (_, _)) => None,
    };
    let match_fn = get_match_fn(scoring.clone());
    BioScoring {
        gap_open: scoring.gap_open,
        gap_extend: scoring.gap_extend,
        match_fn: match_fn,
        match_scores: match_scores,
        xclip_prefix: scoring.xclip_prefix,
        xclip_suffix: scoring.xclip_suffix,
        yclip_prefix: scoring.yclip_prefix,
        yclip_suffix: scoring.yclip_suffix,
    }
}

fn from_bio_scoring<F: BioMatchFunc>(
    scoring: BioScoring<F>,
    resource: ResourceArc<MatchFunc>,
) -> Scoring {
    let match_scores = match scoring.match_scores {
        Some((x, y)) => (MatchScore::Some, (x, y)),
        None => (MatchScore::None, (0, 0)),
    };
    Scoring {
        gap_open: scoring.gap_open,
        gap_extend: scoring.gap_extend,
        match_scores: match_scores,
        xclip_prefix: scoring.xclip_prefix,
        xclip_suffix: scoring.xclip_suffix,
        yclip_prefix: scoring.yclip_prefix,
        yclip_suffix: scoring.yclip_suffix,
        match_func: resource,
    }
}
