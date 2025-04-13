use std::fs::File;

fn main() {
    build::rerun_if_env_changed("M2LIBC_PATH");
    let m2libc_path = std::env::var("M2LIBC_PATH").unwrap();
    
    let arch_defs_path = "assets/riscv32_defs.M1";
    let m1_path = "assets/jcat.M1";
    build::rerun_if_changed(arch_defs_path);
    build::rerun_if_changed(m1_path);

    let mut hex2_path = build::out_dir();
    hex2_path.push("jcat.hex2");
    let mut out_path = build::out_dir();
    out_path.push("jcat");

    let r = std::process::Command::new("M1")
        .arg("--architecture").arg("riscv32")
        .arg("--little-endian")
        .arg("-f").arg(arch_defs_path)
        .arg("-f").arg(m1_path)
        .arg("-o").arg(&hex2_path)
        .status().expect("Failed to run M1");
    assert!(r.success(), "M1 failed");

    let r = std::process::Command::new("hex2")
        .arg("--file").arg("assets/ELF-riscv32.hex2")
        .arg("--file").arg(&hex2_path)
        .arg("--architecture").arg("riscv32")
        .arg("--base-address").arg("0x600000")
        .arg("--little-endian")
        .stdout(File::create(out_path).unwrap())
        .status().expect("Failed to run hex2");
    assert!(r.success(), "hex2 failed");
}
