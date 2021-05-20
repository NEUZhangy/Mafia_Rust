#![feature(proc_macro_hygiene, decl_macro,async_closure)]

use std::thread;
use tokio;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use crate::types::Msg;

mod util;
mod types; 
mod actions;
mod websockets_chat;
 
#[tokio::main]
async fn main() {

    let (web_tx, web_rx): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();
    let (back_tx, back_rx):(Sender<Msg>, Receiver<Msg>) = mpsc::channel(); //logic



    tokio::spawn(async {
        websockets_chat::websocket_init(back_tx.clone(), web_rx.clone()).await;
    });  
    loop {}
}



