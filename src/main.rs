use std::error::Error;
use qualys::*;
use clap::Parser;
use cli::{Cli,Commands};

mod qualys;
mod util;
mod cli;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let args = Cli::parse();
    let mut auth: String = String::new();
    
    match args.command{
        Commands::List {user, pass,client,state,processed,scan_type,target} =>{
            log_in(&mut auth, user,pass).await?;
            scan_actions(&auth,String::from("list")).await?;
        },
        //Todo Fill in the Assets requests
        Commands::Assets{} =>{},
        //Todo Fill in the Scan requests
        Commands::Scan{} => {},
    }

    
    log_out(&auth).await?;
    Ok(())
}

