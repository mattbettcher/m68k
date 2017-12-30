#![allow(non_camel_case_types)]
#![allow(dead_code)]

use Bus;
use M68k;
use super::*;
use super::super::Result;

pub fn ea_ay_pd_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ax_pd_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_pi_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_ai_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_di_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_ix_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_aw_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_word(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_al_8(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea).map(|val| (val as u32, ea)))
}

pub fn ay_pd_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::predecrement_ay_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn ay_pi_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ay_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn ax_pi_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ax_8(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn ay_ai_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn ay_di_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn ay_ix_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn aw_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_word(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn al_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_8(bus, ea)).map(|val| val as u32)
}
pub fn pcdi_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_pc(core, bus)
    .and_then(|ea| core.read_prog_8(bus, ea)).map(|val| val as u32)
}
pub fn pcix_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_pc(core, bus)
    .and_then(|ea| core.read_prog_8(bus, ea)).map(|val| val as u32)
}
pub fn imm_8(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    core.read_imm_data_16(bus)
    .map(|extension| mask_out_above_8!(extension) as u32)
}

pub fn ea_ay_pd_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ax_pd_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_pi_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_ai_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_di_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_ay_ix_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_aw_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_word(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}
pub fn ea_al_16(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea).map(|val| (val as u32, ea)))
}

pub fn ay_pd_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::predecrement_ay_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn ay_pi_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ay_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn ax_pi_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ax_16(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn ay_ai_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn ay_di_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn ay_ix_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn aw_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_word(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn al_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_16(bus, ea)).map(|val| val as u32)
}
pub fn pcdi_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_pc(core, bus)
    .and_then(|ea| core.read_prog_16(bus, ea)).map(|val| val as u32)
}
pub fn pcix_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_pc(core, bus)
    .and_then(|ea| core.read_prog_16(bus, ea)).map(|val| val as u32)
}
pub fn imm_16(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    core.read_imm_data_16(bus)
    .map(|extension| extension as u32)
}

pub fn ea_ay_pd_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ay_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_ax_pd_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::predecrement_ax_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_ay_pi_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::postincrement_ay_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_ay_ai_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_ay_di_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_ay_ix_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_aw_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}
pub fn ea_al_32(core: &mut M68k, bus: &mut impl Bus) -> Result<(u32, u32)> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea).map(|val| (val, ea)))
}

pub fn ay_pd_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::predecrement_ay_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn ay_pi_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ay_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn ax_pi_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::postincrement_ax_32(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn ay_ai_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::address_indirect_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn ay_di_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn ay_ix_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_ay(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn aw_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_word(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn al_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::absolute_long(core, bus)
    .and_then(|ea| core.read_data_32(bus, ea))
}
pub fn pcdi_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::displacement_pc(core, bus)
    .and_then(|ea| core.read_prog_32(bus, ea))
}
pub fn pcix_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    effective_address::index_pc(core, bus)
    .and_then(|ea| core.read_prog_32(bus, ea))
}
pub fn imm_32(core: &mut M68k, bus: &mut impl Bus) -> Result<u32> {
    core.read_imm_data_32(bus)
}
pub fn dx(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(dx!(core))
}
pub fn dy(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(dy!(core))
}
pub fn ay(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(ay!(core))
}
pub fn ax(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok(ax!(core))
}
pub fn quick(core: &mut M68k, _bus: &mut impl Bus) -> Result<u32> {
    Ok((((core.ir as u32 >> 9) - 1) & 7) + 1)
}