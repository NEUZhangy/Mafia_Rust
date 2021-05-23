use crate::types::{Game, Msg, Status};
use crate::websockets_chat::{back_send_message, websocket_init};
use crate::BACK;

pub async fn init_game() ->anyhow::Result<()> {
    let mut game = Game::new(1,1,5);

    let mut msg =  BACK.1.recv()?;
    if msg.user_msg.eq("ready"){
        println!("enter ready");
        for (id, player) in &game.players {
            back_send_message(Msg{user_id:*id, user_msg: format!("{:?}", player.role)}).await;
        }
        println!("send role to user");
        //预言家验人, this part should replace with sound or boardcast the infor to everyone
        let seer_id = game.get_seer().unwrap().number;
        back_send_message(Msg{user_id:seer_id, user_msg: "预言家请验人".to_string()}).await;

        msg = BACK.1.recv()?;
        let seer_result = match game.seer_action( msg.user_msg.parse::<u8>()?){
            true => "狼人".to_string(),
            false => "好人".to_string(),
        };
        back_send_message(Msg{user_id:seer_id, user_msg:seer_result}).await;
        //狼人

        msg = BACK.1.recv()?;
        let killed_people_id =  msg.user_msg.parse::<u8>()?;
        game.wolf_action(killed_people_id);

        //wait second

        //女巫
        let witch_id = game.get_witch().unwrap().number;
        let witch_status = game.get_witch().unwrap().status;
        back_send_message(Msg{user_id: witch_id, user_msg: format!("{:?} 号玩家倒牌", killed_people_id).to_string()}).await;

        back_send_message(Msg{user_id: witch_id, user_msg: "是否使用解药: 20 ->使用， 21->不要".to_string()}).await;
        msg = BACK.1.recv()?;
        if msg.user_msg.parse::<u8>()? == 20 && witch_status!= Status::KILLED {
            game.witch_action(killed_people_id, Status::LIVED);
            //end action for witch
        } else {
            back_send_message(Msg{user_id: witch_id, user_msg: "是否使用毒药: 20 ->使用， 21->不要".to_string()}).await;
            msg = BACK.1.recv()?;
            if msg.user_msg.parse::<u8>()? == 20 {
                back_send_message(Msg{user_id: witch_id, user_msg: format!("给号码: 1-{:?}", game.p_num).to_string()}).await;
                msg = BACK.1.recv()?;
                let poison_people_id = msg.user_msg.parse::<u8>()?;
                game.witch_action(poison_people_id, Status::POISONED)
                //end for witch
            }
        }
        //end for witch

        //猎人,start hunter
        let hunter_id = game.get_hunter().unwrap().number;
        let hunter_status = match game.get_hunter().unwrap().status {
            Status::POISONED => "不能开枪".to_string(),
            _  => "可以开枪".to_string(),
        };
        back_send_message(Msg{user_id: hunter_id, user_msg: hunter_status}).await;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_game_init(){
    tokio::spawn(async{
        websocket_init().await;

    });

    init_game().await;
    loop{}
}
