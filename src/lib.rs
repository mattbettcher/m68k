//#![feature(universal_impl_trait)]
//#![feature(nll)]

#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod instructions;

use std::num::Wrapping;
use instructions::constants::*;
use instructions::optable::generate;
use std::result;

#[derive(Debug)]
pub enum Exception {
    AddressError,
    IllegalInstruction(u16, u32),           // opcode, pc
    Trap(u8, u32),                          // trap number, cycles
    PrivilegeViolation(u16, u32),           // opcode, pc
    UnimplementedInstruction(u16, u32, u8), // ir, pc, vector no
    Interrupt(u8, u8),                      // irq, vector number
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Exception::AddressError => write!(f, "Address Error"),
            Exception::IllegalInstruction(ir, pc) => write!(f, "Illegal Instruction {:04x} at {:08x}", ir, pc),
            Exception::Trap(num, ea_cyc) => write!(f, "Trap: {:02x} (ea cyc {})", num, ea_cyc),
            Exception::PrivilegeViolation(ir, pc) => write!(f, "Privilege Violation {:04x} at {:08x}", ir, pc),
            Exception::UnimplementedInstruction(ir, pc, _) => write!(f, "Unimplemented Instruction {:04x} at {:08x}", ir, pc),
            Exception::Interrupt(irq, vec) => write!(f, "Interrupt {:1x} (vector {:02x})", irq, vec),
        }
    }
}

pub enum Condition {
    True,   // True            1
    False,   // False           0
    HI,  // High            !C & !Z
    LS,  // LowOrSame       C | Z
    CC,  // CarryClearHI    !C
    CS,  // CarrySetLO      C
    NE,  // NotEqual        !Z
    EQ,  // Equal           Z
    VC,  // OverflowClear   !V
    VS,  // OverflowSet     V
    PL,  // Plus            !N
    MI,  // Minus           N
    GE,  // GreaterOrEqual  N & V | !N & !V
    LT,  // LessThan        N & !V | !N & V
    GT,  // GreaterThan     N & V & !Z | !N & !V & !Z
    LE,  // LessOrEqual     Z | N & !V | !N & V
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct AddressSpace(Mode, Segment);

impl AddressSpace {
    pub fn fc(&self) -> u32 {
        match *self {
            USER_DATA => 1,
            USER_PROGRAM => 2,
            SUPERVISOR_DATA => 5,
            SUPERVISOR_PROGRAM => 6,
        }
    }
}
use std::fmt;
impl fmt::Debug for AddressSpace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressSpace(mode, segment) => write!(f, "[{:?}/{:?}]", mode, segment),
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Segment {
    Program, Data
}
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Mode {
    User, Supervisor
}

pub const SUPERVISOR_PROGRAM: AddressSpace = AddressSpace(Mode::Supervisor, Segment::Program);
pub const SUPERVISOR_DATA: AddressSpace = AddressSpace(Mode::Supervisor, Segment::Data);
pub const USER_PROGRAM: AddressSpace = AddressSpace(Mode::User, Segment::Program);
pub const USER_DATA: AddressSpace = AddressSpace(Mode::User, Segment::Data);

pub type Result<T> = result::Result<T, Exception>;
pub type Handler<'a> = fn(&mut M68k, &mut (Bus + 'a)) -> Result<u32>;
pub type InstructionSet<'a> = Vec<Handler<'a>>;

pub trait Bus {
    fn read_8(&self, space: AddressSpace, addr: u32) -> u8;
    fn read_16(&self, space: AddressSpace, addr: u32) -> u16;
    fn read_32(&self, space: AddressSpace, addr: u32) -> u32;

    fn write_8(&mut self, space: AddressSpace, addr: u32, value: u8);
    fn write_16(&mut self, space: AddressSpace, addr: u32, value: u16);
    fn write_32(&mut self, space: AddressSpace, addr: u32, value: u32);
}

pub enum Version {
    MC68000,
    MC68010,
    MC68020,
    //MC68030, // todo !!!!
    //MC68040,
}

#[derive(Copy, Clone, Default)]
pub struct CacheLine020 {
    pub tag: u32,
    pub v: bool,
    pub word: [u16; 2],
}

