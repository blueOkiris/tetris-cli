/*
 * Author: Dylan Turner
 * Description: High Score save system. Yes, it's overkill
 */

use serde::{
    Serialize, Deserialize
};
use confy::{
    load, store
};
use rand::{
    Rng, thread_rng
};
use std::{
    hash::{
        Hash, Hasher
    }, collections::hash_map::DefaultHasher
};

const APP_NAME: &'static str = "tetris-cli";

// High score saving
#[derive(Serialize, Deserialize)]
pub struct SaveData {
    high_score: u64,
    high_score_salt: String, // Must match or HS resets to 0
    high_score_hash: String
}

impl SaveData {
    pub fn load_config() -> Self {
        match load(APP_NAME) {
            Err(_) => {
                SaveData::default()
            }, Ok(save) => save
        }
    }

    fn get_random_hash() -> String {
        let mut ret = String::new();
        let mut rng = thread_rng();
        for _ in 0..20 {
            let digit = rng.gen_range(0..10);
            let random_str = format!("{}", digit);
            ret += &random_str;
        }
        ret
    }

    fn get_hash(msg: &String) -> String {
        let mut hasher = DefaultHasher::new();
        msg.hash(&mut hasher);
        let num = hasher.finish();
        format!("{:x}", num)
    }

    pub fn from_value(high_score: u64) -> Self {
        let high_score_str = format!("{}", high_score);
        let salt = SaveData::get_random_hash();
        let salt_and_hs = salt.clone() + &high_score_str;
        let hash = SaveData::get_hash(&salt_and_hs);

        return Self {
            high_score,
            high_score_salt: salt,
            high_score_hash: hash
        }
    }

    pub fn save_value(high_score: u64) {
        let save = SaveData::from_value(high_score);
        store(APP_NAME, save).unwrap();
    }

    pub fn assert_hs(&self) -> u64 {
        let high_score_str = format!("{}", self.high_score);
        let salt_and_hs = self.high_score_salt.clone() + &high_score_str;
        let test_hash = SaveData::get_hash(&salt_and_hs);

        if self.high_score_hash != test_hash {
            0
        } else {
            self.high_score
        }
    }
}

impl Default for SaveData {
    fn default() -> Self {
        SaveData::from_value(0)
    }
}
