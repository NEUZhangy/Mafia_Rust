use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Msg {
    pub user_id: u8,
    pub user_msg: String,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Status {
    LIVED,
    KILLED,
    POISONED,
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Role {
    Villager,
    Wolf,
    Seer,
    Witch,
    Hunter
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Player {
    pub number: u8,
    pub status: Status,
    pub role: Role,
}

#[derive(Debug)]
pub struct Game {
    pub v_num:u8,
    pub w_num:u8,
    pub p_num:u8,
    pub players: HashMap<u8, Player>,
}

