use crate::opts::{Opt, SubCommands};
use std::collections::HashMap;
use std::io;
use std::iter::FromIterator;
use structopt::StructOpt;

mod opts;

#[derive(Debug)]
enum CliError {
    IoError(io::Error),
}
fn run_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        Err(());
    }
    Ok(());
}
fn compile_hccr(inputFile: String, lang: String) -> Result<(), CliError> {
    if lang.eq("None") {
        return CliError;
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
        return Err(err);
    }
    Ok(());
}

fn run_subcommand(opts: SubCommands, inputFile: String) -> Result<(), CliError> {
    let inputFile_spt: Vec<String> = Vec::from_iter(inputFile.split(".")).reverse();
    let this_ext: String  = inputFile_spt[0];

    let langs: HashMap<String, String> = HashMap::new();
    langs.insert("hfc".to_string(), "FunCobal".to_string());
    langs.insert("hjer".to_string(), "Hjerbata".to_string());
    langs.insert("ches".to_string(), "FunCobal".to_string());
    langs.insert("jis".to_string(), "FunCobal".to_string());
    let lang_f: String;
    if langs.contains_key(this_ext) {
        lang_f = langs[this_ext]
    } else {
        lang_f = "None";
    }
    match opts {
        SubCommands::Run {} => run_hccr(inputFile, lang_f),
        SubCommands::Cp {} => compile_hccr(inputFile, lang_f),
        SubCommands::Compile {} => compile_hccr(inputFile, lang_f),
        SubCommands::Test {} => test_hccr(inputFile, lang_f),
        SubCommands::New { lang } => new_hccr(inputFile, lang),
        SubCommands::Create { lang } => new_hccr(inputFile, lang),
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
