use std::io::BufReader;
use std::thread;
use std::time::Duration;

use crate::BACK;
use crate::types::{Game, Msg, Status};
use crate::websockets_chat::{back_send_message, websocket_init, broadcast};

pub async fn init_game() -> anyhow::Result<()> {
    let mut game = Game::new(1, 1, 5);
    let mut msg = BACK.1.recv()?;
    if msg.user_msg.eq("ready") {
        info!("enter ready");

        for (id, player) in &game.players {
            back_send_message(Msg { user_id: *id, user_msg: format!("你的号码是{:?}，你的身份是{:?}", player.number, player.role) }).await;
        }
        play_sound("voice/check_role.mp3")?;

        BACK.1.recv()?;

        play_sound("voice/players_close_eyes.mp3")?;
        info!("send role to user");
        thread::sleep(Duration::from_secs(3));

        play_sound("voice/seer_open_eye.mp3")?;
        //预言家验人, this part should replace with sound or boardcast the infor to everyone
        let seer_id = game.get_seer().unwrap().number;
        play_sound("voice/seer_verify.mp3")?;
        back_send_message(Msg { user_id: seer_id, user_msg: "预言家请验人".to_string() }).await;

        msg = BACK.1.recv()?;
        let seer_result = match game.seer_action(msg.user_msg.parse::<u8>()?) {
            true => "狼人".to_string(),
            false => "好人".to_string(),
        };
        back_send_message(Msg { user_id: seer_id, user_msg: seer_result }).await;
        thread::sleep(Duration::from_secs(3));
        play_sound("voice/seer_close_eye.mp3")?;

        //狼人
        thread::sleep(Duration::from_secs(3));
        play_sound("voice/wolf_kill.mp3")?;
        msg = BACK.1.recv()?;
        let killed_people_id = msg.user_msg.parse::<u8>()?;
        game.wolf_action(killed_people_id);
        play_sound("voice/wolf_close_eye.mp3")?;
        //wait second
        thread::sleep(Duration::from_secs(3));


        //女巫
        play_sound("voice/witch_open_eye.mp3")?;
        let witch_id = game.get_witch().unwrap().number;
        let witch_status = game.get_witch().unwrap().status;
        back_send_message(Msg { user_id: witch_id, user_msg: format!("{:?} 号玩家倒牌", killed_people_id).to_string() }).await;

        back_send_message(Msg { user_id: witch_id, user_msg: "是否使用解药: 20 ->使用， 21->不要".to_string() }).await;
        msg = BACK.1.recv()?;
        if msg.user_msg.parse::<u8>()? == 20 && witch_status != Status::KILLED {
            game.witch_action(killed_people_id, Status::LIVED);
            //end action for witch
        } else {
            back_send_message(Msg { user_id: witch_id, user_msg: "是否使用毒药: 20 ->使用， 21->不要".to_string() }).await;
            msg = BACK.1.recv()?;
            if msg.user_msg.parse::<u8>()? == 20 {
                back_send_message(Msg { user_id: witch_id, user_msg: format!("给号码: 1-{:?}", game.p_num).to_string() }).await;
                msg = BACK.1.recv()?;
                let poison_people_id = msg.user_msg.parse::<u8>()?;
                game.witch_action(poison_people_id, Status::POISONED)
                //end for witch
            }
        }
        play_sound("voice/witch_close_eye.mp3")?;
        //end for witch
        thread::sleep(Duration::from_secs(6));


        //猎人,start hunter
        play_sound("voice/hunter_open_eye.mp3")?;
        let hunter_id = game.get_hunter().unwrap().number;
        let hunter_status = match game.get_hunter().unwrap().status {
            Status::POISONED => "不能开枪".to_string(),
            _ => "可以开枪".to_string(),
        };
        back_send_message(Msg { user_id: hunter_id, user_msg: hunter_status }).await;

        BACK.1.recv()?;
        play_sound("voice/hunter_close_eye.mp3")?;
        thread::sleep(Duration::from_secs(3));

        play_sound("voice/players_open_eye.mp3")?;
        info!("{:#?}", game);

        BACK.1.recv()?;

        broadcast(Msg {user_id: 0, user_msg: game.get_death_info()}).await;
    }

    Ok(())
}

fn play_sound(path: &str) -> anyhow::Result<()> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;

    let file = std::fs::File::open(path)?;
    sink.append(rodio::Decoder::new(BufReader::new(file))?);

    sink.sleep_until_end();
    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_game_init() {
    tokio::spawn(async {
        websocket_init().await;
    });

    init_game().await;
    loop {}
}

#[test]
fn test_play_sound() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("voice/check_role.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.sleep_until_end();
}
