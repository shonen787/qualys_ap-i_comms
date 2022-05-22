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
         #[clap(long,help("Search for scans after a date format (UTC/GMT), like “2007-07-01” or “2007-01-25T23:12:00Z”."))]
         afterdate:Option<String>,
         #[clap(long,help("Search for scans before a date format (UTC/GMT), like “2007-07-01” or “2007-01-25T23:12:00Z”."))]
         beforedate:Option<String>,
    },
    /// Modify or search Assets
    #[clap(arg_required_else_help = true)]
    Assets{
        #[clap(short,long,help("Username to access Qualys. *Required"))]
        user: String,
       #[clap(short,long,help("Password to access Qualys. *Required"))]
        pass: String,
        #[clap(short,long,help("Which action do you want to perform?\n\t🕳️List\n\t🕳️Add\n\t🕳️Update"))]
        action: String,
        #[clap(short,long,help("Show only certain IP addresses/ranges. One or more IPs/ranges may be specified."))]
        ips: Option<String>,
        #[clap(short,long,help("Show only IP addresses/ranges which have a certain tracking method. Valid values: IP, DNS, NETBIOS."))]
        tracking_method: Option<String>,

    },
    /// Prepare a scan
    Scan{},

}

