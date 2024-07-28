use std::{fs::File, io::Write};
use chrono::prelude::*;



pub enum LogType
{
    Meta,
    DataFile(u32)
}

pub fn form_log_string(request: &str, response: String, request_id: u32) -> String
{
    let mut log = String::new();
    let id_s = request_id.to_string();
    log.push_str("=R†="); //request delimiter for log file, for ease of parsing from tauri/nextjs
    log.push_str(&id_s);
    log.push_str("=|");
    log.push_str(&request);
    log.push_str("=RR†="); //response delimiter
    log.push_str(&response);
    log.push_str("=†=");
    return log;
}




pub fn log_f<S: AsRef<str>>(msg: S, log_type: LogType) -> ()
{
    let time: String = Local::now().to_string() + " ";

    match log_type
    {
        LogType::Meta => 
        {
            let mut final_str_to_log = time + msg.as_ref();
            final_str_to_log.push_str("\n");

            let mut file = open_log_file(0, true);
            
            match writeln!(&mut file, "{}", final_str_to_log) 
            {
                Ok(_) =>  return  (),
                Err(_) => return ()
            }

        },
        LogType::DataFile(id) =>
        {
            let mut file = open_log_file(id, false);

            match writeln!(&mut file, "{}", msg.as_ref()) 
            {
                Ok(_) =>  return  (),
                Err(_) => return ()
            } 
        },
    }





    

}

fn form_filepath_from_id(mut path_prefix: String, id: u32) -> String
{
    let id_s: String = id.to_string();
    if id == 0 {path_prefix.push_str("S")}

    path_prefix.push_str(&id_s);
    path_prefix.push_str(".data");

    return path_prefix;
}

fn open_log_file(id: u32, meta: bool) -> File
{

    let path: String = match meta
    {
        true => "/tmp/f1_pslr/test.log".into(),
        false => form_filepath_from_id("/tmp/f1_pslr/data/".into(), id),
    };


    return std::fs::OpenOptions::new()
    .read(false)
    .write(true)
    .create(true)
    .append(true)
    .open(path).unwrap()
}


