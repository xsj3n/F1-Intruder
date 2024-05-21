use std::{env::{self, args}, fs::File, io::{self, Read}, process::Command};

use async_net_spx::configure_workload;
use parse_util::synth_request_groups;
use tokio::task::spawn_blocking;
use fs4::FileExt;

use crate::parse_util::add_clrf_to_arguement_string;

pub mod async_net_spx;
pub mod interface_structs;
pub mod parse_util;
pub mod log;



#[tokio::main]
async fn main() // params will be the orginal request, and the permutations
{
    let file_lock = chk_lock();

    let args: Vec<String> = env::args().collect();

    if args.len() > 4
    {
        println!("Not enough arguments. <HttpRequest> <FilePathToPermutations> <# of threads> are the required arguments.");
        return;
    };

    

    let mut request = args[1].clone().lines().map(|l| l.to_string() + "\r\n") // lines() strips clrf
    .filter(|l| !l.contains("Accept-Encoding:"))
    .collect::<String>()
    .trim().to_string();
    request.push_str("\r\n\r\n");
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
        todo!() //signal that paths will be different 
    }
    
    let thread_number = match args[3].parse::<u32>()
    {
        Ok(num) => num,
        Err(_) => 10
    };
    println!("Threads: {}", thread_number);

    let request_s_2 = request.clone();
    let rp = synth_request_groups(request, permutations);
    let total_req_num = rp.permutation.len();
    let rp_v = configure_workload(rp, thread_number);
    

    let now = std::time::Instant::now();
    spawn_blocking(move || async
    {
        async_net_spx::start_taskmaster(parse_util::parse_hostname(request_s_2), rp_v).await;
    }).await.unwrap().await;
    println!("Finished all {} requests in {} seconds", total_req_num, now.elapsed().as_secs());
    file_lock.unlock().unwrap();

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

fn chk_lock() -> File
{
    match std::fs::File::open("/tmp/lock")
    {
        Ok(f) => 
        {
            match f.try_lock_exclusive()
            {
                Ok(_) => 
                {
                    println!("[*] Lock acquired...");
                    return f;
                },
                Err(_) => 
                {
                    println!("[!] Locked! Exiting...");
                    std::process::exit(2);
                },
            }
        },
        Err(_) => 
        {
            println!("[!] Locked! Exiting...");
            std::process::exit(2); 
        },
    }
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