// #![deny(warnings)]
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use warp::Filter;
use std::sync::mpsc::{Sender, Receiver};
use crate::types::Msg;
use std::sync::atomic::AtomicU8;
use state::Storage;
use crate::BACK;
use lazy_static::lazy_static;


/// Our global unique user id counter.
static NEXT_USER_ID: AtomicU8 = AtomicU8::new(1);
type Users = Arc<RwLock<HashMap<u8, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>;

lazy_static!{
    pub static ref USERS:Users = Users::default();
}
/// Our state of currently connected users.
///
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`

pub async fn websocket_init() {
    // pretty_env_logger::init();

    // Keep track of all connected users, key is usize, value
    // is a websocket sender.
    // Turn our "state" into a new Filter...

    // GET /chat -> websocket upgrade
    let chat = warp::path("chat")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket))
        });

    // GET / -> index html
    let index = warp::path::end().map(|| warp::reply::html(INDEX_HTML));

    let routes = index.or(chat);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    // tokio::spawn(async{
    //     while let Ok(msg) = web_rx.recv() {
    //         user_message(msg.user_id, Message::text(msg.user_msg), &users);
    //     }
    // });
}

async fn user_connected(ws: WebSocket) {
    // Use a counter to assign a new unique ID for this user.
    let my_id = NEXT_USER_ID.fetch_add(1, Ordering::Relaxed);
    eprintln!("new chat user: {}", my_id);

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the websocket...
    let (tx, rx) = mpsc::unbounded_channel();
    let rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(rx.forward(user_ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    // Save the sender in our list of connected users.
    USERS.write().await.insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.

    // Make an extra clone to give to our disconnection handler...
    let users2 = USERS.clone();

    // Every time the user sends a message, broadcast it to
    // all other users...
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", my_id, e);
                break;
            }
        };

        let send_msg = match msg.to_str(){
            Ok(send_msg) => send_msg,
            Err(e) => ""
        };

        BACK.0.send( Msg{user_id:my_id, user_msg: send_msg.to_string()});
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    user_disconnected(my_id, &users2).await;
}

async fn user_message(my_id: u8, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = format!("<User#{}>: {}", my_id, msg);

    // New message from this user, send it to everyone else (except same uid)...
    for (&uid, tx) in users.read().await.iter() {
        if my_id != uid {
            if let Err(_disconnected) = tx.send(Ok(Message::text(new_msg.clone()))) {
                // The tx is disconnected, our `user_disconnected` code
                // should be happening in another task, nothing more to
                // do here.
            }
        }
    }
}

async fn user_disconnected(my_id: u8, users: &Users) {
    eprintln!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.write().await.remove(&my_id);
}

pub async fn back_send_message(msg:Msg){
    //listen to logic, when receive msg, send to user
    USERS.read().await.get(&msg.user_id).map(|tx| {
        if let Err(_disconnected) = tx.send(Ok(Message::text(msg.user_msg.clone()))) {
        }
    });

}

static INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <title>Warp Chat</title>
    </head>
    <body>
        <h1>Warp chat</h1>
        <div id="chat">
            <p><em>Connecting...</em></p>
        </div>
        <input type="text" id="text" />
        <button type="button" id="send">Send</button>
        <script type="text/javascript">
        const chat = document.getElementById('chat');
        const text = document.getElementById('text');
        const uri = 'ws://' + location.host + '/chat';
        const ws = new WebSocket(uri);

        function message(data) {
            const line = document.createElement('p');
            line.innerText = data;
            chat.appendChild(line);
        }

        ws.onopen = function() {
            chat.innerHTML = '<p><em>Connected!</em></p>';
        };

        ws.onmessage = function(msg) {
            console.log(msg);
            message(msg.data);
        };

        ws.onclose = function() {
            chat.getElementsByTagName('em')[0].innerText = 'Disconnected!';
        };

        send.onclick = function() {
            const msg = text.value;
            ws.send(msg);
            text.value = '';

            message('<You>: ' + msg);
        };
        </script>
    </body>
</html>
"#;
#[tokio::test(flavor = "multi_thread")]
async fn websocket_init_test() {

    tokio::spawn(async{
        websocket_init().await;
    });

    println!("start 10 sec");
    std::thread::sleep(std::time::Duration::from_secs(10));
    USERS.read().await.get(&1).unwrap().send(Ok(Message::text("getString")));
    println!("{:?}", BACK.1.recv());
    println!("end 10 sec");
    loop{}
}

