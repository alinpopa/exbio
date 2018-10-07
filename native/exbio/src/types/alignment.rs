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
enum AlignmentOperation {
    Match,
    Subst,
    Del,
    Ins,
    Xclip,
    Yclip,
}

#[derive(NifUnitEnum)]
enum AlignmentMode {
    Local,
    Semiglobal,
    Global,
    Custom,
}

#[derive(NifStruct)]
#[module = "ExBio.Types.Alignment"]
struct Alignment {
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

fn to_bio_op(ops: Vec<(AlignmentOperation, usize)>) -> Vec<BioAlignmentOperation> {
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

fn to_bio_mode(mode: AlignmentMode) -> BioAlignmentMode {
    match mode {
        AlignmentMode::Local => BioAlignmentMode::Local,
        AlignmentMode::Semiglobal => BioAlignmentMode::Semiglobal,
        AlignmentMode::Global => BioAlignmentMode::Global,
        AlignmentMode::Custom => BioAlignmentMode::Custom,
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
            operations: to_bio_op(align.operations),
            mode: to_bio_mode(align.mode),
        },
    };
    let resource = ResourceArc::new(alignment);
    Ok((atoms::ok(), resource).encode(env))
}
