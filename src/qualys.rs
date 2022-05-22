use reqwest;
use std::error::Error;
use std::time::Duration;
use url::Url;
use std::{str};
use regex::Regex;

use crate::util::xml;


pub async fn log_in(s: &mut String, user: String, pass: String) -> Result<(), Box<dyn Error>>{
    println!("Going to log in as {} 🐒", user);
   
    //create reqwest client
    let client = reqwest::Client::new();
    
    let session_endpoint = String::from("https://qualysapi.qualys.com/api/2.0/fo/session/");
    let form = reqwest::multipart::Form::new()
    .text("username", String::from(user))
    .text("password", String::from(pass));
  
    let doge = client
        .post(Url::parse(&session_endpoint)?)
        .header("Accept", "text/plain")
        .header("X-Requested-With","Rust-Execv2")
        .query(&[("action","login")])
        .multipart(form)
        .timeout(Duration::from_secs(3))
        .send()
        .await?;

        
        let cookies=  match doge.headers().get("set-cookie"){
            Some(v)   => str::from_utf8(v.as_bytes()).unwrap(),
            _ => "Errored out, my dude. \nProbably too many concurrent sessions. 
            So...Something went wrong with logging out, maybe.",
        };
        s.push_str(cookies);
    Ok(())
}

pub async fn log_out(s: &String) -> Result<(), Box<dyn Error>>{
        println!("Going to log out now 👋.");
        use reqwest::{cookie::Jar, Url};
        let cookies = s.clone();
        let session_endpoint = String::from("https://qualysapi.qualys.com/api/2.0/fo/session/");
        let jar = Jar::default();
        jar.add_cookie_str(&cookies, &session_endpoint.parse::<Url>().unwrap());
        let jar = std::sync::Arc::new(jar);

        //create reqwest client
        let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(std::sync::Arc::clone(&jar))
        .build()?;
         
        client
        .post(Url::parse(&session_endpoint)?)
        .header("Accept", "text/plain")
        .header("X-Requested-With","Rust-Execv2")
        .query(&[("action","logout")])
        .timeout(Duration::from_secs(3))
        .send()
        .await?;

    // println!("I think we logged out...");
    Ok(())
}


pub async fn list_actions(s: &String, action: String,clientelle: Option<String>,
                          state: Option<String>,processed: Option<String>,scan_type: Option<String>,
                          target: Option<String>,afterdate: Option<String>,beforedate: Option<String>) -> Result<(), Box<dyn Error>>{
    use reqwest::{cookie::Jar, Url};
    let cookies = s.clone();
    
    let jar = Jar::default();
    let session_endpoint = String::from("https://qualysapi.qualys.com/api/2.0/fo/scan/");
    jar.add_cookie_str(&cookies, &session_endpoint.parse::<Url>().unwrap());
    let jar = std::sync::Arc::new(jar);

    let client = reqwest::Client::builder()
    .cookie_store(true)
    .cookie_provider(std::sync::Arc::clone(&jar))
    .build()?;
    
    let mut parameters = vec![("action",String::from("list"))];
    //Parameter builder

    match state{
        None => print!(""),
        Some(v) => parameters.push(("state", v)),
    }
    match processed{
        None => print!(""),
        Some(v) => { if (v == String::from("True")) || (v == String::from("true")){
                    parameters.push(("processed", String::from("1")))
                    } else {parameters.push(("processed", String::from("0")))}
                },
    }
    match scan_type{
        None => print!(""),
        Some(v) => parameters.push(("type", v)),
    }
    match target{
        None => print!(""),
        Some(v) => parameters.push(("target", v)),
    }
    match afterdate{
        None => print!(""),
        Some(v) => parameters.push(("launched_after_datetime", v)),
    }
    match beforedate{
        None => print!(""),
        Some(v) => parameters.push(("launched_before_datetime", v)),
    }


    xml::parse_list(client
            .post(Url::parse(&session_endpoint)?)
            .header("Accept", "text/plain")
            .header("X-Requested-With","Rust-Execv2")
            .query(&parameters)
            .timeout(Duration::from_secs(3))
            .send()
            .await?.text().await?,clientelle);

    
    Ok(())
}


fn get_hash(s: &mut String){
  let re: Regex = Regex::new(r"[a-f0-9]{32}").unwrap();
  let mut temp:String = String::new();
  
  for cap in re.captures(&s){
    temp.push_str(&cap[0]);}

  s.clear();
  s.push_str(&temp);
}


