use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Qualys API CLI")]
#[clap(about = "A CLI client for the Qualys API", long_about = None)]
pub struct Cli{
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands{
    #[clap(arg_required_else_help = true)]
    List {
        /// The remote to clone
        #[clap(long,help("Username to access Qualys"))]
         user: String,
         #[clap(long,help("Password to access Qualys"))]
         pass: String,
    },



}




#[derive(Parser, Debug)]
pub struct QualysParser {
    /// Name of the person to greet
    #[clap(short = 'u', long,help("Username to access Qualys"))]
    pub user: String,

    /// Number of times to greet
    #[clap(short = 'p', long,help("Password to access Qualys"))]
    pub pass: String,

    //#[clap(short = 'a', long, help("Actions that be done."))]
}