pub struct M68k<'a> {
    pub version: Version,
    pub pc: u32,
    pub inactive_msp: u32, // when in user mode
    pub inactive_usp: u32, // when in supervisor mode
    pub ir: u16,
    pub dar: [u32; 16],
    pub s: u32,
    pub m: u32,
    pub irq_level: u8,
    pub int_mask: u32,
    pub x: u32,
    pub c: u32,
    pub v: u32,
    pub n: u32,
    pub not_z: u32,
    // '010+
    pub vbr: u32,
    pub sfc: u32,
    pub dfc: u32,
    // '020+
    pub cahr: u32,      // cache holding register (ref MC68020UM Fig 1-5. Instruction Pipe)
    pub caar: u32,
    pub cacr: u32,
    pub inactive_isp: u32,

    pub cache_enabled: bool,    // this represents the external pin???
    pub cache: [CacheLine020; 64], // '020 only! other caches are different

    pub ops: InstructionSet<'a>,
}

impl<'a> M68k<'a> {
    pub fn new(version: Version) -> Self {
        M68k {
            version: version,
            pc: 0, inactive_msp: 0, inactive_usp: 0, inactive_isp: 0, ir: 0,
            dar: [0u32; 16], 
            irq_level: 0, 
            s: SFLAG_SET, m: MFLAG_SET, int_mask: 0, x: 0, v: 0, c: 0, n: 0, not_z: 0xffffffff,
            vbr: 0,
            caar: 0,
            cacr: 0,
            cahr: 0,
            sfc: 0, dfc: 0,
            cache_enabled: true,
            cache: [CacheLine020::default(); 64],

            ops: generate(),
        }
    }

    pub fn reset<T: Bus + ?Sized>(&mut self, bus: &mut T) {
        self.s = SFLAG_SET;
        self.int_mask = 0x7;
        self.pc = 0;
        sp!(self) = self.read_imm_prog_32(bus).unwrap();
        self.pc = self.read_imm_prog_32(bus).unwrap();
    }

    // returns # of cycles used
    pub fn step<T: Bus + 'a>(&mut self, bus: &mut T) -> u32 {
        // handle interrupts here
        
        // this isn't correct for any model currently
        self.ir = self.read_imm_prog_16(bus).unwrap();
        let op = self.ops[self.ir as usize];
        let cycles_used = (op)(self, bus);

