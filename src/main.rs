extern crate async_recursion;
extern crate futures;

use async_recursion::async_recursion;
use futures::executor::block_on;

use std::{
    ffi::{OsStr, OsString},
    fs,
    fs::File,
    io,
    io::Read,
    path::Path,
    sync::Arc,
};

const POSIX_SEP: &str = "\n";
const WINDOWS_SEP: &str = "\r\n";
const MAC_SEP: &str = "\r";

#[inline]
async fn count_strings_in_file<P: AsRef<Path>>(path: P) -> io::Result<u128> {
    let mut file = String::new();
    File::open(path)?.read_to_string(&mut file)?;

    Ok(file
        .trim()
        .split(|x| {
            let s = String::from(x);
            s == POSIX_SEP || s == WINDOWS_SEP || s == MAC_SEP
        })
        .count() as u128)
}

#[async_recursion]
async fn count_strings_in_dir(dir: &Path, formats: Arc<Vec<OsString>>) -> io::Result<u128> {
    if dir.is_dir() {
        let dir = fs::read_dir(dir)?;

        let tasks = dir
            .into_iter()
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let formats = formats.clone();

                async move {
                    if path.is_dir() {
                        count_strings_in_dir(&path, formats).await.unwrap()
                    } else {
                        let path = path.to_str().unwrap().to_string();

                        if formats.iter().any(|f| path.ends_with(f.to_str().unwrap())) {
                            count_strings_in_file(path).await.unwrap()
                        } else {
                            0
                        }
                    }
                }
            })
            .collect::<Vec<_>>();

        Ok(futures::future::join_all(tasks).await.iter().sum())
    } else {
        Ok(count_strings_in_file(dir).await?)
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
        block_on(count_strings_in_dir(
            dir,
            Arc::new(
                formats
                    .trim()
                    .split_whitespace()
                    .map(|s| OsStr::new(s).to_os_string())
                    .collect::<Vec<_>>()
            )
        ))?
    );

    Ok(())
}
