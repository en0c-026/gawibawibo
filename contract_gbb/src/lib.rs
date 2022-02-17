use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};


fn get_b_nums (h: String, bn: &u64) -> Vec<u64> {
  let n_blend: Vec<u64> = vec![147, 258, 369];

  match h.as_str() {
    "fa0c01259f28be0e8e306f38ab3b29723fb4d55a48dc654f83b11d1eba61962f" => {
      vec![n_blend[0] * bn, n_blend[0] * bn, n_blend[0] * bn]
    },
    "44dc982c19c9d9dca8181e2dcfa6a830143e8f65e31a77899c337368321f8a37" => {
      vec![n_blend[1] * bn, n_blend[0] * bn, n_blend[0] * bn]
    },
    "e41282ff8c095dda4fe837c81a497ccb843ab7d0c6d8f36e7299c0ed0361ca6d" => {
      vec![n_blend[2] * bn, n_blend[0] * bn, n_blend[0] * bn]
    },
    "47e0aec339f6df2d4eb14bdbe77668d3e21aff6c0eb7c328a466addfc37c81c1" => {
      vec![n_blend[0] * bn, n_blend[1] * bn, n_blend[0] * bn]
    },
    "3b4a70b36764cfc016cacdb8097c85fcb21c38fe1e7c835d72db2c54b88ce161" => {
      vec![n_blend[0] * bn, n_blend[2] * bn, n_blend[0] * bn]
    },
    "000800590e60416392e917a0769a38250157669b1b6e2d43a5ce92dce7189ea8" => {
      vec![n_blend[0] * bn, n_blend[0] * bn, n_blend[1] * bn]
    },
    "fca723e9b333b3adc65cd365ae84bf5af940d12016fdfc8d04ebcf099a6b4e4a" => {
      vec![n_blend[0] * bn, n_blend[0] * bn, n_blend[2] * bn]
    },
    "2c116d02a0b40a5f5d8e42105d424daf086921fd814790ae7a06c913f070e910" => {
      vec![n_blend[1] * bn, n_blend[1] * bn, n_blend[1] * bn]
    },
    "d0a8563d5354a25399c537f8d7fc7d23d1dad3377e7b0d267dcdf61707a79bc8" => {
      vec![n_blend[0] * bn, n_blend[1] * bn, n_blend[1] * bn]
    },
    "6dab46bb11f021e53f0e42a02843dcc405029757ab5eb2d2964ec47f237b20d5" => {
      vec![n_blend[2] * bn, n_blend[1] * bn, n_blend[1] * bn]
    },
    "6556148c9afafb9477ff63904af5eef2dd0eb7aeccc87239cf829c5544d92e3f" => {
      vec![n_blend[1] * bn, n_blend[0] * bn, n_blend[1] * bn]
    },
    "ab862c70f31955a6b7a13052b6d51a7caa293a5b3cc98027b568c3ee355a7656" => {
      vec![n_blend[1] * bn, n_blend[2] * bn, n_blend[1] * bn]
    },
    "db8917d105fddf904e6cce0cc810762e824fb21f680ef17260008e735e33a7ea" => {
      vec![n_blend[1] * bn, n_blend[1] * bn, n_blend[0] * bn]
    },
    "e2e4b1bf60e66c63e7de4d8640c6619600616861b7f7e07874b88f23b9040afb" => {
      vec![n_blend[1] * bn, n_blend[1] * bn, n_blend[2] * bn]
    },
    "ada9adc945a4a8ee19e81c15a2c1e9e05d2678e52ee785edefbe76c7ade21876" => {
      vec![n_blend[2] * bn, n_blend[2] * bn, n_blend[2] * bn]
    },
    "0c94883eb82fa680c11daadbe6e6405dfe10a2c37d5795c5c7aabb4090fd2377" => {
      vec![n_blend[0] * bn, n_blend[2] * bn, n_blend[2] * bn]
    },
    "1971c0ac93f5c80dade4a8aa0ba04fe477acc3de66b3b75c0411e4158a27c63b" => {
      vec![n_blend[1] * bn, n_blend[2] * bn, n_blend[2] * bn]
    },
    "bb3f7e1fc680cad49db000e0f8623a836971e3d032abfcbfca8a7780eef4c436" => {
      vec![n_blend[2] * bn, n_blend[0] * bn, n_blend[2] * bn]
    },
    "02f7e167ad84c7592a64e527fad353f4bafd24211404ef46c1c058f2b31f0972" => {
      vec![n_blend[2] * bn, n_blend[1] * bn, n_blend[2] * bn]
    },
    "e8335c0f96176b48c4095476243d5640f55e673f485fd1186b3d80681ebc90e4" => {
      vec![n_blend[2] * bn, n_blend[2] * bn, n_blend[0] * bn]
    },
    "fbf1a3305ea72211175fa7a19d7c21e2085b1c7b54bc47ba4e6c2d6e82bc2fa5" => {
      vec![n_blend[2] * bn, n_blend[2] * bn, n_blend[1] * bn]
    },
    "1d521f4b205cc932b953b50d4295206068d23045524c2a9962314b3f4d03c300" => {
      vec![n_blend[0] * bn, n_blend[1] * bn, n_blend[2] * bn]
    },
    "24a22c07845efb885b970a8d4ec4d7667d8a40ae88684170b1a797b4c1f2271f" => {
      vec![n_blend[2] * bn, n_blend[1] * bn, n_blend[0] * bn]
    },
    _ => vec![0, 0 ,0]
  }
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum MoveStatus {
    Unplayed,
    Played,
    Cancelled,
    Tied,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct MoveUnplayed {
    id: u32,
    owner: AccountId,
    prize: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Move {
    id: u32,
    owner: AccountId,
    status: MoveStatus,
    hb: String,
    bn: Option<u64>,
    prize: U128,
    adversary: Option<AccountId>,
    winner: Option<AccountId>,
}

impl Move {
    pub fn new(id: u32, owner: AccountId, hb: String, bn: Option<u64>, prize: U128) -> Self {
        Self {
            id,
            owner,
            status: MoveStatus::Unplayed,
            hb,
            bn,
            prize,
            adversary: None,
            winner: None,
        }
    }
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct GawiBawiBo {
    owner_id: AccountId,
    moves: UnorderedMap<u32, Move>,
    unplayed_moves: UnorderedSet<u32>,
    unclaimed_amount: LookupMap<AccountId, U128>,
}

#[near_bindgen]
impl GawiBawiBo {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            moves: UnorderedMap::new(b"m"),
            unplayed_moves: UnorderedSet::new(b"u"),
            unclaimed_amount: LookupMap::new(b"a"),
        }
    }
    #[payable]
    pub fn new_move(&mut self, id: u32, hb: String) {
        let account_id = env::predecessor_account_id();
        let attached_amount = env::attached_deposit();
        assert!(self.moves.get(&id).is_none(), "MOVE_ID_ALREADY_EXISTS");
        let bn = env::block_height();
        let new_move = Move::new(id, account_id, hb, Some(bn), U128(attached_amount));
        self.moves.insert(&id, &new_move);
        self.unplayed_moves.insert(&id);
    }
    pub fn moves_of(&self, account_id: AccountId) -> Vec<Move> {
        let values = self
            .moves
            .values()
            .map(|m| Move {
              id: m.id,
              owner: m.owner,
              status: m.status,
              hb: m.hb,
              bn: None,
              prize: m.prize,
              adversary: m.adversary,
              winner: m.winner,
            })
            .filter(|m| m.owner == account_id || m.winner == Some(account_id.clone()) || m.adversary == Some(account_id.clone()))
            .filter(|m| m.status == MoveStatus::Played || m.status == MoveStatus::Cancelled || m.status == MoveStatus::Tied)
            .collect();
        values
    }
    pub fn cancel_move(&mut self, id: &u32) {
        assert!(
            self.unplayed_moves.contains(&id),
            "MOVE_DONT_EXISTS_OR_PLAYED"
        );
        let account_id = env::predecessor_account_id();
        let mut target_move = self.moves.get(id).unwrap();
        assert_eq!(target_move.owner, account_id, "UNAHUTORIZED_ACTION");
        assert_eq!(target_move.status, MoveStatus::Unplayed, "DONT_CANCEL_MOVE");
        assert!(target_move.winner.is_none(), "DONT_CANCEL_MOVE");
        target_move.status = MoveStatus::Cancelled;
        self.moves.insert(id, &target_move);
        self.set_unclaimed_amount(&target_move.owner, target_move.prize);
    }
    pub fn get_unplayed_moves(&self) -> Vec<MoveUnplayed> {
        let values = self
            .moves
            .values()
            .filter(|m| m.status == MoveStatus::Unplayed)
            .map(|m| MoveUnplayed {
                id: m.id,
                owner: m.owner,
                prize: m.prize,
            })
            .collect();
        values
    }
    #[payable]
    pub fn play_move(&mut self, id: &u32, ha: String) {
        assert!(
            self.unplayed_moves.contains(id),
            "MOVE_DONT_EXISTS_OR_PLAYED"
        );
        let attached_amount = env::attached_deposit();
        let adv_account_id = env::predecessor_account_id();
        let target_move = self.moves.get(id).unwrap();
        
        if target_move.owner == adv_account_id {
          env::panic_str("Move owner and opponent cannot be the same");
        }
        assert_eq!(target_move.status, MoveStatus::Unplayed, "MOVE_PLAYED");
        assert_eq!(target_move.prize, U128(attached_amount), "DEPOSIT_MUST_EQUAL_PRIZE");
        
        let mut move_updated = Move::new(
          target_move.id.clone(),
          target_move.owner.clone(),
          target_move.hb.clone(),
          target_move.bn.clone(),
          target_move.prize.clone(),
        );
        
        move_updated.adversary = Some(adv_account_id.clone());

        let mut counter_game = [0, 0];
        let b_o = get_b_nums(target_move.hb, &target_move.bn.unwrap());
        let b_a = get_b_nums(ha, &target_move.bn.unwrap());


        for (i, val) in b_o.iter().enumerate() {
            let op_o = val.clone() / target_move.bn.unwrap();
            let op_a = b_a[i].clone() / target_move.bn.unwrap();

            let g = [op_o, op_a];

            match g {
                [147, 369] => counter_game[0] += 1,
                [258, 147] => counter_game[0] += 1,
                [369, 258] => counter_game[0] += 1,
                [369, 147] => counter_game[1] += 1,
                [147, 258] => counter_game[1] += 1,
                [258, 369] => counter_game[1] += 1,
                [147, 147] => (),
                [258, 258] => (),
                [369, 369] => (),
                _ => (),
            }
        }

        if counter_game[0] == counter_game[1] {
            self.unplayed_moves.remove(id);
            move_updated.status = MoveStatus::Tied;
            self.moves.insert(id, &move_updated);

            self.set_unclaimed_amount(&target_move.owner, target_move.prize);
            self.set_unclaimed_amount(&adv_account_id, U128(attached_amount));

            log!("The result of this game, is a tie! Move canceled and funds refunded.");
        } else {
            move_updated.prize = U128(u128::from(target_move.prize).wrapping_add(attached_amount));
            if counter_game[0] > counter_game[1] {
                move_updated.winner = Some(target_move.owner.clone());
            } else {
                move_updated.winner = Some(adv_account_id.clone());
            }
            move_updated.status = MoveStatus::Played;
            self.moves.insert(id, &move_updated);
            self.set_unclaimed_amount(&move_updated.winner.unwrap(), move_updated.prize);
        }
    }
    #[private]
    pub fn set_unclaimed_amount(&mut self, claimant: &AccountId, amount: U128) {
        let prev_amount = u128::from(self.unclaimed_amount_of(&claimant));
        self.unclaimed_amount.insert(
            claimant,
            &U128(prev_amount.wrapping_add(u128::from(amount))),
        );
    }
    pub fn unclaimed_amount_of(&self, account_id: &AccountId) -> U128 {
        self.unclaimed_amount.get(&account_id).unwrap_or(U128(0))
    }
    pub fn withdraw(&mut self) -> Promise {
        let claimant = env::predecessor_account_id();
        let unclaim_amount = u128::from(self.unclaimed_amount_of(&claimant));
        if unclaim_amount > 0 {
            self.unclaimed_amount.remove(&claimant);
            Promise::new(claimant).transfer(unclaim_amount)
        } else {
            env::panic_str("Account does not have an amount to claim");
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {}