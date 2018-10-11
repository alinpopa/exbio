use bio::alignment::pairwise::TracebackCell as BioTracebackCell;
use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};
use std::sync::RwLock;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
    }
}

#[derive(NifUnitEnum, Clone)]
pub enum Op {
    I,
    D,
    S,
    All,
}

pub struct TracebackCell {
    tracebackcell: RwLock<BioTracebackCell>,
}

pub fn new<'a>(env: Env<'a>, _args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let tracebackcell = BioTracebackCell::new();
    let resource = ResourceArc::new(TracebackCell {
        tracebackcell: RwLock::new(tracebackcell),
    });
    Ok((atoms::ok(), resource).encode(env))
}

pub fn set_bits<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TracebackCell> = args[0].decode()?;
    let op: Op = args[1].decode()?;
    let value: u16 = args[2].decode()?;
    let mut tracebackcell = resource.tracebackcell.write().unwrap();
    match op {
        Op::I => tracebackcell.set_i_bits(value),
        Op::D => tracebackcell.set_d_bits(value),
        Op::S => tracebackcell.set_s_bits(value),
        Op::All => tracebackcell.set_all(value),
    }
    Ok(atoms::ok().encode(env))
}

pub fn get_bits<'a>(env: Env<'a>, args: &[Term<'a>]) -> NifResult<Term<'a>> {
    let resource: ResourceArc<TracebackCell> = args[0].decode()?;
    let op: Op = args[1].decode()?;
    let tracebackcell = resource.tracebackcell.read().unwrap();
    let value = match op {
        Op::I => (atoms::ok(), vec![(Op::I, tracebackcell.get_i_bits())]),
        Op::D => (atoms::ok(), vec![(Op::D, tracebackcell.get_d_bits())]),
        Op::S => (atoms::ok(), vec![(Op::S, tracebackcell.get_s_bits())]),
        Op::All => (
            atoms::ok(),
            vec![
                (Op::I, tracebackcell.get_i_bits()),
                (Op::D, tracebackcell.get_d_bits()),
                (Op::S, tracebackcell.get_s_bits()),
            ],
        ),
    };
    Ok(value.encode(env))
}
