use std::io::Write;
use chrono::prelude::*;

pub enum LogType
{
    Meta,
    DataFile(u32)
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

            let opts = std::fs::OpenOptions::new()
            .read(false).write(true).create(true).append(true).open("../test.log");

            if opts.is_err() {return ();}

            
            match writeln!(&mut opts.unwrap(), "{}", final_str_to_log) 
            {
                Ok(_) =>  return  (),
                Err(_) => return ()
            }

        },
        LogType::DataFile(id) =>
        {
            let mut final_str_to_log = msg.as_ref().to_string();
            final_str_to_log.push_str("\n");

            let filepath = form_filepath_from_id(id);

            let opts = std::fs::OpenOptions::new()
            .read(false).write(true).create(true).append(true).open(filepath);

            if opts.is_err() {return ();}

            match writeln!(&mut opts.unwrap(), "{}", final_str_to_log) 
            {
                Ok(_) =>  return  (),
                Err(_) => return ()
            } 
        },
    }





    

}

fn form_filepath_from_id(id: u32) -> String
{
    let mut filepath = "../".to_string();
    filepath.push_str(id.to_string().as_ref());
    filepath.push_str(".data");

    return filepath;
}