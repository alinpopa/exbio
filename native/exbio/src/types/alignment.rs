use bio_types::alignment::{
    Alignment as BioAlignment, AlignmentMode as BioAlignmentMode,
    AlignmentOperation as BioAlignmentOperation,
};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

#[derive(NifUnitEnum)]
pub enum AlignmentOperation {
    Match,
    Subst,
    Del,
    Ins,
    Xclip,
    Yclip,
}

#[derive(NifUnitEnum)]
pub enum AlignmentMode {
    Local,
    Semiglobal,
    Global,
    Custom,
}

#[derive(NifStruct)]
#[module = "ExBio.Types.Alignment"]
pub struct Alignment {
    pub score: i32,
    pub ystart: usize,
    pub xstart: usize,
    pub yend: usize,
    pub xend: usize,
    pub ylen: usize,
    pub xlen: usize,
    pub operations: Vec<(AlignmentOperation, usize)>,
    pub mode: AlignmentMode,
}

pub struct AlignmentRef {
    pub alignment: BioAlignment,
}

pub fn from_bio(bio: BioAlignment) -> Alignment {
    Alignment {
        score: bio.score,
        ystart: bio.ystart,
        xstart: bio.xstart,
        yend: bio.yend,
        xend: bio.xend,
        ylen: bio.ylen,
        xlen: bio.xlen,
        operations: from_bio_ops(bio.operations),
        mode: from_bio_mode(bio.mode),
    }
}

pub fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let align: Alignment = args[0].decode()?;
    let alignment = AlignmentRef {
        alignment: BioAlignment {
            score: align.score,
            ystart: align.ystart,
            xstart: align.xstart,
            yend: align.yend,
            xend: align.xend,
            ylen: align.ylen,
            xlen: align.xlen,
            operations: to_bio_ops(align.operations),
            mode: to_bio_mode(align.mode),
        },
    };
    let resource = ResourceArc::new(alignment);
    Ok((atoms::ok(), resource).encode(env))
}

fn to_bio_ops(ops: Vec<(AlignmentOperation, usize)>) -> Vec<BioAlignmentOperation> {
    ops.iter()
        .map(|op| match op {
            (AlignmentOperation::Match, _) => BioAlignmentOperation::Match,
            (AlignmentOperation::Subst, _) => BioAlignmentOperation::Subst,
            (AlignmentOperation::Del, _) => BioAlignmentOperation::Del,
            (AlignmentOperation::Ins, _) => BioAlignmentOperation::Ins,
            (AlignmentOperation::Xclip, val) => BioAlignmentOperation::Xclip(*val),
            (AlignmentOperation::Yclip, val) => BioAlignmentOperation::Yclip(*val),
        }).collect()
}

fn from_bio_ops(ops: Vec<BioAlignmentOperation>) -> Vec<(AlignmentOperation, usize)> {
    ops.iter()
        .map(|op| match op {
            BioAlignmentOperation::Match => (AlignmentOperation::Match, 0),
            BioAlignmentOperation::Subst => (AlignmentOperation::Subst, 0),
            BioAlignmentOperation::Del => (AlignmentOperation::Del, 0),
            BioAlignmentOperation::Ins => (AlignmentOperation::Ins, 0),
            BioAlignmentOperation::Xclip(val) => (AlignmentOperation::Xclip, *val),
            BioAlignmentOperation::Yclip(val) => (AlignmentOperation::Yclip, *val),
        }).collect()
}

fn to_bio_mode(mode: AlignmentMode) -> BioAlignmentMode {
    match mode {
        AlignmentMode::Local => BioAlignmentMode::Local,
        AlignmentMode::Semiglobal => BioAlignmentMode::Semiglobal,
        AlignmentMode::Global => BioAlignmentMode::Global,
        AlignmentMode::Custom => BioAlignmentMode::Custom,
    }
}

fn from_bio_mode(mode: BioAlignmentMode) -> AlignmentMode {
    match mode {
        BioAlignmentMode::Local => AlignmentMode::Local,
        BioAlignmentMode::Semiglobal => AlignmentMode::Semiglobal,
        BioAlignmentMode::Global => AlignmentMode::Global,
        BioAlignmentMode::Custom => AlignmentMode::Custom,
    }
}
