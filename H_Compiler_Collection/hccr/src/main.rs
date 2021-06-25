use crate::err::CliError;
use crate::opts::Opt;
use crate::opts::SubCommands as sc;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::iter::FromIterator;
use structopt::StructOpt;

mod err;
mod opts;

fn run_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        return Err(CliError::IoError(Error::raw_os_error()));
    }
    Ok(());
}
fn compile_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        CliError(());
    }
    Ok(());
}
fn test_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        Err(());
    }
    Ok(());
}
fn new_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        Err(e);
    }
    Ok(());
}
fn hake_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        Err(e);
    }
    Ok(());
}

fn run_subcommand(opts: sc, inputFile: String) -> Result<(), CliError> {
    let inputFile_spt: Vec<String> = inputFile.rsplit('.').unwrap();
    let this_ext: String = inputFile_spt[0];

    let langs: HashMap<String, String> = HashMap::new();
    langs.insert("hfc".to_string(), "FunCobal".to_string());
    langs.insert("hjer".to_string(), "Hjerbata".to_string());
    langs.insert("ches".to_string(), "FunCobal".to_string());
    langs.insert("jis".to_string(), "FunCobal".to_string());
    let lang_f: String;
    if langs.contains_key(this_ext) {
        lang_f = langs[this_ext]
    } else {
        lang_f = "None".to_string();
    }
    match opts {
        sc::Run {} => run_hccr(inputFile, lang_f),
        sc::Cp {} => compile_hccr(inputFile, lang_f),
        sc::Compile {} => compile_hccr(inputFile, lang_f),
        sc::Test {} => test_hccr(inputFile, lang_f),
        sc::New { lang } => new_hccr(inputFile, lang),
        sc::Create { lang } => new_hccr(inputFile, lang),
        sc::Hake { lang } => hake_hccr(inputFile, lang),
    }
}

fn main() {
    let opt: Opt = Opt::from_args();

    match run_subcommand(opt.sub, opt.inputFile) {
        Ok(_) => (),
        Err(error) => {
            println!("{:?}", error);
        }
    }
}
