use core::time;
use std::thread::sleep;
use std::{fs::read, sync::Arc};
use std::io::Write;
use futures::channel::mpsc::unbounded;
use futures::{future, StreamExt, TryStreamExt};
use tauri::async_runtime;
use tokio::net::{TcpListener, TcpSocket, TcpStream as tTcpStream};
use futures::{future::join_all, SinkExt, TryFutureExt};
use rustls::{RootCertStore, ClientConfig};
use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use tokio::task::JoinHandle;

use crate::{interface_structs::{HttpResponseDataC, RequestandPermutation}, log::dbg_log_progress, DOMAIN_BUF};

use tokio_tungstenite::accept_hdr_async;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};

// ASYNC RE_WRITE===
struct WorkerLoad
{
    work_grp_num: u32,
    tasks_per: u32,
    remainder: u32
}

enum HttpStatus 
{
    FullyConstructedHeaderOnly,
    FullyConstructed,
    NotDone
}

async fn process(socket: TcpStream)
{
    
}


async fn start_ipc_server()
{
    let socket = TcpSocket::new_v4().unwrap();
    socket.set_keepalive(true).unwrap();
    socket.set_reuseaddr(true).unwrap();
    socket.set_reuseport(true).unwrap();
    socket.bind("localhost:3001".parse().unwrap()).unwrap();

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
    

   

    loop {
        tx.send("PING".into()).await.unwrap();
        
        while let Some(msg) = rx.next().await {
            match msg.unwrap().into_text().unwrap().as_str() {
                "PONG" => tx.send("PING".into()).await.unwrap(),
                _ => ()
            };
        }
    }
}

pub async fn start_taskmaster(domain_string: String, mut request_perumation_buffer: RequestandPermutation, reqs_per_thread: u32) 
{

    let ipc_future = start_ipc_server();
  

    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
    );
    dbg_log_progress("CHK: RootCertStore up");
    let request_groupings = configure_workload(&mut request_perumation_buffer, reqs_per_thread);

    let mut straggler_kq_v: Vec<JoinHandle<()>> = Vec::new();
    dbg_log_progress("CHK: Starting to spawn workers");
    for rp in request_groupings
    {
        let root_store_dup = root_store.clone();
        let d_s = domain_string.clone();
       

        
        straggler_kq_v.push(tokio::spawn(async move 
        {
            start_worker(d_s,rp , root_store_dup).await;
        }));
    }
    join_all(straggler_kq_v).await;
    ipc_future.await;
}

fn configure_workload(mut vector_rp: &mut RequestandPermutation, reqs_per_thread: u32) -> Vec<RequestandPermutation>
{
  

    assert!(vector_rp.request.len() == vector_rp.permutation.len());
    let ilen = vector_rp.request.len();
    let mut wrk = WorkerLoad
    {
        work_grp_num: 0,
        tasks_per: 0,
        remainder: 0,
    };

    wrk.work_grp_num = vector_rp.request.len() as u32 / reqs_per_thread;
    wrk.tasks_per = reqs_per_thread;
    wrk.remainder = vector_rp.request.len() as u32 - (wrk.work_grp_num * wrk.tasks_per);

    let mut vector_collection = fill_child_vectors(&mut vector_rp, &wrk);
    if wrk.remainder == 0 
    {

        let accum_len: usize = vector_collection
        .iter().map(|v|{ v.request.len() } ).sum();
        assert!(accum_len == ilen);

        return vector_collection;
    }

    while wrk.remainder != 0
    {
        for x in 0..wrk.work_grp_num as usize
        {
            if vector_rp.request.len() == 0 
            {
                return vector_collection;
            }
            vector_collection[x].request.push(vector_rp.request.pop().unwrap());
            vector_collection[x].permutation.push(vector_rp.permutation.pop().unwrap());
            wrk.remainder -= 1;
            
        }
    }

    dbg_log_progress("CHK-DONE: PROVISIONING");
    return vector_collection;
}

fn fill_child_vectors(v: &mut RequestandPermutation, wrk: &WorkerLoad) -> Vec<RequestandPermutation>
{
    let mut vector_collection: Vec<RequestandPermutation> = Vec::new();
    for i in 0..wrk.work_grp_num as usize
    {
        vector_collection.push(RequestandPermutation::new());
        for _ in 0..wrk.tasks_per
        {
            assert!(v.request.len() != 0);
            vector_collection[i].request.push(v.request.pop().unwrap());
            vector_collection[i].permutation.push(v.permutation.pop().unwrap());

            if v.request.len() == 0 { return vector_collection; }
        }
        
    }
    return vector_collection;
}

