// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]



use futures::future::join_all;
use tokio::task::{spawn_blocking, JoinHandle};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tokio::sync::{mpsc, Mutex, OnceCell};
use tokio::sync::mpsc::Sender;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::tungstenite::http::request;
use tokio_tungstenite::tungstenite::Message;
use core::time;

use std::cell::{Ref, RefCell};
use std::fs::read_to_string;
use std::num::IntErrorKind;

use std::process::Command;
use std::sync::Arc;
use std::thread::sleep;

use tauri::http::ResponseBuilder;
use tauri::{window, Manager, Window};

use tokio_tungstenite::{accept_hdr_async, WebSocketStream};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};



fn main()
{
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![start_async_engine])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}



fn readfile_lines(dirstr: String) -> Vec<String>
{
    let mut result: Vec<String> = Vec::new();

    let lines = read_to_string(dirstr).unwrap()
    .lines().map(|l| result.push(l.into()));
    
    return result;
}

#[tauri::command]
fn start_async_engine(http_request: String, permutation_filepath: String)
{

    let output = Command::new("../async_net_engine/AsyncNetEngine/target/debug/AsyncNetEngine")
    .arg(http_request)
    .arg(permutation_filepath)
    .output().unwrap();

    
    println!("{}", String::from_utf8_lossy(&output.stdout) );

}

