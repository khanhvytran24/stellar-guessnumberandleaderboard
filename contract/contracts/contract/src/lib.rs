#![no_std]

use soroban_sdk::{
    contract, contractimpl, symbol_short, Address, Env, Map, Vec, Symbol
};

#[contract]
pub struct GuessGameWithLeaderboard;

#[contractimpl]
impl GuessGameWithLeaderboard {

    // 🎯 INIT GAME
    pub fn init_game(env: Env, admin: Address, secret: i32) {
        admin.require_auth();

        env.storage().instance().set(&symbol_short!("ADMIN"), &admin);
        env.storage().instance().set(&symbol_short!("SECRET"), &secret);
    }

    // 🎮 GUESS NUMBER
    pub fn guess(env: Env, player: Address, number: i32) -> Symbol {
        player.require_auth();

        let secret: i32 = env
            .storage()
            .instance()
            .get(&symbol_short!("SECRET"))
            .unwrap();

        let diff = number - secret;

        // 🎯 Correct → cộng điểm
        if diff == 0 {
            Self::add_score(env.clone(), player.clone(), 10);
            return symbol_short!("correct");
        }

        // 🔥 Gần đúng
        if diff.abs() <= 5 {
            if number < secret {
                return symbol_short!("close_hi");
            } else {
                return symbol_short!("close_lo");
            }
        }

        // 🔼 / 🔽
        if number < secret {
            symbol_short!("higher")
        } else {
            symbol_short!("lower")
        }
    }

    // 🏆 ADD SCORE
    fn add_score(env: Env, player: Address, points: i32) {
        let key = symbol_short!("SCORES");

        let mut scores: Map<Address, i32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let current = scores.get(player.clone()).unwrap_or(0);

        scores.set(player, current + points);

        env.storage().instance().set(&key, &scores);
    }

    // 📊 GET SCORE
    pub fn get_score(env: Env, player: Address) -> i32 {
        let key = symbol_short!("SCORES");

        let scores: Map<Address, i32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        scores.get(player).unwrap_or(0)
    }

    // 🥇 GET LEADERBOARD
    pub fn get_top(env: Env, limit: u32) -> Vec<(Address, i32)> {
        let key = symbol_short!("SCORES");

        let scores: Map<Address, i32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let mut list: Vec<(Address, i32)> = Vec::new(&env);

        // map → list
        for (addr, score) in scores.iter() {
            list.push_back((addr, score));
        }

        // sort giảm dần
        let len = list.len();
        for i in 0..len {
            for j in 0..len - 1 {
                let a = list.get(j).unwrap();
                let b = list.get(j + 1).unwrap();

                if a.1 < b.1 {
                    list.set(j, b.clone());
                    list.set(j + 1, a.clone());
                }
            }
        }

        // lấy top N
        let mut result = Vec::new(&env);
        let max = if limit > list.len() { list.len() } else { limit };

        for i in 0..max {
            result.push_back(list.get(i).unwrap());
        }

        result
    }
}
stellar contract invoke \
  --id CBWWZZUVPGH43E5QCYJ4VTKGYJTB3E67IYRXOSYPIHU5F7RJPCYEKND2 \
  --source student \
  --network testnet \
  -- \
  init_game \
  --admin student \
  --secret 73

  stellar contract invoke \
    --id CBWWZZUVPGH43E5QCYJ4VTKGYJTB3E67IYRXOSYPIHU5F7RJPCYEKND2 \
  --source student \
  --network testnet \
  -- \
  guess \
  --player student \
  --number 50
  stellar contract invoke \
    --id CBWWZZUVPGH43E5QCYJ4VTKGYJTB3E67IYRXOSYPIHU5F7RJPCYEKND2 \
  --source student \
  --network testnet \
  -- \
  get_top \
  --limit 5