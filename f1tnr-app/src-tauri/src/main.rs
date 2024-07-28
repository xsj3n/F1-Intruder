// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;
use tokio::{io::{AsyncBufReadExt, AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader}, net::TcpListener};
use ws::{RxArcMux, TxArcMux, WServer};
use std::{borrow::BorrowMut, io::{BufRead, Write}, sync::{mpsc::{Receiver, Sender}, Arc}, time::Duration};
use tokio::sync::Mutex;
use std::path::PathBuf;
use std::process::Command;
use futures::{io::Lines, stream::SplitSink, SinkExt, StreamExt};
use tauri::{api::file, Manager, State};
use tokio::fs::File;
use fs4::FileExt;

pub mod ws;

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

#[derive(Clone)]
struct PayloadBuffer(Arc<std::sync::Mutex<Vec<String>>>);
#[derive(Clone)]
struct WordlistFilePath(Arc<std::sync::Mutex<String>>);
#[derive(Clone)]
struct HttpBaseRequest(Arc<std::sync::Mutex<String>>);
#[derive(Clone)]
struct ThreadNumbers(Arc<std::sync::Mutex<String>>);


fn main()
{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        start_async_engine,
        check_for_new_http_data,
        refresh_datadir_cache,
        parse_burp_file,
        get_cached_filepath,
        set_cached_filepath,
        get_cached_http_request,
        set_cached_http_request,
        get_cached_thread_num,
        set_cached_thread_num,
        set_cached_payloads,
        get_cached_payloads,
        empty_cache_dir
        ])
    .setup(|app|
    {

        app.manage(Mutex::new(DataFileMemoBuffer::new()));

        app.manage(WordlistFilePath(Arc::new(std::sync::Mutex::new(String::new()))));
        app.manage(HttpBaseRequest(Arc::new(std::sync::Mutex::new(String::new()))));
        app.manage(ThreadNumbers(Arc::new(std::sync::Mutex::new(String::new()))));
        app.manage(PayloadBuffer(Arc::new(std::sync::Mutex::new(Vec::new()))));

        let app_handle_1 = app.handle();
        tauri::async_runtime::spawn( async move
        {
           
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            let server = TcpListener::bind("127.0.0.1:9005")
            .await.unwrap();

            println!("[*] TcpListender binded");

            'server_accept: loop
            {

                let (stream,_)   = server.accept().await.unwrap();
                println!("[*] Connected to GUI");
                let ws_stream = match tokio_tungstenite::accept_async(stream).await
                {
                    Ok(ws) => ws,
                    Err(e) => 
                    {
                        println!("Error connecting to the front-end WS client: {}", e);
                        return;
                    }
                };
        
                let (tx, mut rx) = ws_stream.split();
                match app_handle_1.try_state::<TxArcMux>()
                {
                    Some(state) => 
                    {
                        *state.lock().await = tx;
                    },
                    None => 
                    {
                        app_handle_1.manage(Arc::new(Mutex::new(tx)));
                        ()
                    }
                };

                
                let mut first_pass = true;
                let tx_state = app_handle_1.state::<TxArcMux>();
                
                loop
                {
                    if first_pass
                    {
                        tokio::time::interval(Duration::from_secs(1)).tick().await;
                        let mut tx_lck = tx_state.lock().await;
                        match tx_lck.send("ping".into()).await
                        {
                            Ok(_) => std::mem::drop(tx_lck),
                            Err(e) => 
                            {
                                println!("[!] Error: {}", e.to_string());
                                continue 'server_accept
                            },
                        }
    
                        first_pass = false;
                    }

                    
                    while let Some(msg) = rx.next().await
                    {
                        match msg
                        {
                            Ok(m) => 
                            {

                                if m.to_string() == "pong"
                                {
                                    interval.tick().await;
                                    let mut tx_lck = tx_state.lock().await;
                                    match tx_lck.send("ping".into()).await
                                    {
                                        Ok(_) => println!("[*] ws-heartbeat"),
                                        Err(e) =>
                                        {
                                            println!("[*] Error: {}", e.to_string());
                                            continue 'server_accept
                                        },
                                    };
                                } else
                                {

                                }
                            },
                            Err(e) => 
                            {
                                println!("[*] Error: {}", e.to_string());
                                continue 'server_accept

                            },
                        }
                    };
                    
                }
            }

            

        });

        
        let app_handle_2 = app.handle();
        tauri::async_runtime::spawn(async move
        {
            loop
            {
                app_handle_2.emit_all("data-poll", "").unwrap();
                tokio::time::sleep(tokio::time::Duration::from_millis(250)).await;     
                /* PROFILING 
                if let Some(u) = memory_stats::memory_stats()  
                {
                    println!("Virtual MEM usage: {} MB", u.virtual_mem / 1000000 );
                    println!("Physical MEM usage: {} MB", u.physical_mem / 1000000);
                } else {println!("Faield to get MEM usage..."); }
                */
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
    match std::fs::read_to_string("/tmp/f1_pslr/request.data")
    {
      Ok(s) => return s, 
      Err(e) => 
      {
        return e.to_string() 
      },
    };


}


#[tauri::command]
fn empty_cache_dir()
{
    let entries = match std::fs::read_dir("/tmp/f1_pslr/data/")
    {
        Ok(e) => e,
        Err(_) => return,
    };


    for p in entries
    {
        let dir_entry = p.unwrap();
        std::fs::remove_file(dir_entry.path()).unwrap();
    }

    return;
    
}

#[tauri::command]
fn start_async_engine(http_request: String, permutation_filepath: String, threads_num: String)
{

    println!("Front-end request:\n{}", &http_request);
    let _: std::process::Child = Command::new("../../async_net_engine/AsyncNetEngine/target/debug/AsyncNetEngine")
    .arg(http_request)
    .arg(permutation_filepath)
    .arg(threads_num)
    .spawn().unwrap();

    return

}


#[tauri::command]
async fn refresh_datadir_cache(datafilememo: State<'_, Mutex<DataFileMemoBuffer>>) -> Result<(), ()>
{
    let entries = std::fs::read_dir("/tmp/f1_pslr/data").unwrap();
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
async fn check_for_new_http_data(datafilememo: State<'_, Mutex<DataFileMemoBuffer>>, tx_state:  State<'_, TxArcMux>) -> Result<(), ()>
{
    let data_file_lock = datafilememo.inner();
    

    let mut data_file_metadata_buffer = data_file_lock.lock().await;
    if data_file_metadata_buffer.data_file_metadata.len() == 0 {return Err(())};
    let stream = tokio_stream::iter(data_file_metadata_buffer.data_file_metadata.iter_mut());
    tokio::pin!(stream);

    while let Some(fm) = stream.next().await 
    {
        let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(&fm.path).await.unwrap();

        let meta_d = file.metadata().await.unwrap();
        fm.len = meta_d.len();
        
        file.seek(std::io::SeekFrom::Current(
            fm.pos_r.try_into()
            .expect("u64 -> i64 conversion overflowed"
        ))).await.unwrap(); 
        // should switch to bufreader sometime 
        let mut buffer = Vec::<u8>::new();
        
        file.read_to_end(&mut buffer).await.unwrap();
        fm.pos_r = file.seek(std::io::SeekFrom::Current(0)).await.unwrap();

        
        let http_data_buffer = String::from_utf8_lossy(&buffer).to_string();
        let sorted_data = parse_http_data(http_data_buffer).await;
        for rr in sorted_data
        {
            if rr.0.full_request_string.len() == 0 || rr.1.full_response_string.len() == 0 {continue;}

            let http_data = HttpData{http_request: rr.0, http_response: rr.1}; 
            let js_obj = match serde_json::to_string(&http_data)
            {
                Ok(json_string) => json_string,
                Err(e) => format!("error: {}", e),
            };

            tx_state.lock().await
            .send(js_obj.into())
            .await.unwrap()
                
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
                let mut request_info_split = req_resp_half.split("=|"); // occassionally, this produces an invalid split, leading to a request being parsed wrong 
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
        Err(e) => 
        {
            let err_s = e.to_string();
            println!("Request parse failure - {}:\n{}", &err_s[..20], &response); 
            return Err(err_s);
        },
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



fn _run_shell_cmd<C: AsRef<str>>(cmd: C) -> std::io::Result<()>
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


fn ensure_tmp_dir()
{
    std::fs::create_dir_all("/tmp/f1_pslr/data")
    .unwrap_or(())
}


#[tauri::command]
fn set_cached_payloads(new_payload_sv: Vec<String>) -> Result<(), ()>
{
    ensure_tmp_dir();

    let mut file = std::fs::OpenOptions::new()
    .create(true)
    .truncate(true)
    .write(true)
    .open("/tmp/f1_pslr/plsr.dat")
    .unwrap();

    for payload in new_payload_sv
    {
        file.write_all((payload + "\n").as_bytes()).unwrap();
    }

    return Ok(())
}

#[tauri::command]
fn get_cached_payloads() -> Vec<String>
{

    let file = std::fs::OpenOptions::new()
    .create(false)
    .write(false)
    .read(true)
    .open("/tmp/f1_pslr/plsr.dat")
    .unwrap();

    let mut payload_sv: Vec<String> = Vec::new();
    let lines = std::io::BufReader::new(file).lines();
    for l in lines
    {
        payload_sv.push(l.unwrap());
    }
 

    return payload_sv;
}


#[tauri::command]
fn get_cached_filepath(filepath: State<'_, WordlistFilePath>) -> Result<String, String>
{
    let fp = filepath.0.lock().unwrap().clone();
    return Ok(fp);
}

#[tauri::command]
fn set_cached_filepath(new_filepath: String, filepath: State<'_, WordlistFilePath>) -> Result<(), ()>
{
    *filepath.0.lock().unwrap() = new_filepath;
    return Ok(())
}


#[tauri::command]
fn get_cached_http_request(request: State<'_, HttpBaseRequest>) -> Result<String, String>
{
    let r = request.0.lock().unwrap().clone();
    return Ok(r);
}

#[tauri::command]
fn set_cached_http_request(new_request: String, request: State<'_, HttpBaseRequest>) -> Result<(), ()>
{
    *request.0.lock().unwrap() = new_request;
    return Ok(())
}

#[tauri::command]
fn get_cached_thread_num(thread_num_s: State<'_, ThreadNumbers>) -> Result<String, String>
{
  
    let r = thread_num_s.0.lock().unwrap().clone();
    return Ok(r);
}

#[tauri::command]
fn set_cached_thread_num(new_thread_num_s: String, thread_num_s: State<'_, ThreadNumbers>) -> Result<(), ()>
{
    *thread_num_s.0.lock().unwrap() = new_thread_num_s;
    return Ok(())
}