        match cycles_used {
            Ok(cycles) => cycles,
            Err(e) => unimplemented!(),
        }
        //self.pc = self.pc.wrapping_add(2);
    }

    pub fn status_register(&self) -> u16 {
        ((self.s << SFLAG_BIT)         |
        (self.m << MFLAG_BIT)          |
        (self.int_mask << INT_BITS)    |
        ((self.x & XFLAG_SET) >> 4)    |
        ((self.n & NFLAG_SET) >> 4)    |
        ((not1!(self.not_z))  << 2)    |
        ((self.v & VFLAG_SET) >> 6)    |
        ((self.c & CFLAG_SET) >> 8)) as u16
    }

    pub fn condition_code_register(&self) -> u16 {
        self.status_register() & 0xff
    }

    pub fn sr_to_flags(&mut self, sr: u16) {
        let old_mflag = self.m;
        let sr = (sr & CPU_SR_MASK) as u32;                                 // mask out any invalid bits
        let old_sflag = self.s;                                             // save old status
        self.int_mask = (sr & CPU_SR_INT_MASK) >> INT_BITS;                 // get interrupt level mask
        self.s = sr & SFLAG_SET;                                            // get s flag
        self.m = sr & MFLAG_SET;                                            // get m flag
        // below remains unchanged so far
        self.x = (sr <<  4) & XFLAG_SET;
        self.n = (sr <<  4) & NFLAG_SET;
        self.not_z = not1!(sr & 0b00100);
        self.v = (sr <<  6) & VFLAG_SET;
        self.c = (sr <<  8) & CFLAG_SET;
        // account for s & m flags as per M68020UM 2.1 (Note: this should be backward compatible with all earlier models)
        // this needs tested, I'm not 100% clear on every edge case here
        if old_sflag != self.s {
            if self.s == SFLAG_SET {            // change to supervisor level
                self.inactive_usp = sp!(self);  // save usp
                if self.m == MFLAG_SET {
                    sp!(self) = self.inactive_msp;  // if m flag is set use msp
                } else {
                    sp!(self) = self.inactive_isp;  // if m flag is clear use isp
                }
            } else {
                if old_mflag == MFLAG_SET {
                    self.inactive_msp = sp!(self);  // if m flag was set save msp
                } else {
                    self.inactive_isp = sp!(self);  // if m flag was clear save isp
                }
                sp!(self) = self.inactive_usp;
            }
        }
    }

    pub fn ccr_to_flags(&mut self, ccr: u16) {
        let sr = self.status_register();
        self.sr_to_flags((sr & 0xff00) | (ccr & 0xff));
    }

    fn condition(&self, c: Condition) -> bool {
        match c {
            Condition::True  => true,
            Condition::False  => false,
            Condition::HI => (self.c & CFLAG_SET==0) && (self.not_z != ZFLAG_SET),
            Condition::LS => (self.c & CFLAG_SET!=0) || (self.not_z == ZFLAG_SET),
            Condition::CC => self.c & CFLAG_SET==0,
            Condition::CS => self.c & CFLAG_SET!=0,
            Condition::NE => (self.not_z != ZFLAG_SET),
            Condition::EQ => (self.not_z == ZFLAG_SET),
            Condition::VC => (self.v & VFLAG_SET==0),
            Condition::VS => (self.v & VFLAG_SET!=0),
            Condition::PL => (self.n & NFLAG_SET==0),
            Condition::MI => (self.n & NFLAG_SET!=0),
            Condition::GE => (self.n & NFLAG_SET!=0) && (self.v & VFLAG_SET!=0) || (self.n & NFLAG_SET==0) && (self.v & VFLAG_SET==0),
            Condition::LT => (self.n & NFLAG_SET!=0) && (self.v & VFLAG_SET==0) || (self.n & NFLAG_SET==0) && (self.v & VFLAG_SET!=0),
            Condition::GT => (self.n & NFLAG_SET!=0) && (self.v & VFLAG_SET!=0) && (self.not_z != ZFLAG_SET) || (self.n & NFLAG_SET==0) && (self.v & VFLAG_SET==0) && (self.not_z != ZFLAG_SET),
            Condition::LE => (self.not_z == ZFLAG_SET) || (self.n & NFLAG_SET!=0) && (self.v & VFLAG_SET==0) || (self.n & NFLAG_SET==0) && (self.v & VFLAG_SET!=0),
        }
    }
    fn push_sp<T: Bus + ?Sized>(&mut self, bus: &mut T) -> u32 {
         let new_sp = (Wrapping(sp!(self)) - Wrapping(4)).0;
         sp!(self) = new_sp;
         self.write_data_32(bus, new_sp, new_sp).unwrap();
         new_sp
    }
    fn push_32<T: Bus + ?Sized>(&mut self, bus: &mut T, value: u32) -> u32 {
         let new_sp = (Wrapping(sp!(self)) - Wrapping(4)).0;
         sp!(self) = new_sp;
         self.write_data_32(bus, new_sp, value).unwrap();
         new_sp
    }
    fn pop_32<T: Bus + ?Sized>(&mut self, bus: &mut T) -> u32 {
        let sp = sp!(self);
        let data = self.read_data_32(bus, sp).unwrap();
        sp!(self) = sp.wrapping_add(4);
        data
    }
    fn push_16<T: Bus + ?Sized>(&mut self, bus: &mut T, value: u16) -> u32 {
         let new_sp = (Wrapping(sp!(self)) - Wrapping(2)).0;
         sp!(self) = new_sp;
         self.write_data_16(bus, new_sp, value).unwrap();
         new_sp
    }
    fn pop_16<T: Bus + ?Sized>(&mut self, bus: &mut T) -> u16 {
        let sp = sp!(self);
        let data = self.read_data_32(bus, sp).unwrap() as u16;
        sp!(self) = sp.wrapping_add(2);
        data
    }

    fn write_data_8<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32, value: u8) -> Result<()> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.write_8(address_space, addr, value))
    }

    fn write_data_16<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32, value: u16) -> Result<()> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.write_16(address_space, addr, value))
    }

    fn write_data_32<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32, value: u32) -> Result<()> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.write_32(address_space, addr, value))
    }

    fn read_data_8<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u8> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.read_8(address_space, addr))
    }

    fn read_data_16<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u16> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.read_16(address_space, addr))
    }

    fn read_data_32<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u32> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.read_32(address_space, addr))
    }

    fn read_prog_8<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u8> {
        let address_space = if self.s != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        Ok(bus.read_8(address_space, addr))
    }

    fn read_prog_16<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u16> {
        let address_space = if self.s != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        Ok(bus.read_16(address_space, addr))
    }

    fn read_prog_32<T: Bus + ?Sized>(&mut self, bus: &mut T, addr: u32) -> Result<u32> {
        let address_space = if self.s != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        Ok(bus.read_32(address_space, addr))
    }

    fn read_imm_data_16<T: Bus + ?Sized>(&mut self, bus: &mut T) -> Result<u16> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.read_16(address_space, self.pc))
    }

    fn read_imm_data_32<T: Bus + ?Sized>(&mut self, bus: &mut T) -> Result<u32> {
        let address_space = if self.s != 0 {SUPERVISOR_DATA} else {USER_DATA};
        Ok(bus.read_32(address_space, self.pc))
    }

    fn read_imm_prog_16<T: Bus + ?Sized>(&mut self, bus: &mut T) -> Result<u16> {
        let address_space = if self.s != 0 {SUPERVISOR_PROGRAM} else {USER_PROGRAM};
        match self.version {
            Version::MC68000 => {
                // uses prefetch
                unimplemented!()
             },
            Version::MC68010 => { 
                // loop mode???
                unimplemented!()
            },
            Version::MC68020 => { 
                if self.pc & 1 > 0 {
                    return Err(Exception::AddressError)
                }

                // instruction cache
                if self.cache_enabled && (self.cacr & 1) == 1 { // TODO - make these consts????
                    let tag = ((self.pc & 0xFFFFFF00) >> 8) as u32;
                    let index = ((self.pc & 0xfc) >> 2) as usize;
                    let word_sel = ((self.pc & 0x2) >> 1) as usize;
                    let line = self.cache[index];
                    if line.v && line.tag == tag { // line must be valid and tag same to get a hit
                        // cache hit! set ir from cache
                        Ok(line.word[word_sel])
                    } else {
                        // cache miss! do a real fetch!
                        let lw = bus.read_32(address_space, self.pc & 0xffff_fffc);  // man says we always do a long word aligned instruction fetches, pc should be aligned here by masking?
                        let low_w = ((lw & 0x0000_ffff) >> 0) as u16;
                        let high_w = ((lw & 0xffff_0000) >> 16) as u16;
                        if self.caar & 2 >> 1 == 0 {    // if the cache isn't frozen, update it
                            self.cache[index].v = true;
                            self.cache[index].tag = tag;                            
                            self.cache[index].word[0] = low_w;
                            self.cache[index].word[1] = high_w;
                        }
                        // finally set the ir to the correct part of the 32 bits we read
                        if word_sel == 0 {
                            Ok(low_w)
                        } else {
                            Ok(high_w)
                        }
                    }
                } else {
                    let lw = bus.read_32(address_space, self.pc & 0xffff_fffc);  // man says we always do a long word aligned instruction fetches, pc should be aligned here by masking?
                    let low_w = ((lw & 0x0000_ffff) >> 0) as u16;
                    let high_w = ((lw & 0xffff_0000) >> 16) as u16;
                    let word_sel = ((self.pc & 0x2) >> 1) as usize;
                    // finally set the ir to the correct part of the 32 bits we read
                    if word_sel == 0 {
                        Ok(low_w)
                    } else {
                        Ok(high_w)
                    }
                }
            },
        }
    }

    fn read_imm_prog_32<T: Bus + ?Sized>(&mut self, bus: &mut T) -> Result<u32> {
        unimplemented!()
    }
}