
use std::{io::Error, sync::Arc};

use futures::future::join_all;
use rustls::{RootCertStore, ClientConfig};

use tokio::{io::{self, AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::task::JoinHandle;
use tokio_rustls::client::TlsStream;

use crate::{interface_structs::{HttpRequest, RequestandPermutation}, log::form_log_string};
use crate::log::{log_f, LogType};

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

struct TlsConnection
{
    tls_pipe: Option<TlsStream<TcpStream>>,
    keepalive_support: Option<bool>,
    domain_string: String,
    root_store: RootCertStore
}

impl TlsConnection
{
    pub async fn new(domain_s: String, root_store_obj: RootCertStore) -> io::Result<TlsConnection>
    {
        let tcp_stream = TcpStream::connect(domain_s.clone() + ":443").await?;

        tcp_stream.set_nodelay(true).unwrap();
    
        let client_config = ClientConfig::builder()
            .with_root_certificates(root_store_obj.clone())
            .with_no_client_auth();
    
        let conn = tokio_rustls::TlsConnector::from(Arc::new(client_config));
    
        let tls_stream = conn.connect(domain_s.clone().try_into().unwrap(), tcp_stream).await?;
      

        return Ok(TlsConnection
        {
            tls_pipe: Some(tls_stream),
            keepalive_support: None,
            domain_string: domain_s,
            root_store: root_store_obj
        });
    }

    pub fn borrow_pipe(&mut self) -> Result<&mut TlsStream<TcpStream>, Error>
    {
        match &mut self.tls_pipe
        {
            Some(t) => return Ok(t),
            None => return Err(Error::new(io::ErrorKind::BrokenPipe, "Tls stream pipe is closed"))
        };
    }

    pub async fn reconnect(&mut self) -> io::Result<()>
    {
        let tcp_stream = TcpStream::connect(self.domain_string.clone() + ":443").await?;

        tcp_stream.set_nodelay(true).unwrap();
    
        let client_config = ClientConfig::builder()
            .with_root_certificates(self.root_store.clone())
            .with_no_client_auth();
    
        let conn = tokio_rustls::TlsConnector::from(Arc::new(client_config));
    
        self.tls_pipe = Some(conn.connect(self.domain_string.clone().try_into().unwrap(), tcp_stream).await?);

        return Ok(());
    }

    async fn chk_if_keepalive_supported(&mut self, response: &str) -> bool
    {
        let response_head = response.split("\r\n\r\n").next().unwrap();
        if response_head.contains("Connection: close\r\n") 
        {
            self.keepalive_support = Some(false);
            return false;
        }
        
        self.keepalive_support = Some(true);
        return true; 

    }

    async fn log_http_data(&mut self, buffer: &[u8], request: &str, request_id: u32, thread_id: u32)
    {
        let fin = String::from_utf8_lossy(&buffer).to_string();
        println!("RESPONSE=======\n{}", &fin);
        self.keep_alive_chk(&fin).await;

        let log_s = form_log_string(request, fin, request_id);
        log_f(log_s, LogType::DataFile(thread_id));
    }

    async fn keep_alive_chk(&mut self, final_s: &str)
    {
        let ka_support = match self.keepalive_support
        {
            Some(support) => support,
            None => self.chk_if_keepalive_supported(final_s).await,
        };

        if !ka_support
        {
            _ = self.tls_pipe.as_mut().unwrap().shutdown().await;
            self.tls_pipe = None;
        }
    }

    pub async fn make_http_request(&mut self, request: &str, request_id: u32, thread_id: u32)  -> io::Result<String>
    {
        println!("REQUEST {} :\n", request_id);


        let now = std::time::Instant::now();
        // use client again if keep alive, if not, use a new oborrow_pipene
        let tls_ref = match self.borrow_pipe()
        {
            Ok(t) => t,
            Err(_) =>
            {
                self.reconnect().await.unwrap();
                self.borrow_pipe().unwrap()
            },
        };
        
        tls_ref.write_all(request.as_bytes()).await.unwrap();
        tls_ref.flush().await.unwrap();
        
        let mut buffer: Vec<u8> = Vec::new();
        let mut rd_buf = [0u8; 4096];
 
        loop
        {
            let bytes_r = match tls_ref.read(&mut rd_buf[..]).await
            {
                Ok(b) => b,
                Err(_) =>
                {
                    // TODO:log error here
                    self.log_http_data(&buffer, request, request_id, thread_id ).await;
                    let _ = self.reconnect();
                    return Ok(String::new())
                },
            };

            buffer.extend_from_slice(&rd_buf[..bytes_r]);

            match chk_if_http_is_done(&buffer).await
            {
                HttpStatus::FullyConstructed => 
                {
                    self.log_http_data(&buffer, request, request_id, thread_id ).await;
                    break;
                }

                HttpStatus::FullyConstructedHeaderOnly =>
                {
                    self.log_http_data(&buffer, request, request_id, thread_id ).await;
                    break;
                }

                HttpStatus::NotDone => continue
            }

        }

        println!("Request {} took {} seconds!", request_id, now.elapsed().as_secs());
        Ok(String::new())
    }
}

/*
TODO:
 1. Ill have to handle br, gzip, compress, deflate, zstd, etc OR strip accept-encoding from all requests
 2. OR NOT and jus tell servers we dont do that shit around these parts
*/



pub fn configure_workload(mut vector_rp: RequestandPermutation, reqs_per_thread: u32) -> Vec<RequestandPermutation>
{

    assert!(vector_rp.request.len() == vector_rp.permutation.len());
    if vector_rp.request.len() < 25
    {
        let mut one_workload: Vec<RequestandPermutation> = Vec::new();
        one_workload.push(vector_rp);
        return one_workload;
    }

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


pub async fn start_taskmaster(domain_string: String, request_groupings: Vec<RequestandPermutation>) 
{

    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(
    webpki_roots::TLS_SERVER_ROOTS
        .iter()
        .cloned()
    );
    println!("here");

    let mut requests_joinhandle_v: Vec<JoinHandle<()>> = Vec::new();

    let mut thread_id: u32 = 0;
    for rp in request_groupings
    {
        let root_store_dup = root_store.clone();
        let d_s = domain_string.clone();
       
        requests_joinhandle_v.push(tokio::spawn(start_worker(d_s,rp , root_store_dup, thread_id)));
        thread_id += 1;
    }

    join_all(requests_joinhandle_v).await;
    
}


async fn start_worker(d_s: String, request_perumation_buffer: RequestandPermutation, root_store: RootCertStore, thread_id: u32) -> ()
{

    println!("[+] Worker {} started", thread_id.to_string());

    let future_v: Vec<JoinHandle<()>> = Vec::new();
    let mut tls = TlsConnection::new(d_s, root_store).await.unwrap();

    for http_r in &request_perumation_buffer.request
    {
        let _ = tls.make_http_request(&http_r.request, http_r.request_number, thread_id).await;


    }



    join_all(future_v).await; 
    return;
}





#[inline(always)]
// perhaps CL can represenrt the bytes left to read
async fn chk_if_http_is_done(accum: &[u8]) -> HttpStatus
{


    let response = String::from_utf8_lossy(&accum).to_string();
    let target_content_len  = chk_content_length(&accum).await;
    let current_content_len = determine_body_sz_in_accum(&accum).await;

    if !response.contains("\r\n\r\n")
    {
        return HttpStatus::NotDone;
    }


    if !response.contains("Content-Length") && !response.contains("content-length") || target_content_len == 0
    {
        return HttpStatus::FullyConstructedHeaderOnly; // No body, message end 
    }


    if response.contains("\r\n\r\n") && target_content_len == current_content_len
    {
        return HttpStatus::FullyConstructed; // Headers are ended with CLRFCLRF && we have the desired amount of bytes
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
        if response.contains("HTTP/1.1") && (l.contains("Content-Length") || l.contains("content-length")) && response.contains("\r\n\r\n")
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
            println!("Half {}", half);
            return half.len().try_into().unwrap();
        }

        
    }

    return 0; //failure or headers only
}


fn _kq_straggler(d_s: String,rs: &str, root_store: RootCertStore, id: u32) -> JoinHandle<()>
{

    let hr = HttpRequest::new(rs.to_string(), id);
    let r = RequestandPermutation
    {
        request: vec![hr; 1],
        permutation: vec!["perm".to_string(); 1]
    };

    println!("Spawning KQ Task due to connection closed>>>>");

    return tokio::spawn(async move 
    {
        start_worker(d_s, r, root_store, 0).await;
    });
}



/* 
mod tests
{
    use std::io::Read;

    use crate::interface_structs::HttpRequest;

    use super::*;

    #[test]
    fn test_workload_provisioning() -> ()//bool
    {
        let mut id_c = 0;
        let http_request = HttpRequest::new("foo".to_string(), 0);
        let rp = RequestandPermutation
        {
       
            request: vec!["rock".;377],
            permutation: vec!["reskl".to_string();377]
        };
        let ilen = rp.request.len();

        let child_vectors = configure_workload(rp, 15);
        let accum_len: usize = child_vectors
        .iter().map(|v|{ v.request.len() } ).sum();
        println!("inital-len: {},\ndivided-len: {}", ilen, accum_len);
        assert!(accum_len == ilen,
             "Requests would be lost in this configuration algo:\n\tinital-len: {},\n\tdivided-len: {}", 
            ilen,
            accum_len);

    }
    
        
}   

*/
