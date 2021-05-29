use crate::{types::{Game, Player, Role, Status}, util::assign_card};

impl Game {
    pub fn new(v_num: u8, w_num: u8, p_num: u8) -> Self {
        Game {
            v_num,
            w_num,
            p_num,
            players: assign_card(v_num, w_num, p_num),
        }
    }

    pub fn get_hunter(&self) -> Option<&Player> {
        for (_key, player) in self.players.iter() {
            if player.role == Role::Hunter {
                return Some(player);
            }
        }
        None
    }

    pub fn get_witch(&self) -> Option<&Player> {
        for (_key, player) in self.players.iter() {
            if player.role == Role::Witch {
                return Some(player);
            }
        }
        None
    }

    pub fn get_seer(&self) -> Option<&Player> {
        for (_key, player) in self.players.iter() {
            if player.role == Role::Seer {
                return Some(player);
            }
        }
        None
    }

    pub fn player_status_change(&mut self, p_num: u8, status: Status) {
        if let Some(player) = self.players.get_mut(&p_num) {
            player.status = status;
        }
    }

    pub fn wolf_action(&mut self, p_num: u8) {
        self.player_status_change(p_num, Status::KILLED);
    }

    pub fn witch_action(&mut self, p_num: u8, status: Status) {
        if let Some(player) = self.players.get(&p_num) {
            if player.role == Role::Witch {
                return;
            }
        }
        self.player_status_change(p_num, status);
    }

    pub fn hunter_action(&mut self, p_num: u8, status: Status) {
        if let Some(player) = self.get_hunter() {
            if player.status == Status::POISONED {
                return;
            }
        }
        self.player_status_change(p_num, status);
    }

    pub fn seer_action(&mut self, p_num: u8) -> bool {
        if let Some(player) = self.players.get(&p_num) {
            if player.role == Role::Wolf {
                return true;
            }
        };
        return false;
    }

    pub fn get_death_info(&self) -> String {
        let mut v = Vec::new();
        for (_key, player) in self.players.iter() {
            if player.status == Status::POISONED || player.status == Status::KILLED {
                v.push(player.number);
            }
        }
        if v.len() > 0 {
            format!("{} 号玩家死亡", v.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","))
        } else {
            "今天晚上平安夜".to_string()
        }
    }
}


#[test]
fn test_game() {
    let game = Game::new(3, 3, 9);
    info!("{:?}", game);
}

#[test]
fn test_get_hunter() {
    let game = Game::new(3, 3, 9);

    info!("{:?}", game.get_hunter());
}

#[test]
fn test_player_status_change() {
    let mut game = Game::new(3, 3, 9);
    game.player_status_change(4, Status::KILLED);
    info!("{:?}", game);
}

#[test]
fn test_witch_action() {
    let mut game = Game::new(3, 3, 9);
    let witch = game.get_witch().unwrap().clone();
    game.player_status_change(witch.number, Status::KILLED);
    game.witch_action(witch.number, Status::LIVED);

    info!("{:?}", game);
}

#[test]
fn test_hunter_action() {
    let mut game = Game::new(3, 3, 9);
    let hunter = game.get_hunter().unwrap().clone();
    // game.player_status_change(hunter.number, Status::POISONED);
    game.hunter_action(9, Status::KILLED);

    info!("{:?}", game);
}