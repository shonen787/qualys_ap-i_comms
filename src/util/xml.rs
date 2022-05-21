use xml::{EventReader};
use xml::reader::XmlEvent;
use std::io::{stdin,stdout,Write};


#[derive(Copy, Clone)]
enum Pusher{
    Title,
    Type,
    User,
    State,
    Target,
    Duration,
    DateTIME,
    Neither,
}

struct Scans{
    title: String,
    scan_type: String,
    user: String,
    stat: String,
    target: String,
    duration: String,
    launch_dt: String,

}

pub fn parse_list(s: String){
    let mut collected: Vec<Scans> = Vec::new();
    let mut pusher: Pusher = Pusher::Neither;
    
    let mut tempduration = String::new();
    let mut tempuser = String::new();
    let mut temptarget = String::new();
    let mut temptitle = String::new(); 
    let mut scant = String::new(); 
    let mut scansta = String::new(); 
    let mut tempdate = String::new();

    let parse = EventReader::from_str(&s);
    for e in parse{
        match e{
            Ok(XmlEvent::StartElement{name,..})=>{

                match name.local_name.as_str() {
                    "DURATION" => pusher = Pusher::Duration,
                    "TITLE" => pusher = Pusher::Title,
                    "TYPE" => pusher = Pusher::Type,
                    "USER_LOGIN" =>pusher = Pusher::User,
                    "STATE" => pusher = Pusher::State,
                    "TARGET" => pusher = Pusher::Target,
                    "LAUNCH_DATETIME" => pusher=Pusher::DateTIME,
                    _ => pusher = Pusher::Neither,
                };
            },
            Ok(XmlEvent::CData(text)) =>{
                match pusher {
                    Pusher::Title => {temptitle = text.clone()},
                    Pusher::Target => {temptarget = text.clone()},
                    _ => {},
                };
            },
            Ok(XmlEvent::Characters(text)) =>{
            match pusher {
                Pusher::Duration => {tempduration = text.clone()},      
                Pusher::State => {scansta = text.clone()},          
                Pusher::Type => {scant = text.clone()},
                Pusher::User => {tempuser = text.clone()},
                Pusher::DateTIME => {tempdate = text.clone()}
                _ => {},
            };},
            Ok(XmlEvent::EndElement{name}) =>{
                if name.local_name == "SCAN"{
                    collected.push(Scans{
                        title: temptitle.to_owned(),
                        scan_type: scant.to_owned(),
                        user: tempuser.to_owned(),
                        stat: scansta.to_owned(),
                        target: temptarget.to_owned(),
                        duration: tempduration.to_owned(),
                        launch_dt: tempdate.to_owned(),
                        });
                        
                        tempduration.clear();
                        tempuser.clear();
                        temptarget.clear();
                        temptitle.clear();
                        scant.clear();
                        scansta.clear();
                }

            },
            Err(e) =>{println!("Erro: {}", e)},
            _=>{}
        }
    }

    collected.sort_by_key(|d| d.title.clone());

    print_tabled(collected);
    


}


fn print_tabled(collected: Vec<Scans>){
    let mut client: String =String::new();
  

    for i in collected{
        println!("Scan Title:\n\t{}\nLaunch DateTime:\n\t{}\nCreated by:\n\t{}\nScan Status:\n\t{}\nScan Type:\n\t{}\nScan Targets:\n\t{}\nScan Duration:\n\t{}", i.title, i.launch_dt,i.user,i.stat, i.scan_type, i.target,i.duration);
        println!("\n################################");}
}


 fn user_input(s: &mut String) -> String{
    print!("Which client are you looking for?\nYou don't need to input it exactly. It's a little fuzzy..\n");
    let _=stdout().flush();
    stdin().read_line(s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s.to_string()
}
