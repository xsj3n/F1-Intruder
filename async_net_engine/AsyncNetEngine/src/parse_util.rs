use std::io::{BufRead, BufReader};
use std::{io, num::IntErrorKind};
use std::fs::File;
use crate::{interface_structs::RequestandPermutation, log::{log_f, LogType}};

#[derive(Debug)]
pub struct CacheReadError
{
    pub details: String
}

impl CacheReadError
{
    pub fn new(msg: &str) -> CacheReadError
    {
        CacheReadError { details: msg.to_string()}
    }
}

pub struct URICOMPONENTS
{
    pub scheme: String,
    pub host: String,
    pub port: Option<u32>,
    pub path: String,
    pub query: Option<String>,
}
/* 
pub fn parse_uri(full_uri: String) -> URICOMPONENTS
{
    let uri_comps = Url::parse(&full_uri).unwrap();
    
    return URICOMPONENTS
    {
        scheme: uri_comps.scheme().to_string(),
        host: uri_comps.host().unwrap().to_string(),
        port:  match uri_comps.port() 
        {
            Some(p) => Some(p as u32),
            None => None
            
        },
        path: uri_comps.path().to_string(),
        query: match uri_comps.query() 
        {
            Some(q) => Some(q.to_string()),
            None => None
        }
    };

}
*/

pub fn parse_host_from_cache_data(request_string: &str) -> Result<String, CacheReadError>
{
    let mut host = String::new();
    let lines = request_string.split("\r\n");
    
    for line in lines 
    {
        if line.contains("Host:")
        {
            host = line.replace("Host: ", "")
            .replace("\r\n", "");
        
        }
    }

    let log_s = "[+] Host parsed: ".to_string() + &host;
    log_f(log_s, LogType::Meta);

    if host.is_empty() == true { return Err(CacheReadError::new("[!] Unable to parse host from the request in request cache")); }
    return Ok(host);
}


pub fn parse_burp_file() -> String
{
    log_f("[+] parse_burp_file started", LogType::Meta);
    let req_byte_string = match std::fs::read_to_string("/Users/xis31/tmp/req_cache.dat")
    {
      Ok(s) => s, 
      Err(_) => 
      {
        log_f("[!] Unable to read cache file", LogType::Meta);
        return String::new();
      },
    };

    let req_byte_string_iterator = req_byte_string.split("\n");
    let mut bytes: Vec<u8> = Vec::new();

    for strings in req_byte_string_iterator
    {
        match strings.parse::<u8>()  
        {
            Ok(i) => bytes.push(i),
            Err(e) => 
            {
                if e.kind() == &IntErrorKind::Empty 
                {
                    println!("[+] Reached end of Burp Suite request cache");
                }
            }
        };
    }
    

    let parsed_string = String::from_utf8_lossy(&bytes)
        .to_string();

    println!("[+] Request parsed from BurpSuite request cache:");
    print!("{}", parsed_string);
    return parsed_string;

}

fn permutate_request(perm_src: &str, perm_mod: &str) -> Option<String>
{
    
    let n_index = perm_src.find("†")?;
    let n_e_index = perm_src.find("‡")?;
    
    let mut ns = perm_src.to_string();
    ns.replace_range(n_index..n_e_index + 3, perm_mod);
    return Some(ns);
    

}

pub fn synth_request_groups(http_request: String, permuations_v: Vec<String>) -> RequestandPermutation
{
    let mut rp: RequestandPermutation = RequestandPermutation::new();

    for permutation in permuations_v
    {
        rp.request.push(permutate_request(&http_request, &permutation).unwrap());
        rp.permutation.push(permutation);
    };

    return rp;
}

pub fn read_permutation_lines(filepath: &str) -> io::Result<Vec<String>>
{
    let file = File::open(filepath)?;

    let permutation_lines: Vec<String> = BufReader::new(file).lines()
    .map(|l| l.unwrap() )
    .collect();

    return Ok(permutation_lines);
}

pub fn add_clrf_to_arguement_string(arg_1: String) -> String
{
    return arg_1.replace(r#"\r\n"#, "\r\n");
}