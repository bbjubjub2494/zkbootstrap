use std::fs::{File, self};
use std::path::{Path, PathBuf};

fn main() {
    build::rerun_if_env_changed("M2LIBC_PATH");
    let m2libc_path = std::env::var("M2LIBC_PATH").unwrap();
    
    let assets_path = Path::new("assets");

    for name in ["jhex0", "jcat"] {
        compile_m1(assets_path.join(format!("{name}.M1")).as_path(), false);
    }

    compile_m2(Path::new("assets/cat_reference.M2"));
    //compile_m2(Path::new("assets/M1-macro.c"));
    compile_m1(Path::new("assets/hello.M1"), false);
}
fn compile_m2(source: &Path) {
    let prelude_path = "assets/libj_prelude.c";
    let mut dest = build::out_dir().join(source.file_name().unwrap());
    dest.set_extension("M1");
    build::rerun_if_changed(prelude_path);
    build::rerun_if_changed(source);

    let r = std::process::Command::new("M2-Planet")
        .arg("--architecture").arg("riscv32")
        .arg("-f").arg(prelude_path)
        .arg("-f").arg(source)
        .arg("-o").arg(&dest)
        .status().expect("Failed to run M2-Planet");
    assert!(r.success(), "M2-Planet failed");

    compile_m1(&dest, true);
}

fn compile_m1(source: &Path, add_libc: bool) {
    let arch_defs_path = "assets/riscv32_defs.M1";
    let libc_shim_path = "assets/libc-core.M1";
    let mut dest = build::out_dir().join(source.file_name().unwrap());
    dest.set_extension("hex2");
    build::rerun_if_changed(arch_defs_path);
    if add_libc {
        build::rerun_if_changed(libc_shim_path);
    }
    build::rerun_if_changed(source);

    let mut c = std::process::Command::new("M1");

        c.arg("--architecture").arg("riscv32")
        .arg("--little-endian")
        .arg("-f").arg(arch_defs_path);
    if add_libc {
        c.arg("-f").arg(libc_shim_path);
    }
        c.arg("-f").arg(source)
        .arg("-o").arg(&dest);
        
    let r = c.status().expect("Failed to run M1");
    assert!(r.success(), "M1 failed");

    compile_hex2(&dest);
}

fn compile_hex2(source: &Path) {
    let elf_prelude_path = "assets/ELF-riscv32.hex2";
    let mut dest = build::out_dir().join(source.file_name().unwrap());
    dest.set_extension(""); // no extension
    build::rerun_if_changed(elf_prelude_path);
    build::rerun_if_changed(source);

    let r = std::process::Command::new("hex2")
        .arg("--file").arg(elf_prelude_path)
        .arg("--file").arg(source)
        .arg("--architecture").arg("riscv32")
        .arg("--base-address").arg("0x600000")
        .arg("--little-endian")
        .stdout(File::create(&dest).unwrap())
        .status().expect("Failed to run hex2");
    assert!(r.success(), "hex2 failed");
}
