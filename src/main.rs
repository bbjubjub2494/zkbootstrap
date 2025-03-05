use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

fn line_comment(source_file: &mut dyn Read, tape_02: &mut dyn Write) -> io::Result<()> {
    let mut buffer = [0; 1];
    loop {
        source_file.read_exact(&mut buffer)?;
        tape_02.write_all(&buffer)?;
        if buffer[0] == 10 || buffer[0] == 13 {
            break;
        }
    }
    Ok(())
}

fn hex(c: u8, source_file: &mut dyn Read, tape_02: &mut dyn Write) -> io::Result<i32> {
    // Clear out line comments
    if c == b';' || c == b'#' {
        line_comment(source_file, tape_02)?;
        return Ok(-1);
    }

    // Deal with non-hex chars
    if c < b'0' {
        return Ok(-1);
    }

    // Deal with 0-9
    if c <= b'9' {
        return Ok((c - b'0') as i32);
    }

    // Convert a-f to A-F
    let c = c & 0xDF;

    // Get rid of everything below A
    if c < b'A' {
        return Ok(-1);
    }

    // Deal with A-F
    if c <= b'F' {
        return Ok((c - b'A' + 10) as i32);
    }

    // Everything else is garbage
    Ok(-1)
}

fn main() -> io::Result<()> {
    let mut source_file = io::stdin();
    let mut tape_02 = File::create("tape_02")?;
    let mut tape_01 = File::create("tape_01")?;

    let mut toggle = false;
    let mut holder = 0;

    let mut buffer = [0; 1];
    while source_file.read_exact(&mut buffer).is_ok() {
        let c = buffer[0];
        tape_02.write_all(&[c])?;
        let r0 = hex(c, &mut source_file, &mut tape_02)?;
        if r0 >= 0 {
            if toggle {
                let byte = (holder * 16) + r0 as u8;
                tape_01.write_all(&[byte])?;
                holder = 0;
            } else {
                holder = r0 as u8;
            }
            toggle = !toggle;
        }
    }

    Ok(())
}
