//! struct_ops

/// 


#[derive(Debug)]
#[doc(alias = "BPF_PROG_TYPE_STRUCT_OPS")]
pub struct StructOps {
    pub(crate) data: ProgramData<StructOpsLink>,
}

impl StructOps {
    /// Loads the program inside the kernel
    pub fn load(&mut self) -> Result<(), ProgramError> {
        load_program(BPF_PROG_TYPE_STRUCT_OPS, &mut self.data0
    }

    /// Attaches the program

    /// Detaches the program

    ///
