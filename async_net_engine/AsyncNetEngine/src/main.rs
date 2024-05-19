use std::{env::{self, args}, io, process::Command};

use async_net_spx::configure_workload;
use parse_util::synth_request_groups;
use tokio::task::spawn_blocking;

use crate::parse_util::add_clrf_to_arguement_string;

pub mod async_net_spx;
pub mod interface_structs;
pub mod parse_util;
pub mod log;

/*
This binary is intended to be triggered by react's useEffect hook, and as a consequence, it needs 
to be fault tolerant to being triggred multiple times, so fine. We'll have a lock system based on 

*/

#[tokio::main]
async fn main() // params will be the orginal request, and the permutations
{

    let pwd = get_pwd();
    _ = check_lock(pwd);

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

    if &args[2] == "--cli"
    {
        todo!()//signal that paths will be different 
    }

    let rp = synth_request_groups(request, permutations);
    let rp_v = configure_workload(rp, 12);
    
    spawn_blocking(move || async
    {
        async_net_spx::start_taskmaster("httpbin.org".to_string(), rp_v).await;
    }).await.unwrap().await;

    
    
    //_ = unlock(pwd);

    return 
}


fn run_shell_cmd<C: AsRef<str>>(cmd: C) -> io::Result<String>
{
    let mut cmd_parts = cmd.as_ref().split(" ");
    let program = match cmd_parts.next()
    {
        Some(s) => s,
        None => return Err(io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to run shell commands due to failed program designation.")),
    };
    
    let mut cmd = Command::new(program);
    for p in cmd_parts
    {
        cmd.arg(p);
    };
   
   let output = cmd.output()?;
   let out = String::from_utf8_lossy(&output.stdout).to_string();
   let err = String::from_utf8_lossy(&output.stderr).to_string();

   if err.len() != 0 { return Ok(err)};

   return Ok(out);

        
    
}

fn is_locked(pwd: Pwd) -> bool 
{
    match pwd
    {
        Pwd::Gui => 
        {
            match std::fs::File::open("../../async_net_engine/lock")
            {
                Ok(_) => return true,
                Err(_) => return false,
            }
        },
        Pwd::Cli => 
        {
            match std::fs::File::open("../lock")
            {
                Ok(_) => return true,
                Err(_) => return false,
            }
        },
    }
}

fn _unlock(pwd: Pwd) -> bool
{
    let mut cmd_prefix = "rm ".to_string();
    match pwd
    {
        Pwd::Gui => cmd_prefix.push_str("../../async_net_engine/lock"),
        Pwd::Cli => cmd_prefix.push_str("../lock")
    };


    match run_shell_cmd(cmd_prefix)
    {
        Ok(_) => return true,
        Err(_) => return false,
    }
}

fn check_lock(pwd: Pwd) 
{
    match is_locked(pwd)
    {
        true =>  
        {
            println!("[!] Lock file still present. Not running again.");
            std::process::exit(2);
        },
        false => lock(pwd)
    };
}

fn lock(pwd: Pwd)
{
    match pwd
    {
        Pwd::Gui => run_shell_cmd("touch ../../async_net_engine/lock").unwrap(),
        Pwd::Cli => run_shell_cmd("touch ../lock").unwrap(),
    };
}

fn get_pwd() -> Pwd
{
    let pwd_string = run_shell_cmd("pwd").unwrap();    
    if pwd_string.contains("tauri") {return Pwd::Gui};
    return Pwd::Cli;    
}

#[derive(Clone, Copy)]
pub enum Pwd
{
    Gui,
    Cli
}