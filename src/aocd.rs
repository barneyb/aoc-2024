use crate::Part;
use std::io::{self, Write};
use std::process::Command;

pub(crate) fn get_input(year: u32, day: u8) -> io::Result<String> {
    let output = Command::new("aocd")
        .arg(year.to_string())
        .arg(day.to_string())
        .output()
        .expect("Failed to execute 'aocd'");
    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| {
            io::Error::new(
                io::ErrorKind::Other,
                format!("Input contains invalid UTF-8: {e}"),
            )
        })
    } else {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("aocd exited with {}", output.status),
        ))
    }
}

const SUBMIT_WRAPPER: &'static str = include_str!("aocd_submit_wrapper.py");

pub(crate) fn submit_answer(year: u32, day: u8, part: &Part) -> io::Result<bool> {
    let (p, a) = match part {
        Part::A(a) => ("a", a),
        Part::B(a) => ("b", a),
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Only parts a and b can be submitted",
            ))
        }
    };
    // python src/aocd_submit_wrapper.py 2015 1 a 280
    let output = Command::new("python")
        .arg("-c")
        .arg(SUBMIT_WRAPPER)
        .arg(year.to_string())
        .arg(day.to_string())
        .arg(p)
        .arg(a.to_string())
        .output()
        .expect("Failed to execute 'aocd'");
    if output.status.success() {
        Ok(true)
    } else {
        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Ok(false)
    }
}
