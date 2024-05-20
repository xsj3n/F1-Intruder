// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncSeekExt};
use std::sync::mpsc::{Receiver, Sender};
use tokio::sync::Mutex;
use std::path::PathBuf;
use std::process::Command;
use futures::StreamExt;
use tauri::{Manager, State};

#[derive(Clone, Serialize, Debug)]
struct HttpData
{
    pub http_request: HttpRequest,
    pub http_response: HttpResponse
}

#[derive(Clone, Serialize, Debug)]
struct HttpResponse
{
    pub status_code: u32,
    pub status_string: String,
    pub length: usize,
    pub full_response_string: String
}
#[derive(Clone, Serialize, Debug)]
struct HttpRequest
{
    pub method: String,
    pub path: String,
    pub version: u8,
    pub full_request_string: String,
    pub id: u32
}
impl HttpRequest
{
    pub fn new() -> HttpRequest
    {
        return HttpRequest
        {
            method: String::new(),
            path: String::new(),
            version: 0,
            full_request_string: String::new(),
            id: 0
        }
    }
}

impl HttpResponse
{
    pub fn new() -> HttpResponse
    {
        return HttpResponse
        {
            status_code: 0,
            status_string: String::new(),
            length: 0, 
            full_response_string: String::new()
        }
    }
}



struct DataFileMemoBuffer
{
    pub data_file_metadata: Vec<FileMemo>
}

impl DataFileMemoBuffer
{
    pub fn new() -> DataFileMemoBuffer
    {
        return DataFileMemoBuffer
        {
            data_file_metadata: Vec::new()
        }
    }


    pub fn contains(&self, pattern: &str) -> bool
    {
        for filememo in self.data_file_metadata.iter()
        {
            if filememo.path.to_string_lossy().to_string().contains(pattern)
            {
                return true
            }
        }
        
        return false
    }
}

struct FileMemo
{
    pub path: PathBuf,
    pub pos_r: u64,
    pub len: u64,
}

impl FileMemo
{
    pub fn new(path: PathBuf) -> FileMemo
    {
        return FileMemo { path: path, pos_r: 0, len: 0 }
    }

}

