#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;
#[macro_use]
extern crate lazy_static;
extern crate bio_types;

use bio_types::alignment::{
    Alignment as BioAlignment, AlignmentMode as BioAlignmentMode,
    AlignmentOperation as BioAlignmentOperation,
};

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

pub fn from_bio(bio: BioAlignment) -> Alignment {
    Alignment {
        score: bio.score,
        ystart: bio.ystart,
        xstart: bio.xstart,
        yend: bio.yend,
        xend: bio.xend,
        ylen: bio.ylen,
        xlen: bio.xlen,
        operations: bio.operations.iter().map(|op| from_bio_op(op)).collect(),
        mode: from_bio_mode(bio.mode),
    }
}

pub fn from_bio_op(op: &BioAlignmentOperation) -> (AlignmentOperation, usize) {
    match op {
        BioAlignmentOperation::Match => (AlignmentOperation::Match, 0),
        BioAlignmentOperation::Subst => (AlignmentOperation::Subst, 0),
        BioAlignmentOperation::Del => (AlignmentOperation::Del, 0),
        BioAlignmentOperation::Ins => (AlignmentOperation::Ins, 0),
        BioAlignmentOperation::Xclip(val) => (AlignmentOperation::Xclip, *val),
        BioAlignmentOperation::Yclip(val) => (AlignmentOperation::Yclip, *val),
    }
}

pub fn from_bio_mode(mode: BioAlignmentMode) -> AlignmentMode {
    match mode {
        BioAlignmentMode::Local => AlignmentMode::Local,
        BioAlignmentMode::Semiglobal => AlignmentMode::Semiglobal,
        BioAlignmentMode::Global => AlignmentMode::Global,
        BioAlignmentMode::Custom => AlignmentMode::Custom,
    }
}

pub fn to_bio_mode(mode: AlignmentMode) -> BioAlignmentMode {
    match mode {
        AlignmentMode::Local => BioAlignmentMode::Local,
        AlignmentMode::Semiglobal => BioAlignmentMode::Semiglobal,
        AlignmentMode::Global => BioAlignmentMode::Global,
        AlignmentMode::Custom => BioAlignmentMode::Custom,
    }
}

pub fn to_bio_op(op: &(AlignmentOperation, usize)) -> BioAlignmentOperation {
    match op {
        (AlignmentOperation::Match, _) => BioAlignmentOperation::Match,
        (AlignmentOperation::Subst, _) => BioAlignmentOperation::Subst,
        (AlignmentOperation::Del, _) => BioAlignmentOperation::Del,
        (AlignmentOperation::Ins, _) => BioAlignmentOperation::Ins,
        (AlignmentOperation::Xclip, val) => BioAlignmentOperation::Xclip(*val),
        (AlignmentOperation::Yclip, val) => BioAlignmentOperation::Yclip(*val),
    }
}
