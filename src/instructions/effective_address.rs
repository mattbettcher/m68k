#![allow(non_camel_case_types)]
#![allow(dead_code)]

use Bus;
use M68k;
use std::num::Wrapping;
use super::super::Result;

pub fn absolute_word(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    //core.read_imm_i16().map(|res| res as u32)
    core.read_imm_data_16(bus).map(|res| res as u32)   // todo - should this get cache access?
}
pub fn absolute_long(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    core.read_imm_data_32(bus)
}
pub fn predecrement_ay_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_8(core, bus, reg_ndx))
}
pub fn postincrement_ay_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_8(core, bus, reg_ndx))
}
pub fn predecrement_ay_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_16(core, bus, reg_ndx))
}
pub fn postincrement_ay_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_16(core, bus, reg_ndx))
}
pub fn predecrement_ay_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(predecrement_32(core, bus, reg_ndx))
}
pub fn postincrement_ay_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ay!(core);
    Ok(postincrement_32(core, bus, reg_ndx))
}
pub fn address_indirect_ay(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(ay!(core))
}
pub fn address_indirect_ax(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(ax!(core))
}
pub fn displacement_ay(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_val = core.dar[ir_ay!(core)];
    displacement(core, bus, reg_val)
}
pub fn displacement_ax(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_val = core.dar[ir_ax!(core)];
    displacement(core, bus, reg_val)
}
pub fn displacement_pc(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let old_pc = core.pc;
    displacement(core, bus, old_pc)
}
pub fn index_ay(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_val = core.dar[ir_ay!(core)];
    index(core, bus, reg_val)
}
pub fn index_ax(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_val = core.dar[ir_ax!(core)];
    index(core, bus, reg_val)
}
pub fn index_pc(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let pc = core.pc;
    index(core, bus, pc)
}
pub fn predecrement_ax_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_8(core, bus, reg_ndx))
}
pub fn predecrement_ax_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_16(core, bus, reg_ndx))
}
pub fn predecrement_ax_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(predecrement_32(core, bus, reg_ndx))
}
pub fn postincrement_ax_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_8(core, bus, reg_ndx))
}
pub fn postincrement_ax_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_16(core, bus, reg_ndx))
}
pub fn postincrement_ax_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    let reg_ndx = ir_ax!(core);
    Ok(postincrement_32(core, bus, reg_ndx))
}

fn predecrement_8(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // pre-decrement
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) - match reg_ndx {
        15 => Wrapping(2), // A7 is kept even
         _ => Wrapping(1)
    }).0;
    core.dar[reg_ndx]
}
fn postincrement_8(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = core.dar[reg_ndx];
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) + match reg_ndx {
        15 => Wrapping(2), // A7 is kept even
         _ => Wrapping(1)
    }).0;
    ea
}
fn predecrement_16(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // pre-decrement
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) - Wrapping(2)).0;
    core.dar[reg_ndx]
}
fn postincrement_16(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = core.dar[reg_ndx];
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) + Wrapping(2)).0;
    ea
}
fn predecrement_32(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // pre-decrement
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) - Wrapping(4)).0;
    core.dar[reg_ndx]
}
fn postincrement_32(core: &mut M68k, _bus: &mut impl Bus, reg_ndx: usize) -> u32 {
    // post-increment
    let ea = core.dar[reg_ndx];
    core.dar[reg_ndx] = (Wrapping(core.dar[reg_ndx]) + Wrapping(4)).0;
    ea
}
pub fn displacement(core: &mut M68k, bus: &mut impl Bus, reg_val: u32) -> Result<u32> {
    let displacement = try!(core.read_imm_data_16(bus));
    let ea = (Wrapping(reg_val) + Wrapping(displacement as u32)).0;
    Ok(ea)
}
// Brief Extension Word format (see M68000 PRM section 2.1)
const LONG_INDEX_MASK: u16 = 0x0800;
fn index(core: &mut M68k, bus: &mut impl Bus, reg_val: u32) -> Result<u32> {
    let extension = try!(core.read_imm_data_16(bus));
    // top four bits = (D/A RRR) matches our register array layout
    let xreg_ndx = (extension>>12) as usize;
    let xn = core.dar[xreg_ndx];
    let xn = if (extension & LONG_INDEX_MASK) > 0 {xn} else {(xn as i16) as u32};

      let index = extension as i8;
    let ea = (Wrapping(reg_val) + Wrapping(xn) + Wrapping(index as u32)).0;
    Ok(ea)
}