#![no_std]
#![no_main]

use aya_ebpf::{macros::struct_ops, programs::StructOpsContext};
use sched_ext_ops::{sched_ext_ops, SchedExtOps};

#[unsafe(link_section = ".struct_ops.link")]
static SIMPLE_OPS: SchedExtOps = sched_ext_ops!(c"name");

#[struct_ops]
pub fn struct_ops_test(_ctx: StructOpsContext) -> i32 {
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
