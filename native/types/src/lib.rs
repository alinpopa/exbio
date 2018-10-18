#[macro_use]
extern crate rustler;
#[macro_use]
extern crate lazy_static;
extern crate bio;
extern crate bio_types;
extern crate types_lib;

use bio::utils::TextSlice;
use bio_types::alignment::Alignment as BioAlignment;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;
use types_lib::{from_bio_op, to_bio_mode, to_bio_op, Alignment};

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        atom invalid_args;
    }
}

struct AlignmentRef {
    pub alignment: RwLock<BioAlignment>,
}

fn new<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
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

fn pretty<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let x: String = args[1].decode()?;
    let y: String = args[2].decode()?;
    let x: TextSlice = x.as_bytes();
    let y: TextSlice = y.as_bytes();
    let alignment = resource.alignment.read().unwrap();
    let pretty = alignment.pretty(x, y);
    Ok((atoms::ok(), pretty).encode(env))
}

fn cigar<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let hard_clip: bool = args[1].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let cigar = alignment.cigar(hard_clip);
    Ok((atoms::ok(), cigar).encode(env))
}

fn path<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let path = alignment.path();
    let path = path
        .iter()
        .map(|(x_i, y_i, op)| (x_i, y_i, from_bio_op(op)))
        .collect::<Vec<_>>();
    Ok((atoms::ok(), path).encode(env))
}

fn filter_clip_operations<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let mut alignment = resource.alignment.write().unwrap();
    alignment.filter_clip_operations();
    Ok(atoms::ok().encode(env))
}

fn y_aln_len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let y_aln_len = alignment.y_aln_len();
    Ok((atoms::ok(), y_aln_len).encode(env))
}

fn x_aln_len<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<AlignmentRef> = args[0].decode()?;
    let alignment = resource.alignment.read().unwrap();
    let x_aln_len = alignment.x_aln_len();
    Ok((atoms::ok(), x_aln_len).encode(env))
}

rustler_export_nifs! {
    "Elixir.ExBio.Nif.Types",
    [
        ("alignment_new", 1, new),
        ("alignment_pretty", 3, pretty),
        ("alignment_cigar", 2, cigar),
        ("alignment_path", 1, path),
        ("alignment_filter_clip_operations", 1, filter_clip_operations),
        ("alignment_y_aln_len", 1, y_aln_len),
        ("alignment_x_aln_len", 1, x_aln_len),
    ],
    Some(on_load)
}

fn on_load<'a>(env: Env<'a>, _load_info: Term<'a>) -> bool {
    resource_struct_init!(AlignmentRef, env);
    true
}
