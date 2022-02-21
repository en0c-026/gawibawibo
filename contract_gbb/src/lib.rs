use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};
use hex::encode;
use std::convert::TryFrom;


fn get_b_nums (pk: String, hb: String) -> Vec<u64> {
  let n_blend: Vec<u64> = vec![147, 258, 369];
  let ops = vec![
  "a8241dee1b",
  "9c3ef738a5",
  "9c4bea65e9",
  "038e16cdf9",
  "481ddbfbe9",
  "69343e02fa",
  "1af577cdd3",
  "f070fae536",
  "686690db1f",
  "f611d744c5",
  "444c34bf26",
  "5463570d5e",
  "8aa9d396ea",
  "76a27a71ee",
  "41f3de6eed",
  "d999fb7fe8",
  "8792ba8121",
  "68d0ea14ef",
  "c11af4a478",
  "823f07b380",
  "4432146540",
  "0f1514d671",
  "62d63583cb"
  ];

  let ops_h: Vec<String> = ops
  .clone()
  .into_iter()
  .map(|o| {
    let path = format!("{}{}", o, pk);
    let hash_path = env::sha256(path.as_bytes());
    encode(hash_path)
  })
  .collect();


  let mut pattern = "";

  for (i, o) in ops_h.clone().into_iter().enumerate() {
    if o == hb {
      pattern = ops[i];
    }
  }

  match pattern {
    "a8241dee1b" => {
      vec![n_blend[0], n_blend[0], n_blend[0]]
    },
    "9c3ef738a5" => {
      vec![n_blend[1], n_blend[0], n_blend[0]]
    },
    "9c4bea65e9" => {
      vec![n_blend[2], n_blend[0], n_blend[0]]
    },
    "038e16cdf9" => {
      vec![n_blend[0], n_blend[1], n_blend[0]]
    },
    "481ddbfbe9" => {
      vec![n_blend[0], n_blend[2], n_blend[0]]
    },
    "69343e02fa" => {
      vec![n_blend[0], n_blend[0], n_blend[1]]
    },
    "1af577cdd3" => {
      vec![n_blend[0], n_blend[0], n_blend[2]]
    },
    "f070fae536" => {
      vec![n_blend[1], n_blend[1], n_blend[1]]
    },
    "686690db1f" => {
      vec![n_blend[0], n_blend[1], n_blend[1]]
    },
    "f611d744c5" => {
      vec![n_blend[2], n_blend[1], n_blend[1]]
    },
    "444c34bf26" => {
      vec![n_blend[1], n_blend[0], n_blend[1]]
    },
    "5463570d5e" => {
      vec![n_blend[1], n_blend[2], n_blend[1]]
    },
    "8aa9d396ea" => {
      vec![n_blend[1], n_blend[1], n_blend[0]]
    },
    "76a27a71ee" => {
      vec![n_blend[1], n_blend[1], n_blend[2]]
    },
    "41f3de6eed" => {
      vec![n_blend[2], n_blend[2], n_blend[2]]
    },
    "d999fb7fe8" => {
      vec![n_blend[0], n_blend[2], n_blend[2]]
    },
    "8792ba8121" => {
      vec![n_blend[1], n_blend[2], n_blend[2]]
    },
    "68d0ea14ef" => {
      vec![n_blend[2], n_blend[0], n_blend[2]]
    },
    "c11af4a478" => {
      vec![n_blend[2], n_blend[1], n_blend[2]]
    },
    "823f07b380" => {
      vec![n_blend[2], n_blend[2], n_blend[0]]
    },
    "4432146540" => {
      vec![n_blend[2], n_blend[2], n_blend[1]]
    },
    "0f1514d671" => {
      vec![n_blend[0], n_blend[1], n_blend[2]]
    },
    "62d63583cb" => {
      vec![n_blend[2], n_blend[1], n_blend[0]]
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
    id: String,
    owner: AccountId,
    prize: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Deserialize, Serialize, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Move {
    id: String,
    owner: AccountId,
    status: MoveStatus,
    hb: String,
    pk: Option<String>,
    prize: U128,
    adversary: Option<AccountId>,
    winner: Option<AccountId>,
}

impl Move {
    pub fn new(id: String, owner: AccountId, hb: String, pk: Option<String>, prize: U128) -> Self {
        Self {
            id,
            owner,
            status: MoveStatus::Unplayed,
            hb,
            pk,
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
    moves: UnorderedMap<String, Move>,
    unplayed_moves: UnorderedSet<String>,
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
    pub fn new_move(&mut self, id: String, hb: String) {
        let account_id = env::predecessor_account_id();
        let attached_amount = env::attached_deposit();
        assert!(self.moves.get(&id).is_none(), "MOVE_ID_ALREADY_EXISTS");
        let pk_bytes = env::signer_account_pk();
        let pk_string = String::try_from(&pk_bytes).unwrap();
        let new_move = Move::new(id.clone(), account_id, hb, Some(pk_string), U128(attached_amount));
        self.moves.insert(&id, &new_move);
        self.unplayed_moves.insert(&id);
    }
    pub fn moves_of(&self, account_id: AccountId) -> Vec<Move> {
        let values = self
            .moves
            .values()
            .map(|m| {
              Move {
                id: m.id,
                owner: m.owner,
                status: m.status,
                hb: m.hb,
                pk: None,
                prize: m.prize,
                adversary: m.adversary,
                winner: m.winner
              }
            })
            .filter(|m| m.owner == account_id || m.winner == Some(account_id.clone()) || m.adversary == Some(account_id.clone()))
            .filter(|m| m.status == MoveStatus::Played || m.status == MoveStatus::Cancelled || m.status == MoveStatus::Tied)
            .collect();
        values
    }
    pub fn cancel_move(&mut self, id: &String) {
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
    pub fn play_move(&mut self, id: &String, ha: String) {
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
        let a_pk_bytes = env::signer_account_pk();
        let a_pk_string = String::try_from(&a_pk_bytes).unwrap();
        let mut move_updated = Move::new(
          target_move.id.clone(),
          target_move.owner.clone(),
          target_move.hb.clone(),
          target_move.pk.clone(),
          target_move.prize.clone(),

        );
        
        move_updated.adversary = Some(adv_account_id.clone());

        let mut counter_game = [0, 0];
        let b_o = get_b_nums(target_move.pk.unwrap(), target_move.hb);
        let b_a = get_b_nums(a_pk_string, ha);


        for (i, val) in b_o.iter().enumerate() {
            let op_o = val.clone();
            let op_a = b_a[i].clone();

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