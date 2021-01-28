use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub sub: SubCommands,
    pub inputFile: String,
}

#[derive(StructOpt)]
#[structopt(about = "hccr COMMAND INPUT-FILES [OPTIONS, ...]")]
pub enum SubCommands {
    #[structopt(about = "Run File or Application")]
    Run {},
    #[structopt(about = "Compile File or Application")]
    Cp {},
    #[structopt(about = "Compile File or Application")]
    Compile {},
    #[structopt(about = "Test Application")]
    Test {},
    #[structopt(about = "Make New Application")]
    New { lang: String },
    #[structopt(about = "Make New Application")]
    Create { lang: String },
}
