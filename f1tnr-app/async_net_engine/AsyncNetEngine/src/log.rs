use std::io::Write;
use chrono::prelude::*;

pub fn dbg_log_progress(msg: &str) -> ()
{
    let time: String = Local::now().to_string() + " ";

    let fpdir = "../test.log";
    let opts = std::fs::OpenOptions::new()
        .read(false).write(true).create(true).append(true).open(fpdir);

    if opts.is_err() {return ();}

    let mut f = opts.unwrap();
    if f.metadata().unwrap().len() > 0xf4240
    {
        f.set_len(0).unwrap_or(());
    }


    match writeln!(&mut f, "{}", time + msg + "\n") 
    {
        Ok(_) =>  return  (),
        Err(_) => return ()
    }
    

}
