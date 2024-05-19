use std::{fs::File, io::Write, sync::Arc};
use chrono::prelude::*;

use crate::Pwd;

pub enum LogType
{
    Meta,
    DataFile(u32)
}

pub fn log_f<S: AsRef<str>>(msg: S, log_type: LogType, pwd: std::sync::Arc<Pwd>) -> ()
{
    let time: String = Local::now().to_string() + " ";

    match log_type
    {
        LogType::Meta => 
        {
            let mut final_str_to_log = time + msg.as_ref();
            final_str_to_log.push_str("\n");

            let mut file = open_log_file(pwd, 0, true);
            
            match writeln!(&mut file, "{}", final_str_to_log) 
            {
                Ok(_) =>  return  (),
                Err(_) => return ()
            }

        },
        LogType::DataFile(id) =>
        {
            let mut file = open_log_file(pwd, id, false);

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

    path_prefix.push_str("data/");
    if id == 0 {path_prefix.push_str("S")}

    path_prefix.push_str(&id_s);
    path_prefix.push_str(".data");

    return path_prefix;
}

fn open_log_file(pwd: Arc<Pwd>, id: u32, meta: bool) -> File
{
    if meta == true
    {
        match *pwd
        {
            Pwd::Gui => return std::fs::OpenOptions::new()
            .read(false).write(true).create(true).append(true)
            .open("../../async_net_engine/test.log").unwrap(),

            Pwd::Cli => return std::fs::OpenOptions::new()
            .read(false).write(true).create(true).append(true)
            .open("../test.log" ).unwrap(),
        }
    }


    match *pwd
    {
        Pwd::Gui => return std::fs::OpenOptions::new()
                    .read(false).write(true).create(true).append(true)
                    .open(form_filepath_from_id("../../async_net_engine/".into(), id) ).unwrap(),

        Pwd::Cli => return std::fs::OpenOptions::new()
                    .read(false).write(true).create(true).append(true)
                    .open(form_filepath_from_id("../".into(), id)).unwrap()
    };
}