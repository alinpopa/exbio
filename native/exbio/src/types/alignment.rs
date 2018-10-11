use bio::utils::TextSlice;
use bio_types::alignment::{
    Alignment as BioAlignment, AlignmentMode as BioAlignmentMode,
    AlignmentOperation as BioAlignmentOperation,
};
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;

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
    pub alignment: RwLock<BioAlignment>,
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

pub fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let align: Alignment = args[0].decode()?;
    let alignment = AlignmentRef {
        alignment: RwLock::new(BioAlignment {
            score: align.score,
            ystart: align.ystart,
            xstart: align.xstart,
            yend: align.yend,
            xend: align.xend,
            ylen: align.ylen,
            xlen: align.xlen,
            operations: align.operations.iter().map(|op| to_bio_op(op)).collect(),
            mode: to_bio_mode(align.mode),
        }),
    };
    let resource = ResourceArc::new(alignment);
    Ok((atoms::ok(), resource).encode(env))
}

pub fn pretty<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let x: String = args[1].decode()?;
    let y: String = args[2].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let alignment = resource.alignment.read().unwrap();
    let pretty = alignment.pretty(x, y);
    Ok((atoms::ok(), pretty).encode(env))
}

pub fn cigar<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let hard_clip: bool = args[1].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let cigar = alignment.cigar(hard_clip);
    Ok((atoms::ok(), cigar).encode(env))
}

pub fn path<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let path = alignment.path();
    let path = path
        .iter()
        .map(|(x_i, y_i, op)| (x_i, y_i, from_bio_op(op)))
        .collect::<Vec<_>>();
    Ok((atoms::ok(), path).encode(env))
}

pub fn filter_clip_operations<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let mut alignment = resource.alignment.write().unwrap();
    alignment.filter_clip_operations();
    Ok(atoms::ok().encode(env))
}

pub fn y_aln_len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let y_aln_len = alignment.y_aln_len();
    Ok((atoms::ok(), y_aln_len).encode(env))
}

pub fn x_aln_len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let x_aln_len = alignment.x_aln_len();
    Ok((atoms::ok(), x_aln_len).encode(env))
}

fn to_bio_op(op: &(AlignmentOperation, usize)) -> BioAlignmentOperation {
    match op {
        (AlignmentOperation::Match, _) => BioAlignmentOperation::Match,
        (AlignmentOperation::Subst, _) => BioAlignmentOperation::Subst,
        (AlignmentOperation::Del, _) => BioAlignmentOperation::Del,
        (AlignmentOperation::Ins, _) => BioAlignmentOperation::Ins,
        (AlignmentOperation::Xclip, val) => BioAlignmentOperation::Xclip(*val),
        (AlignmentOperation::Yclip, val) => BioAlignmentOperation::Yclip(*val),
    }
}

fn from_bio_op(op: &BioAlignmentOperation) -> (AlignmentOperation, usize) {
    match op {
        BioAlignmentOperation::Match => (AlignmentOperation::Match, 0),
        BioAlignmentOperation::Subst => (AlignmentOperation::Subst, 0),
        BioAlignmentOperation::Del => (AlignmentOperation::Del, 0),
        BioAlignmentOperation::Ins => (AlignmentOperation::Ins, 0),
        BioAlignmentOperation::Xclip(val) => (AlignmentOperation::Xclip, *val),
        BioAlignmentOperation::Yclip(val) => (AlignmentOperation::Yclip, *val),
    }
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
