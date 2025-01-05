use aya::{programs::StructOps, util::KernelVersion, Btf, Ebpf};
use test_log::test;

#[test]
fn struct_ops() {
    let kernel_version = KernelVersion::current().unwrap();
    if kernel_version < KernelVersion::new(6, 6, 12) {
        eprintln!("skipping struct_ops test on kernel {kernel_version:?}");
        return;
    }

    let mut ebpf = Ebpf::load(crate::STRUCTOPS).unwrap();
    let btf = Btf::from_sys_fs().unwrap();
    let programs: Vec<_> = ebpf.programs().collect();
    println!("len: {}", programs.len());
    for (name, _) in programs {
        println!("found program`{}`", name);
    }
    let prog: &mut StructOps = ebpf
        .program_mut("struct_ops_test")
        .unwrap()
        .try_into()
        .unwrap();
    prog.load(&btf).unwrap();
}
