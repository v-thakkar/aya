use core::ffi::c_void;

use crate::{args::FromBtfArgument, EbpfContext};

pub struct StructOpsContext {
    ctx: *mut c_void,
}

impl StructOpsContext {
    pub fn new(ctx: *mut c_void) -> StructOpsContext {
        StructOpsContext { ctx }
    }

    pub unsafe fn arg<T: FromBtfArgument>(&self, n: usize) -> T {
        T::from_argument(self.ctx as *const _, n)
    }
}

impl EbpfContext for StructOpsContext {
    fn as_ptr(&self) -> *mut c_void {
        self.ctx
    }
}