async fn start_worker(d_s: String, request_perumation_buffer: RequestandPermutation, root_store: RootCertStore) -> ()
{
   
    // this just lets jus boot our connection back up if we get our connection closed on us
    // this can happen if the server returns a 404 and insta-closes 
    let mut straggler_kq_v: Vec<JoinHandle<()>> = Vec::new();
    let mut resume = 0; // -1 as it will be used for indexing
    'worker_start: loop {
        let tcp_stream = match TcpStream::connect(d_s.clone() + ":443").await
        {
            Ok(t) => t,
            Err(_) => 
            {
    
                return;
            }
        };
        tcp_stream.set_nodelay(true).unwrap();
    
        //println!("===Starting Worker...");
        let client_config = ClientConfig::builder()
            .with_root_certificates(root_store.clone())
            .with_no_client_auth();
    
        let conn = tokio_rustls::TlsConnector::from(Arc::new(client_config));
    
  
        let mut t = match conn.connect(d_s.clone().try_into().unwrap(), tcp_stream).await
        {
            Ok(t) => t,
            Err(_) => 
            {
                let dbg_s = "[!] Worker unable to connect to ".to_string() + &d_s;
                println!("{}", &dbg_s);
                return;
    
                
            }
        };
        
        
        'out: for rs in &request_perumation_buffer.request[resume..]
        {
            if resume == request_perumation_buffer.request.len()
            {
                return;
            }
            t.write_all(rs.as_bytes()).await.unwrap();
            //println!("Written:\n{}", &rs);
            t.flush().await.unwrap();
            
    
            let mut b: Vec<u8> = Vec::new();
            let mut rd_buf = [0u8; 4096];
            
            loop 
            {
                

                // failing to avoid this read when there is nothing left, is e v e r y thing
                let _bytes_read = t.read(&mut rd_buf[..]).await.unwrap();
                if _bytes_read == 0 
                { 
            
                    straggler_kq_v.push(kq_straggler(d_s.clone(), &rs, root_store.clone()));
                    resume += 1;
                    continue 'worker_start;
                } // TODO: we would log a failure
                b.extend_from_slice(&rd_buf[.._bytes_read]);
           
                
               match chk_if_http_is_done(&b).await
               {
                    HttpStatus::FullyConstructed => 
                    {
                       
                        let fin = String::from_utf8_lossy(&b)
                            .to_string();
                        
                        resume += 1;
                        continue 'out;
                    }
    
                    HttpStatus::FullyConstructedHeaderOnly =>
                    {
                        
                        let fin = String::from_utf8_lossy(&b)
                            .to_string();
                        
                        resume += 1;
                        continue 'out;
                    }

                    HttpStatus::NotDone => continue
               }
     
               
            }   
    
            
            
    
        }
        break;
    }
    join_all(straggler_kq_v).await; 
    return;
}

/* 
fn access_and_increment_rowlevel() -> ()//u16
{
  
    let row = ROW_LEVEL.with(|i: &Arc<RefCell<u16>>|
        {
            *i.borrow_mut() += 1;
            let i_in = *i.borrow_mut();
            i_in.clone() - 1
        });

    return row;
   
}
 */

#[inline(always)]
// perhaps CL can represenrt the bytes left to read
async fn chk_if_http_is_done(accum: &[u8]) -> HttpStatus
{


    let response = String::from_utf8_lossy(&accum).to_string();
    let target_len  = chk_content_length(&accum).await;
    let current_len = determine_body_sz_in_accum(&accum).await;

    //println!("{} out of {} body bytes read!", current_len, target_len);

    if response.len() != 0 
    {
        //assert!(response.contains("HTTP/1.1"));
    }


    if response.contains("\r\n\r\n") && !response.contains("Content-Length") && !response.contains("content-length")
    {
        //println!("Valid-HO:\n{}", response);
        return HttpStatus::FullyConstructedHeaderOnly; // No body, message end 
        
    }

    if response.contains("\r\n\r\n") && target_len <= current_len
    {
        //println!("Valid:\n{}", response);
        return HttpStatus::FullyConstructed;
    }

    return HttpStatus::NotDone; // Incomplete response, read more;
}

#[inline(always)]
async fn chk_content_length(accum: &[u8]) -> isize
{
    let response = String::from_utf8_lossy(&accum).to_string();
    let lines = response.split("\r\n");
    for l in lines
    {
        if response.contains("HTTP/1.1") &&
        (l.contains("Content-Length") || l.contains("content-length")) && response.contains("\r\n\r\n") 
        {
            let body_len = if l.contains("Content-Length") 
            {
                l.replace("Content-Length: ", "").trim()
                    .parse::<isize>().unwrap()
            } else 
            {
                l.replace("content-length: ", "").trim()
                    .parse::<isize>().unwrap()
            };     
            return body_len as isize; // there is a body, and it is next
        }
    }

    if response.contains("HTTP/1.1") && response.contains("\r\n\r\n")
    {
        return 0; // Response done, only the header
    }

    return -1; // return -1 when not even the full http header has been received 
}


#[inline(always)]
async fn determine_body_sz_in_accum(accum: &[u8]) -> isize
{
    let response = String::from_utf8_lossy(&accum).to_string();
    let sub_strs = response.split("\r\n\r\n");

    for half in sub_strs
    {
        
        if !half.contains("HTTP/1.1") && !half.is_empty()
        {
            return half.len().try_into().unwrap();
        }
        
    }

    return 0; //failure or headers only
}


fn kq_straggler(d_s: String,rs: &str, root_store: RootCertStore) -> JoinHandle<()>
{
    let r = RequestandPermutation
    {
        request: vec![rs.to_string(); 1],
        permutation: vec!["perm".to_string(); 1]
    };

    println!("Spawning KQ Task due to connection closed>>>>");
    return tokio::spawn(async move 
        {
            start_worker(d_s, r, root_store).await;
        });
}




mod tests
{
    use std::io::Read;

    use super::*;

    #[test]
    fn test_workload_provisioning() -> ()//bool
    {
        let mut rp = RequestandPermutation
        {
            request: vec!["rock".to_string();377],
            permutation: vec!["reskl".to_string();377]
        };
        let ilen = rp.request.len();

        let child_vectors = configure_workload(&mut rp, 15);
        let accum_len: usize = child_vectors
        .iter().map(|v|{ v.request.len() } ).sum();
        println!("inital-len: {},\ndivided-len: {}", ilen, accum_len);
        assert!(accum_len == ilen,
             "Requests would be lost in this configuration algo:\n\tinital-len: {},\n\tdivided-len: {}", 
            ilen,
            accum_len);

    }
    
    #[test]
    fn log_test() -> ()
    {
        dbg_log_progress("Check")
    }
        
}   