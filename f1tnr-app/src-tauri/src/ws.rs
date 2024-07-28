use std::{io, sync::Arc, time::Duration};

use futures::{stream::{SplitSink, SplitStream}, SinkExt, StreamExt, TryStreamExt};
use tokio::{ net::{TcpListener, TcpStream}, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

pub struct WServer(());
pub type RxArcMux = Arc<Option<Mutex<SplitStream<WebSocketStream<TcpStream>>>>>; 
pub type TxArcMux = Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>;

impl WServer
{




}



