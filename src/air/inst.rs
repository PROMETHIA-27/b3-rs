use tinyvec::TinyVec;

use crate::{bank::Bank, sparse_collection::SparseElement, value::ValueId, width::Width, jit::reg::Reg};

use super::{
    arg::{Arg, ArgRole},
    form_table::{decode_form_bank, decode_form_role, decode_form_width},
    kind::Kind,
    opcode::Opcode,
    opcode_generated::G_FORM_TABLE,
    tmp::Tmp, stack_slot::StackSlotId,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Inst {
    pub args: TinyVec<[Arg; 3]>,
    pub origin: ValueId,
    pub kind: Kind,
    pub index: usize,
}

impl Default for Inst {
    fn default() -> Self {
        Self {
            args: TinyVec::new(),
            origin: ValueId::default(),
            kind: Kind::default(),
            index: usize::MAX,
        }
    }
}

impl Inst {

    pub fn has_late_use_or_def(&self) -> bool {
        if self.kind.opcode == Opcode::Patch {
            todo!()
        }
        let mut result = false;

        self.for_each_arg(|_arg, role, _, _| {
            result |= role.is_late_use() || role.is_late_def();
        });

        result
    }

    pub fn has_early_def(&self) -> bool {
        if self.kind.opcode == Opcode::Patch {
            todo!()
        }
        let mut result = false;

        self.for_each_arg(|_arg, role, _, _| {
            result |= role.is_early_def();
        });

        result
    }

    pub fn needs_padding(prev: &Self, next: &Self) -> bool {
        prev.has_late_use_or_def() && next.has_early_def()
    }

    pub fn for_each_arg_simple<F>(&self, mut f: F)
    where
        F: FnMut(Arg, ArgRole, Bank, Width),
    {
        let num_operands = self.args.len();
        
        let form_offset = num_operands.wrapping_sub(1) * num_operands / 2;
        let form_base = &G_FORM_TABLE[self.kind.opcode as usize * 21 + form_offset..];

        for i in 0..num_operands {
            let form = form_base[i];
            f(
                self.args[i],
                decode_form_role(form),
                decode_form_bank(form),
                decode_form_width(form),
            )
        }
    }

    pub fn for_each_arg_simple_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut Arg, ArgRole, Bank, Width),
    {
        let num_operands = self.args.len();
        
        let form_offset = (num_operands.wrapping_sub(1)) * num_operands / 2;
        let form_base = &G_FORM_TABLE[self.kind.opcode as usize * 21 + form_offset..];

        for i in 0..num_operands {
            let form = form_base[i];
            f(
                &mut self.args[i],
                decode_form_role(form),
                decode_form_bank(form),
                decode_form_width(form),
            )
        }
    }

    pub fn new(kind: Kind, origin: ValueId, arguments: &[Arg]) -> Self {
        Inst {
            args: arguments.iter().copied().collect(),
            origin,
            kind,
            index: 0,
        }
    }

    pub fn for_each_arg(&self, f: impl FnMut(Arg, ArgRole, Bank, Width)) {
        match self.kind.opcode {
            Opcode::EntrySwitch => {
                todo!()
            }

            Opcode::Shuffle => {
                todo!()
            }

            Opcode::Patch => {
                todo!()
            }

            Opcode::CCall => {
                todo!()
            }

            Opcode::ColdCCall => {
                todo!()
            }

            Opcode::WasmBoundsCheck => {
                todo!()
            }

            _ => self.for_each_arg_simple(f),
        }
    }

    pub fn for_each_arg_mut(&mut self, f: impl FnMut(&mut Arg, ArgRole, Bank, Width)) {
        match self.kind.opcode {
            Opcode::EntrySwitch => {
                todo!()
            }

            Opcode::Shuffle => {
                todo!()
            }

            Opcode::Patch => {
                todo!()
            }

            Opcode::CCall => {
                todo!()
            }

            Opcode::ColdCCall => {
                todo!()
            }

            Opcode::WasmBoundsCheck => {
                todo!()
            }

            _ => self.for_each_arg_simple_mut(f),
        }
    }

    pub fn for_each_tmp(&self, mut f: impl FnMut(Tmp, ArgRole, Bank, Width)) {
        self.for_each_arg(|arg, role, bank, width| {
            arg.for_each_tmp(role, bank, width, |tmp, role, bank, width| {
                f(tmp, role, bank, width)
            })
        })
    }

    pub fn for_each_tmp_mut(&mut self, mut f: impl FnMut(&mut Tmp, ArgRole, Bank, Width)) {
        self.for_each_arg_mut(|arg, role, bank, width| {
            arg.for_each_tmp_mut(role, bank, width, |tmp, role, bank, width| {
                f(tmp, role, bank, width)
            })
        })
    }

    pub fn for_each_tmp_fast(&self, mut f: impl FnMut(Tmp)) {
        self.for_each_arg(|arg, _, _, _| {
            arg.for_each_tmp_fast(|tmp| {
                f(tmp)
            })
        })
    }

    pub fn for_each_tmp_fast_mut(&mut self, mut f: impl FnMut(&mut Tmp)) {
        self.for_each_arg_mut(|arg, _, _, _| {
            arg.for_each_tmp_fast_mut(|tmp| {
                f(tmp)
            })
        })
    }

    pub fn for_each_reg(&self, mut f: impl FnMut(Reg, ArgRole, Bank, Width)) {
        self.for_each_arg(|arg, role, bank, width| {
            arg.for_each_reg(role, bank, width, |reg, role, bank, width| {
                f(reg, role, bank, width)
            })
        })
    }

    pub fn for_each_reg_mut(&mut self, mut f: impl FnMut(&mut Reg, ArgRole, Bank, Width)) {
        self.for_each_arg_mut(|arg, role, bank, width| {
            arg.for_each_reg_mut(role, bank, width, |reg, role, bank, width| {
                f(reg, role, bank, width)
            })
        })
    }

    pub fn for_each_reg_fast(&self, mut f: impl FnMut(Reg)) {
        self.for_each_arg(|arg, _, _, _| {
            arg.for_each_reg_fast(|reg| {
                f(reg)
            })
        })
    }

    pub fn for_each_reg_fast_mut(&mut self, mut f: impl FnMut(&mut Reg)) {
        self.for_each_arg_mut(|arg, _, _, _| {
            arg.for_each_reg_fast_mut(|reg| {
                f(reg)
            })
        })
    }

    pub fn for_each_stack_slot(&self, mut f: impl FnMut(StackSlotId, ArgRole, Bank, Width)) {
        self.for_each_arg(|arg, role, bank, width| {
            arg.for_each_stack_slot(role, bank, width, |stack_slot, role, bank, width| {
                f(stack_slot, role, bank, width)
            })
        })
    }

    pub fn for_each_stack_slot_mut(&mut self, mut f: impl FnMut(&mut StackSlotId, ArgRole, Bank, Width)) {
        self.for_each_arg_mut(|arg, role, bank, width| {
            arg.for_each_stack_slot_mut(role, bank, width, |stack_slot, role, bank, width| {
                f(stack_slot, role, bank, width)
            })
        })
    }

    pub fn for_each_stack_slot_fast(&self, mut f: impl FnMut(StackSlotId)) {
        self.for_each_arg(|arg, _, _, _| {
            arg.for_each_stack_slot_fast(|stack_slot| {
                f(stack_slot)
            })
        })
    }

    pub fn for_each_stack_slot_fast_mut(&mut self, mut f: impl FnMut(&mut StackSlotId)) {
        self.for_each_arg_mut(|arg, _, _, _| {
            arg.for_each_stack_slot_fast_mut(|stack_slot| {
                f(stack_slot)
            })
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct InstId(pub usize);

impl From<usize> for InstId {
    fn from(index: usize) -> Self {
        InstId(index)
    }
}

impl From<InstId> for usize {
    fn from(id: InstId) -> Self {
        id.0
    }
}

impl SparseElement for Inst {
    type Id = InstId;

    fn id(&self) -> Self::Id {
        InstId(self.index)
    }

    fn set_id(&mut self, id: Self::Id) {
        self.index = id.0;
    }
}

impl std::fmt::Display for Inst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.kind)?;
        for (i, arg) in self.args.iter().enumerate() {
            write!(f, "{}", arg)?;

            if i != self.args.len() - 1 {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}