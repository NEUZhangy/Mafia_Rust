#![feature(proc_macro_hygiene, decl_macro,async_closure)]

use std::thread;
use tokio;
use crossbeam_channel::Sender;
use crossbeam_channel::Receiver;
use crate::types::Msg;
use lazy_static::lazy_static;
use crossbeam_channel::unbounded;
use crate::websockets_chat::websocket_init;


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

    tokio::spawn(async{
        websocket_init().await;
        loop{}
    });



    println!("start 10 sec");
    std::thread::sleep(std::time::Duration::from_secs(10));
    WEB.0.send(Msg{user_id:1, user_msg:"ssss".to_string()}).unwrap();
    println!("end 10 sec");

}



