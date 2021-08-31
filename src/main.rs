use std::fmt::Debug;
use std::{ffi::OsStr, fs, fs::File, io, io::Read, path::Path};

const FIRST: &str = "\n";
const SECOND: &str = "\r\n";

#[inline]
fn count_strings_in_file<P: AsRef<Path> + Debug>(path: P) -> io::Result<u128> {
    let mut file = String::new();
    File::open(path)?.read_to_string(&mut file)?;

    Ok(file
        .trim()
        .split(|x| {
            let s = String::from(x);
            s == FIRST || s == SECOND
        })
        .count() as u128)
}

fn count_strings_in_dir(dir: &Path, formats: &Vec<&OsStr>, cnt: &mut u128) -> io::Result<u128> {
    if dir.is_dir() {
        let dir = fs::read_dir(dir)?;

        for entry in dir {
            let path = entry?.path();

            *cnt += if path.is_dir() {
                count_strings_in_dir(&path, formats, &mut 0)?
            } else {
                let path = path.to_str().unwrap().to_string();

                if formats.iter().any(|&f| path.ends_with(f.to_str().unwrap())) {
                    count_strings_in_file(path)?
                } else {
                    0
                }
            };
        }

        Ok(*cnt)
    } else {
        Ok(*cnt + count_strings_in_file(dir)?)
    }
}

fn main() -> io::Result<()> {
    println!("Directory of files in which strings should be counted:");

    let mut dir = String::new();
    io::stdin().read_line(&mut dir)?;
    let dir = Path::new(dir.trim());

    println!(
        "Formats of files in which strings should be counted (example input: rs kt java xml c):"
    );

    let mut formats = String::new();
    io::stdin().read_line(&mut formats)?;

    println!(
        "Strings founded: {}",
        count_strings_in_dir(
            dir,
            &formats
                .trim()
                .split_whitespace()
                .map(|s| OsStr::new(s))
                .collect::<Vec<_>>(),
            &mut 0
        )?
    );

    Ok(())
}
