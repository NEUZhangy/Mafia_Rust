#![feature(proc_macro_hygiene, decl_macro, async_closure)]
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use crossbeam_channel::unbounded;
use lazy_static::lazy_static;
use tokio;

use crate::game::init_game;
use crate::types::Msg;
use crate::websockets_chat::websocket_init;

mod util;
mod types;
mod actions;
mod websockets_chat;
mod game;

lazy_static! {
    pub static ref WEB:(Sender<Msg>, Receiver<Msg>) = unbounded();
    pub static ref BACK:(Sender<Msg>, Receiver<Msg>) = unbounded();
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    tokio::spawn(async {
        websocket_init().await;
    });
    init_game().await;
    loop {}
}



