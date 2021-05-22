#![feature(proc_macro_hygiene, decl_macro,async_closure)]

use std::thread;
use tokio;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crate::types::Msg;
use lazy_static::lazy_static;
use crossbeam_channel::unbounded;

mod util;
mod types; 
mod actions;
mod websockets_chat;

lazy_static!{
    pub static ref WEB:(Sender<Msg>, Receiver<Msg>) = unbounded();
    pub static ref BACK:(Sender<Msg>, Receiver<Msg>) = unbounded();
}

#[tokio::main]
async fn main() {

    tokio::spawn(async {
        websockets_chat::websocket_init().await;
    });
    loop {}
}