fn main()
{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        start_async_engine,
        check_for_new_http_data,
        refresh_datadir_cache,
        parse_burp_file,
        unlock_net_engine])
    .setup(|app|
    {

        app.manage(Mutex::new(DataFileMemoBuffer::new()));
        let (tx, rx): (Sender<HttpData>, Receiver<HttpData>) = std::sync::mpsc::channel();
        app.manage(Mutex::new(tx));

        let app_handle = app.app_handle();
        tauri::async_runtime::spawn(async move
        {
            loop
            {
                
                match rx.recv()
                {
                    Ok(http_data) =>
                    {
                        app_handle.emit_all("http-data", http_data).unwrap();
                    },
                    Err(_) => println!("rx error!")
                };    
            }
        });

        
        let app_handle_2 = app.handle();
        tauri::async_runtime::spawn(async move
        {
            loop
            {
                app_handle_2.emit_all("data-poll", "").unwrap();
                tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;       
            }
        });


        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn parse_burp_file() -> String
{
    match std::fs::read_to_string("/tmp/request.data")
    {
      Ok(s) => return s, 
      Err(_) => 
      {
        return String::new();
      },
    };


}


#[tauri::command]
fn start_async_engine(http_request: String, permutation_filepath: String)
{
    let _: std::process::Child = Command::new("../../async_net_engine/AsyncNetEngine/target/debug/AsyncNetEngine")
    .arg(http_request)
    .arg(permutation_filepath)
    .spawn().unwrap();

    return

}

#[tauri::command]
fn unlock_net_engine()
{ 
    std::fs::remove_file("../../async_net_engine/lock").unwrap_or(());
    for dir in std::fs::read_dir("../../async_net_engine/data").unwrap()
    {
        let path = dir.unwrap().path();
        std::fs::remove_file(path).unwrap();
    }
}

#[tauri::command]
async fn refresh_datadir_cache(datafilememo: State<'_, Mutex<DataFileMemoBuffer>>) -> Result<(), ()>
{
    let entries = std::fs::read_dir("../../async_net_engine/data").unwrap();
    let mut data_file_lock = datafilememo.lock().await;
    for p in entries
    {
        let path = p.unwrap().path();
        if data_file_lock.contains(path.to_str().to_owned().unwrap()) { continue; }
        println!("Monitoring: {}", path.to_str().unwrap());
        data_file_lock.data_file_metadata.push(FileMemo::new(path));
    }

    return Ok(())
}


#[tauri::command]
async fn check_for_new_http_data(datafilememo: State<'_, Mutex<DataFileMemoBuffer>>, tx_half: State<'_, Mutex<Sender<HttpData>>>) -> Result<(), ()>
{
    let data_file_lock = datafilememo.inner();
    

    let mut data_file_metadata_buffer = data_file_lock.lock().await;
    if data_file_metadata_buffer.data_file_metadata.len() == 0 {println!("Failed to check for new http data, file path memo buffer empty")};
    let stream = tokio_stream::iter(data_file_metadata_buffer.data_file_metadata.iter_mut());
    tokio::pin!(stream);

    while let Some(fm) = stream.next().await 
    {
        let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&fm.path).await.unwrap();

        let file_fresh_metadta = file.metadata().await.unwrap();
        fm.len = file_fresh_metadta.len();

        let mut buffer = Vec::<u8>::new();
        file.seek(std::io::SeekFrom::Start(fm.pos_r)).await.unwrap();
        file.read_to_end(&mut buffer).await.unwrap();
        fm.pos_r = file.stream_position().await.unwrap();

        //println!("new data in file {} in  pos {}:\n {}",fm.path.to_str().unwrap() , fm.pos_r.to_string(), String::from_utf8_lossy(&buffer));
        let http_data_buffer = String::from_utf8_lossy(&buffer).to_string();
        let sorted_data = parse_http_data(http_data_buffer).await;
        for rr in sorted_data
        {
          if rr.0.full_request_string.len() == 0 || rr.1.full_response_string.len() == 0 {continue;}
          tx_half.lock().await.send(HttpData{http_request: rr.0, http_response: rr.1}).unwrap();

        }
         
    };
    
    


    return Ok(());
}

async fn parse_http_data(buffer: String) -> Vec<(HttpRequest, HttpResponse)>
{
    let mut request_response_v: Vec<(HttpRequest, HttpResponse)> = Vec::new();
    
    for segment in buffer.split("=†=")
    {
        let mut req_resp_ts: (HttpRequest, HttpResponse) = (HttpRequest::new(), HttpResponse::new());
        for req_resp_half in segment.split("=RR†=")
        {
            if req_resp_half.contains("=R†=") 
            {
                let mut request_info_split = req_resp_half.split("=|");
                let req_id_side = request_info_split.next().unwrap();
                let request_id = match strip_r_dagger_delim(req_id_side.trim())
                .parse::<u32>()
                {
                    Ok(id) => id,
                    Err(_) =>
                    {
                        println!("Failed to parse to u32: {}", req_id_side);
                        0
                    } 
                };

                let request_string = request_info_split.next().unwrap().to_string();
                let parsed_rq = parse_http_rq(request_string, request_id).await.unwrap();
                
                req_resp_ts.0 = parsed_rq;
                continue;
            } else if req_resp_half.trim().len() == 0 {continue;}
            
            req_resp_ts.1 = parse_http_rsp(req_resp_half.to_string()).await.unwrap();
        }
        request_response_v.push(req_resp_ts);
    }
    return request_response_v;
}

async fn parse_http_rsp(response: String) -> Result<HttpResponse, String>
{
    let mut header_buffer = [httparse::EMPTY_HEADER; 64];
    let mut response_buffer = httparse::Response::new(&mut header_buffer);
    match response_buffer.parse(response.as_bytes())
    {
        Ok(_) => 
        {
            return Ok(HttpResponse
            {
                status_code: response_buffer.code.unwrap() as u32,
                status_string: response_buffer.reason.unwrap().to_string(),
                length: response.len(),
                full_response_string: response
            })
        },
        Err(e) => return Err(e.to_string()),
    }
}

async fn parse_http_rq(request: String, request_id: u32) -> Result<HttpRequest, String>
{
    let mut header_buffer = [httparse::EMPTY_HEADER; 64];
    let mut request_buffer = httparse::Request::new(&mut header_buffer);
    match request_buffer.parse(request.as_bytes())
    {
        Ok(_) => 
        {
            return Ok(HttpRequest
            {
                method: request_buffer.method.unwrap().to_string(),
                path: request_buffer.path.unwrap().to_string(),
                version: request_buffer.version.unwrap(),
                full_request_string: request,
                id: request_id
            });
        },
        Err(e) => return Err(e.to_string())
    }
}

fn strip_r_dagger_delim(unstripped_id_s: &str) -> String
{
    let mut unstripped_chars = unstripped_id_s.chars();
    for _ in 0..4
    {
        unstripped_chars.next().unwrap();
    }

    return unstripped_chars.collect::<String>();
}



fn run_shell_cmd<C: AsRef<str>>(cmd: C) -> std::io::Result<()>
{
    let mut cmd_parts = cmd.as_ref().split(" ");
    let program = match cmd_parts.next()
    {
        Some(s) => s,
        None => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Failed to run shell commands due to failed program designation.")),
    };
    
    let mut cmd = Command::new(program);
    for p in cmd_parts
    {
        cmd.arg(p);
    };
   

   return Ok(());

        
    
}

