// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



use futures::{SinkExt, StreamExt};
use interface_structs::{HttpResponseDataC, RequestandPermutation};
use log::dbg_log_progress;
use parse_util::parse_host_from_cache_data;
use core::time;
use std::cell::RefCell;
use std::fs::read_to_string;
use std::num::IntErrorKind;
use std::sync::Arc;
use std::thread::sleep;

use tauri::http::ResponseBuilder;
use tauri::{window, Manager, Window};

use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};

//use crate::net_spx::*;
use crate::parse_util::__permutate_request__;
use crate::interface_structs::*;
use tokio::net::{TcpListener, TcpSocket};

pub mod async_net_spx;
pub mod interface_structs;
pub mod log;
pub mod parse_util;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_ipc_server])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  }


thread_local!{ static DOMAIN_BUF: Arc<RefCell<String>> = Arc::new(RefCell::new(String::new())); }
//pub type Callback = Option< extern "C" fn(hrdc: HttpResponseDataC, permuation: String, row_num: u16) -> bool>;

#[tauri::command]
async fn start_ipc_server()
{
    let socket = TcpSocket::new_v4().unwrap();
    socket.set_keepalive(true).unwrap();
    socket.set_reuseaddr(true).unwrap();
    socket.set_reuseport(true).unwrap();
    socket.bind("127.0.0.1:3001".parse().unwrap()).unwrap();

    let listener = socket.listen(1).unwrap();
    let (serversock,_) = listener.accept().await.unwrap();
        
    let cb = |req: &Request, mut response: Response|
        {
            println!("WS Handshake");
            Ok(response)
        };

    let websocket = accept_hdr_async(serversock,cb)
    .await.unwrap();

    let (mut tx, mut rx) = websocket.split();
    

   
    let mut first_loop = true;
    loop {
        if first_loop == false
        {
            sleep(time::Duration::from_secs(10));
            
        } else 
        {
            first_loop = false;
        }

        tx.send("PING".into()).await.unwrap();
        
        while let Some(msg) = rx.next().await {
            match msg.unwrap().into_text().unwrap().as_str() {
                "PONG" => tx.send("PING".into()).await.unwrap(),
                _ => ()
            };
        }
    }
}


#[tauri::command]
fn readfile_lines(dirstr: String) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    let lines = read_to_string(dirstr).unwrap()
    .lines().map(|l| result.push(l.into()));
    
    return result;
}

#[tauri::command]
fn test(window: Window) -> ()
{
    
}

#[tauri::command]
fn send_com_async(request_buffer: Vec<String>, permutation_buffer: Vec<String>) -> ()
{
    let request_permutation_buffer = RequestandPermutation
    {
        request: request_buffer,
        permutation: permutation_buffer
    };

    if request_permutation_buffer.request.len() == 0
    {
        //return Err(PyValueError::new_err("Permuation buffer length empty"));
    }
    

    
    let tk_rt = tokio::runtime::Runtime::new().unwrap();
    tk_rt.block_on(async move 
    {
        let tsk_d_str = DOMAIN_BUF.with(|d: &Arc<RefCell<String>>| { d.borrow_mut().clone() } );
    
        async_net_spx::start_taskmaster(tsk_d_str, request_permutation_buffer, 15).await;
    });
    
    /* 
    pyo3_asyncio::tokio::future_into_py(py, async move 
    {
        let tsk_d_str = DOMAIN_BUF.with(|d: &Arc<RefCell<String>>| { d.borrow_mut().clone() } );
        async_net_spx::start_taskmaster(tsk_d_str, request_permutation_buffer, 15, cb).await;
        prepare_freethreaded_python();
        return Ok(());
    }).unwrap();
    */
    
    return; 
  
}


#[tauri::command]
fn parse_burp_file_export() -> String
{
    dbg_log_progress("[+] parse_burp_file started");
    let req_byte_string = match std::fs::read_to_string("/Users/xis31/tmp/req_cache.dat")
    {
      Ok(s) => s, 
      Err(_) => 
      {
        dbg_log_progress("[!] Unable to read cache file");
        return String::new();
      },
    };

    let req_byte_string_iterator = req_byte_string.split("\n");
    let mut bytes: Vec<u8> = Vec::new();

    for strings in req_byte_string_iterator
    {
        match strings.parse::<u8>()  
        {
            Ok(i) => bytes.push(i),
            Err(e) => 
            {
                if e.kind() == &IntErrorKind::Empty 
                {
                    println!("[+] Reached end of Burp Suite request cache");
                }
            }
        };
    }

    let parsed_string = String::from_utf8_lossy(&bytes)
    .to_string();

    DOMAIN_BUF.with(|d: &Arc<RefCell<String>>| 
        {
            let inp_str = parse_host_from_cache_data(&parsed_string).unwrap();
            d.borrow_mut().push_str(&inp_str);
        });


    println!("[+] Request parsed from BurpSuite request cache:");
    print!("{}", parsed_string);
    return parsed_string;
}
    



impl HttpResponseDataKeepAliveC
{
    pub fn new(hrd_v: Vec<HttpResponseDataC>, len: usize, empty: bool) -> HttpResponseDataKeepAliveC
    {

        let mut r = HttpResponseDataKeepAliveC 
        {
            http_response_data_c: Vec::new().into(),
            len: len
            
        };
        
        if empty == false
        {
            r.http_response_data_c = hrd_v.try_into()
                .unwrap_or(Vec::new().into());
        }

        return r;
    }   
}

