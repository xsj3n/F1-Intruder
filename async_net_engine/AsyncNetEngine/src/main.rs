use std::{env, process::Command};

use async_net_spx::configure_workload;
use interface_structs::RequestandPermutation;
use parse_util::synth_request_groups;
use tokio::task::spawn_blocking;

use crate::parse_util::add_clrf_to_arguement_string;

pub mod async_net_spx;
pub mod interface_structs;
pub mod parse_util;
pub mod log;

#[tokio::main]
async fn main() -> () // params will be the orginal request, and the permutations 
{

    let args: Vec<String> = env::args().collect();

    if args.len() > 3
    {
        println!("Not enough arguments. <HttpRequest> <FilePathToPermutations> are the required arguments.");
        return;
    };

    let request = add_clrf_to_arguement_string(args[1].clone());
    println!("[+] Request:\n{}", &request);


    let permutations = match parse_util::read_permutation_lines(&args[2])
    {
        Ok(p) => p,
        Err(e) => 
        {
            println!("Error reading from permutation file: {}", e.to_string());
            return;
        },
    };

    let rp = synth_request_groups(request, permutations);
    let rp_v = configure_workload(rp, 12);
    
    spawn_blocking(move || async
    {
        async_net_spx::start_taskmaster("httpbin.org".to_string(), rp_v).await;
    }).await.unwrap().await;

    let s = Command::new("bash")
    .arg("-c")
    .arg("pwd")
    .output().unwrap();
    
    println!("{}", String::from_utf8_lossy(s.stdout.as_ref()));

    return 
}
