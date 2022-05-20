use reqwest;
use std::error::Error;
use std::time::Duration;
use url::Url;
use std::{str,env};
use regex::Regex;

pub async fn log_in(s: &mut String, user: String, pass: String) -> Result<(), Box<dyn Error>>{
    println!("Logging in");
   
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
            So...Something went wrong with the logging out, maybe.",
        };
        s.push_str(cookies);
    Ok(())
}

pub async fn log_out(s: &String) -> Result<(), Box<dyn Error>>{
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

    println!("I think we logged out...");
    //println!("Here's some info:\n{}",doge.status());
    Ok(())
}


pub async fn scan_actions(s: &String, action: String) -> Result<(), Box<dyn Error>>{
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
    println!("{}", s);

    if action.as_str() == String::from("list"){
        println!("{:#?}", client
            .post(Url::parse(&session_endpoint)?)
            .header("Accept", "text/plain")
            .header("X-Requested-With","Rust-Execv2")
            .query(&[("action","list")])
            .timeout(Duration::from_secs(3))
            .send()
            .await?.text().await?);
    }



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