impl HttpHeadersC
{
    fn new() -> HttpHeadersC
    {
        let empt_v  = vec![String::new(); 64];
        let empt_v2 = vec![String::new(); 64];

        return HttpHeadersC
        {
            header: empt_v.try_into().unwrap(),
            value: empt_v2.try_into().unwrap(),
            init: false,
       };

    }

}



impl HttpResponseDataC
{
    pub fn new(response_tp: (Option<httparse::Response>, Option<String>), bytes_from_server: usize, full_response_string: String) -> HttpResponseDataC
    {

      
       if response_tp.0.is_none() && response_tp.1.is_none()
       {
            dbg_log_progress("[!] Failed to parse into HTTPResponseDataC, no response or body found...");
            return HttpResponseDataC 
            {
                headers:              HttpHeadersC::new(),
                full_response:        String::new(),
                body:                 String::new(),  
                status_code:          0,
                total_response_bytes: 0
            };
       }
 

        let r = ResponseFFITransformer(response_tp.0.unwrap());
        let code = r.0.code.unwrap_or(0);

        let mut http_response_data = HttpResponseDataC 
        {
            headers:              r.transform(),
            full_response:        full_response_string,
            body:                 String::new(),  
            status_code:          code,
            total_response_bytes: bytes_from_server as u32
        };

        match response_tp.1
        {
            Some(b) => http_response_data.body = b,
            None => ()
        }

        return http_response_data;
    }

}



pub struct ResponseFFITransformer<'h, 'b>(httparse::Response<'h, 'b>);
impl<'h, 'b> ResponseFFITransformer<'h, 'b>
{
    // ill check the results on cstring creations if it causes problems 
    fn transform(self) -> HttpHeadersC
    {   
        let mut http_struct = HttpHeadersC::new();
        
        // why would null bytes appear... right, right?!
        let mut i = 0;
        for h in self.0.headers
        {
            let bf = h.value.to_vec();
            let strs = String::from_utf8_lossy(&bf).to_string();

            http_struct.value[i] = strs;
            http_struct.header[i] = h.name.to_string();

            i += 1;
        }

        return http_struct;

    }
}





//static mut TLS_CLIENT: Option<net_spx::TlsClient> = None;

/* 
fn start_com_cycle() -> u8
{
    
    if get_state() != STATE::UNINIT
    {
        dbg_log_progress("[!] Failed due to state lock");
        return 0;
    }

    dbg_log_progress("[*] __start_com__cycle INIT START");
    match __start_com_cycle__()
    {
        Ok(tc) => unsafe {TLS_CLIENT = Some(tc)},
        Err(_) => return 0
    };


    dbg_log_progress("[*] __start_com__cycle INIT DONE");
    set_state(STATE::INIT);

    return 1;

}
*/


#[tauri::command]
fn parse_burp_request_cache() -> String
{

    if get_state() != STATE::INIT
    {
        return String::new();
    }    

    let rust_string = parse_util::parse_burp_file();

    set_state(STATE::READY);
    return rust_string;

}


#[tauri::command]
fn permutate_request(perm_string: String, perm_mod: String) -> String
{
    if perm_string.is_empty() || perm_mod.is_empty()
    {
        return String::new();
    }

    //let dbg_s: String = "[+] Original String:\n".to_string() + &perm_string_c + "\n[+] Permuatation to insert:  " + &perm_mod_c;
    //dbg_log_progress(&dbg_s);

    let permutation = __permutate_request__(&perm_string, &perm_mod);

    return permutation;
}


#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum STATE 
{
    UNINIT,
    INIT,
    //LOCKED,
    READY
}

thread_local! {static STATE_T: RefCell<STATE> = RefCell::new(STATE::UNINIT);}

fn get_state() -> STATE
{
    let st: STATE = STATE_T.with(|state: &RefCell<STATE> | 
        {
            *state.borrow()
        });

    return st;
}

fn set_state(state_set: STATE) -> ()
{
    STATE_T.with(|state: &RefCell<STATE> | 
        {
            *state.borrow_mut() = state_set;
        });

}

/* 
fn send_com_keep_alive(request_s: String) -> ffi::HttpResponseDataKeepAliveC
{
    if get_state() != STATE::READY
    {
        dbg_log_progress("Send_Com failure: state not ready");
        return ffi::HttpResponseDataKeepAliveC::new(Vec::new(), 0, true);
    }


    dbg_log_progress("Reading request from C...");
    let reques_rs_s: std::string::String = request_s.to_string();

    
    let response =  unsafe { __send_comm_keepalive__ (&mut TLS_CLIENT.as_mut().unwrap(),reques_rs_s)};

    dbg_log_progress("Response generated, transferring to C...");
    match response 
    {
        Ok(hrdc) => return hrdc,
        Err(HTTPResult::WRITTING_STILL_INTO_BUFFER) => return ffi::HttpResponseDataKeepAliveC::new(Vec::new(), 1, true),
        Err(_) => return ffi::HttpResponseDataKeepAliveC::new(Vec::new(), 0, true)
    };

}

fn send_com(request_s: String) -> ffi::HttpResponseDataC
{
    if get_state() != STATE::READY
    {
        dbg_log_progress("[!] Send_Com failure: state not ready");
        return ffi::HttpResponseDataC::new((None, None), 0, request_s);
    }

    dbg_log_progress("[+] Reading request from C...");


    let response =  unsafe { __send_comm__ (request_s.clone())};

    dbg_log_progress("[+] Response generated, transferring to C...");
    match response
    {
        Ok(hrdc) => return hrdc,
        Err(_) => return ffi::HttpResponseDataC::new((None, None), 0, request_s)
    };

}
*/



