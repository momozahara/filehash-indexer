use clap::{arg, ArgAction, Command};
use colored::Colorize;
use data_encoding::HEXUPPER;
use regex::Regex;
use ring::digest::{Context, Digest, SHA256};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{self, DirEntry, File},
    io::{self, BufReader, Read},
    path::Path,
    process,
    sync::Mutex,
};

#[derive(Serialize, Deserialize, Clone)]
struct Version {
    version: String,
    assets: Vec<FileData>,
}

#[derive(Serialize, Deserialize, Clone)]
struct FileData {
    path: String,
    hash: String,
}

fn main() -> io::Result<()> {
    let matches = Command::new("HashIndexer")
        .disable_version_flag(true)
        .author("Pream Pinbut")
        .arg(arg!(--path <value>).short('p').required(true))
        .arg(arg!(--version <value>).short('v').required(true))
        .arg(
            arg!(--pretty <value>)
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .arg(
            arg!(--print <value>)
                .action(ArgAction::SetTrue)
                .required(false),
        )
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();

    let origin_dir = env::current_dir()?;
    env::set_current_dir(Path::new(path))?;

    let ver = matches.get_one::<String>("version").unwrap();
    let pretty = matches.get_one::<bool>("pretty").unwrap();
    let print = matches.get_one::<bool>("print").unwrap();

    // source: https://regex101.com/r/JOytBR/1/codegen?language=rust
    let regex = Regex::new(r"(?m)^(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$").unwrap();
    if !regex.is_match(ver) {
        eprint("version is in wrong format!");
    }

    let files: Mutex<Vec<FileData>> = Mutex::new(Vec::new());

    visit_dirs(Path::new("."), &|entry| {
        let mut paths = files.lock().unwrap();
        let path_buf = entry.path();
        let path = path_buf.clone().into_os_string().into_string().unwrap();

        // https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html#calculate-the-sha-256-digest-of-a-file
        let input = File::open(&path).unwrap();
        let reader = BufReader::new(input);
        let digest = sha256_digest(reader).unwrap();
        let hash = HEXUPPER.encode(digest.as_ref()).to_lowercase();

        if cfg!(debug_assertions) || *print {
            println!("{} {}", hash, path);
        }
        paths.push(FileData { path, hash });
    })?;

    let files_lock = files.lock().unwrap();

    let version = Version {
        version: ver.clone(),
        assets: files_lock.clone(),
    };

    let json: String;
    match pretty {
        true => {
            json = serde_json::to_string_pretty(&version)?;
        }
        false => {
            json = serde_json::to_string(&version)?;
        }
    }

    env::set_current_dir(origin_dir)?;
    let out_path = env::current_dir()?.join("out");
    if !out_path.exists() {
        fs::create_dir_all(&out_path)?;
    }
    fs::write(out_path.join("index.json"), json)?;

    println!(
        "{}: {}",
        "success".bold().bright_green(),
        out_path.join("index.json").as_os_str().to_str().unwrap()
    );

    Ok(())
}

fn eprint(msg: &str) -> ! {
    eprintln!("{}: {}", "error".bold().bright_red(), msg);
    process::exit(1)
}

/// source: https://doc.rust-lang.org/std/fs/fn.read_dir.html#examples
fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/// source: https://rust-lang-nursery.github.io/rust-cookbook/cryptography/hashing.html#calculate-the-sha-256-digest-of-a-file
fn sha256_digest<R: Read>(mut reader: R) -> io::Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}
