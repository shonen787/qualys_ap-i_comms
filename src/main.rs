use std::error::Error;
use qualys::*;
use clap::Parser;
use cli::{QualysParser,Cli,Commands};

mod qualys;
mod util;
mod cli;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();
    let mut auth: String = String::new();
    let mut action: String = String::new();
    
    match args.command{
        Commands::List {user, pass} =>{
            log_in(&mut auth, user,pass).await?;
            scan_actions(&auth,String::from("list")).await?;
        },
    }


    
    log_out(&auth).await?;
    // println!("Auth value is: \n{}", auth);

    Ok(())
}

