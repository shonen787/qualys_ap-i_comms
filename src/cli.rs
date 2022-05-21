use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "Qualys API CLI")]
#[clap(author="ERMProtect - CS", about = "A CLI client for the Qualys API", long_about = None)]
pub struct Cli{
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands{
    #[clap(arg_required_else_help = true)]
    /// List Qualys Scans
    List {
        #[clap(long,help("Username to access Qualys. *Required"))]
         user: String,
        #[clap(long,help("Password to access Qualys. *Required"))]
         pass: String,
        #[clap(long,help("Search for scans with string in it's title"))]
         client:Option<String>,
         #[clap(long,help("Search for scans based on the state\nValid values are:\n\t🕳️Running\n\t🕳️Pause\n\t🕳️Canceled\n\t🕳️Finished\n\t🕳️Error\n\t🕳️Queued\n\t🕳️Loading\nMultiple values can be used. Sperate them by commas and no spaces."))]
         state:Option<String>,
         #[clap(long,help("Search for scans based on their process state true or false. "))]
         processed:Option<String>,
         #[clap(long,help("Search for scans based on the type.\nValid values are:\n\t🕳️On-Demand\n\t🕳️Scheduled\n\t🕳️API"))]
         scan_type:Option<String>,
         #[clap(long,help("Search for scans based on the targets.\nEnter ranges using a hyphen, x.x.x.x - x.x.x.x"))]
         target:Option<String>,
    },
    /// Modify or search Assets
    Assets{

    },
    /// Prepare a scan
    Scan{},

}

