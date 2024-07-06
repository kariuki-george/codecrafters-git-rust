use clap::{Parser, Subcommand};
use flate2::bufread::ZlibDecoder;
use std::fs;
use std::io::Read;

#[derive(Parser, Debug)]
#[command(name = "Mitheko", version)]
pub struct Args {
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    println!("Logs from your program will appear here!");

    let args = Args::parse();

    match args.command {
        Command::Init => init(),
        Command::CatFile { p, s, t } => {
            if let Some(path) = p {
                cat_file(path, CatOpt::P)
            }
            if let Some(path) = s {
                cat_file(path, CatOpt::S)
            }
            if let Some(path) = t {
                cat_file(path, CatOpt::T)
            }
        }
    }
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Initializes a git repo
    Init,
    ///  Prints the contents of an object   
    CatFile {
        /// Prints the content of the file
        #[clap(short = 'p')]
        p: Option<String>,
        /// Prints the size of the content
        #[clap(short = 's')]
        s: Option<String>,
        /// Prints the type of the object
        #[clap(short = 't')]
        t: Option<String>,
    },
}

fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
    println!("Initialized git directory")
}

enum CatOpt {
    P,
    S,
    T,
}

fn cat_file(mut file: String, opt: CatOpt) {
    // Uses ZLIB COMPRESSION
    // Read the file
    // Decode the file
    // Print the contents

    // file.insert(2, '/');

    let contents = fs::read(file).expect("Unable to read the contents of the file");

    // Decompress

    let mut d = ZlibDecoder::new(&contents[..]);

    let mut contents = String::new();

    d.read_to_string(&mut contents).unwrap();

    let (header, content) = contents.split_once('\0').expect("Invalid file passed");
    let (object_type, size) = header.split_once(' ').expect("Invalid file passed");

    match opt {
        CatOpt::P => print!("{content}"),
        CatOpt::S => print!("{size}"),
        CatOpt::T => print!("{object_type}"),
    }
}
