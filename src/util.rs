use std::{collections::{HashMap}, usize};

use rand::{prelude::SliceRandom, thread_rng};

use crate::types::{Player, Role, Status};

pub fn assign_card(v_num:u8, w_num:u8, p_num: u8) -> HashMap<u8,Player> {
    
    assert_eq!(v_num + w_num, p_num-3);

    let mut players:HashMap<u8, Player> = HashMap::new(); 
    let mut player_num:Vec<u8> = (0..p_num).map(|a| a+1).collect(); 
    let mut rng = thread_rng(); 
    player_num.shuffle(&mut rng); 
    

    for i in 0..v_num {
        players.insert(player_num[i as usize], Player{
            number: player_num[i as usize],
            status: Status::LIVED,
            role: Role::Villager,
        }); 
    }

    for i in v_num..v_num + w_num {
        players.insert(player_num[i as usize], Player{
            number: player_num[i as usize],
            status: Status::LIVED,
            role: Role::Wolf,
        }); 
    }

    players.insert(player_num[(v_num + w_num) as usize],Player{
        number: player_num[(v_num + w_num) as usize],
        status: Status::LIVED,
        role: Role::Seer,
    });

    players.insert(player_num[(v_num + w_num+1) as usize],Player{
        number: player_num[(v_num + w_num+1) as usize],
        status: Status::LIVED,
        role: Role::Witch,
    }); 

    players.insert(player_num[(v_num + w_num+2) as usize], Player{
        number: player_num[(v_num + w_num+2) as usize],
        status: Status::LIVED,
        role: Role::Hunter,
    });
    players
}

#[test]
#[should_panic]
fn test_assign_card(){
    let vec = assign_card(3,3, 10); 
    println!("{:?}", vec);
}