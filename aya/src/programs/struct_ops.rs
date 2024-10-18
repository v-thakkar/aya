//! struct_ops

use crate::{
    generated::{bpf_attach_type::BPF_STRUCT_OPS, bpf_prog_type::BPF_PROG_TYPE_STRUCT_OPS},
    obj::btf::{Btf, BtfKind},
    programs::{define_link_wrapper, load_program, FdLink, FdLinkId, ProgramData, ProgramError},
};

/// A program that attaches to struct_ops
/// TODO: Write actual sane doc here
#[derive(Debug)]
#[doc(alias = "BPF_PROG_TYPE_STRUCT_OPS")]
pub struct StructOps {
    pub(crate) data: ProgramData<StructOpsLink>,
}

impl StructOps {
    /// Loads the program inside the kernel
    pub fn load(&mut self, btf: &Btf) -> Result<(), ProgramError> {
        self.data.expected_attach_type = Some(BPF_STRUCT_OPS);
        let type_name = "sched_ext_ops";
        self.data.attach_btf_id = Some(btf.id_by_type_name_kind(type_name, BtfKind::Struct)?);
        load_program(BPF_PROG_TYPE_STRUCT_OPS, &mut self.data)
    }

    // Attaches the program
}

define_link_wrapper!(
    /// The link used by struct_ops programs
    StructOpsLink,
    /// The type returned by [StructOps::attach]. Can be passed to [StructOps::detach].
    StructOpsLinkId,
    FdLink,
    FdLinkId,
    StructOps,
);
