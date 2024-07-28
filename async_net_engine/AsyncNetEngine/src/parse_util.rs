use std::io::{self, BufRead, BufReader, Read};
use std::fs::File;
use crate::interface_structs::HttpRequest;
use crate::interface_structs::RequestandPermutation;

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
pub fn parse_hostname(request: String) -> String
{
    let hostname = request.split("\r\n")
    .nth(1).unwrap()
    .split("Host: ").nth(1).unwrap()
    .to_string();

    return hostname
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
    let permutation_v_len = permuations_v.len();

    let mut id_c: usize = 0;
    for permutation in permuations_v
    {
        let http_request = HttpRequest::new(permutate_request(&http_request, &permutation).unwrap(), id_c as u32);
        rp.request.push(http_request);
        rp.permutation.push(permutation);
        id_c += 1;
    };

    assert!(permutation_v_len == id_c,
         "More IDs then there are requests. ID count: {}, Permutation count: {}",
          id_c, permutation_v_len); 

    return rp;
}

pub fn read_permutation_lines(filepath: &str) -> io::Result<Vec<String>>
{
    let mut file = std::fs::OpenOptions::new()
    .create(false)
    .read(true)
    .write(false)
    .open(filepath)?;

    let mut buffer: String = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut permutation_lines: Vec<String> = buffer
    .split("\n")
    .map(|line| line.to_string())
    .collect();

    permutation_lines.pop(); // pop off the \n entry

    println!("[*] PSLR filepath: {}", filepath);
    println!("{:#?}", &permutation_lines);
    return Ok(permutation_lines);
}

pub fn add_clrf_to_arguement_string(arg_1: String) -> String
{
    return arg_1.replace(r#"\r\n"#, "\r\n");
}
