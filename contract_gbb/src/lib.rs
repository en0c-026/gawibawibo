use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, log, near_bindgen, AccountId, PanicOnDefault, Promise};

struct Game {
    move_owner: GBB,
    move_opp: GBB,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum GBB {
    Gawi,
    Bawi,
    Bo,
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
    blend: Vec<GBB>,
    prize: U128,
    adversary: Option<AccountId>,
    winner: Option<AccountId>,
}

impl Move {
    pub fn new(id: u32, owner: AccountId, blend: Vec<GBB>, prize: U128) -> Self {
        Self {
            id,
            owner,
            status: MoveStatus::Unplayed,
            blend,
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
    pub fn new_move(&mut self, id: u32, blend: Vec<GBB>) {
        let account_id = env::predecessor_account_id();
        let attached_amount = env::attached_deposit();
        assert!(self.moves.get(&id).is_none(), "MOVE_ID_ALREADY_EXISTS");
        let new_move = Move::new(id, account_id, blend, U128(attached_amount));
        self.moves.insert(&id, &new_move);
        self.unplayed_moves.insert(&id);
    }
    pub fn moves_of(&self, account_id: AccountId) -> Vec<Move> {
        let values = self
            .moves
            .values()
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
    pub fn play_move(&mut self, id: &u32, blend_op: Vec<GBB>) {
        assert!(
            self.unplayed_moves.contains(id),
            "MOVE_DONT_EXISTS_OR_PLAYED"
        );
        let attached_amount = env::attached_deposit();
        let opp_account_id = env::predecessor_account_id();
        let mut target_move = self.moves.get(id).unwrap();
        if target_move.owner == opp_account_id {
            env::panic_str("Move owner and opponent cannot be the same");
        }
        assert_eq!(
            target_move.prize,
            U128(attached_amount),
            "DEPOSIT_MUST_EQUAL_PRIZE"
        );
        target_move.adversary = Some(opp_account_id.clone());

        let mut counter_game = [0, 0];

        for (i, val) in target_move.blend.iter().enumerate() {
            let move_owner = val.clone();
            let move_opp = blend_op[i].clone();
            let g = Game {
                move_owner,
                move_opp,
            };
            match g {
                Game {
                    move_owner: GBB::Gawi,
                    move_opp: GBB::Bo,
                } => counter_game[0] += 1,
                Game {
                    move_owner: GBB::Bawi,
                    move_opp: GBB::Gawi,
                } => counter_game[0] += 1,
                Game {
                    move_owner: GBB::Bo,
                    move_opp: GBB::Bawi,
                } => counter_game[0] += 1,
                Game {
                    move_owner: GBB::Bo,
                    move_opp: GBB::Gawi,
                } => counter_game[1] += 1,
                Game {
                    move_owner: GBB::Gawi,
                    move_opp: GBB::Bawi,
                } => counter_game[1] += 1,
                Game {
                    move_owner: GBB::Bawi,
                    move_opp: GBB::Bo,
                } => counter_game[1] += 1,
                _ => (),
            }
        }

        if counter_game[0] == counter_game[1] {
            self.unplayed_moves.remove(id);
            target_move.status = MoveStatus::Tied;
            self.moves.insert(id, &target_move);
            self.set_unclaimed_amount(&target_move.owner, target_move.prize);
            self.set_unclaimed_amount(&opp_account_id, U128(attached_amount));

            log!("The result of this game, is a tie! Move canceled and funds refunded.");
        } else {
            target_move.prize = U128(u128::from(target_move.prize).wrapping_add(attached_amount));
            if counter_game[0] > counter_game[1] {
                target_move.winner = Some(target_move.owner.clone());
            } else {
                target_move.winner = Some(opp_account_id.clone());
            }
            target_move.status = MoveStatus::Played;
            self.moves.insert(id, &target_move);
            self.set_unclaimed_amount(&target_move.winner.unwrap(), target_move.prize);
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
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, VMContext};

    fn get_context(predecessor: AccountId, amount: Option<u128>, is_view: bool) -> VMContext {
        match amount {
            Some(amnt) => VMContextBuilder::new()
                .signer_account_id(predecessor)
                .attached_deposit(amnt)
                .build(),
            None => VMContextBuilder::new()
                .signer_account_id(predecessor)
                .is_view(is_view)
                .build(),
        }
    }
    fn deploy_contract() -> GawiBawiBo {
        GawiBawiBo::new(AccountId::new_unchecked("gawibawibo.testnet".to_string()))
    }
    fn get_owner_and_amount() -> (AccountId, u128) {
        let owner = AccountId::new_unchecked("en0c-player.tesnet".to_string());
        let amount = 1000000000000000000000000;
        (owner, amount)
    }
    #[test]
    fn create_new_move() {
        let (owner, amount) = get_owner_and_amount();
        let context = get_context(owner, Some(amount), false);
        testing_env!(context);
        let mut contract = deploy_contract();
        contract.new_move(1, vec![GBB::Bo, GBB::Gawi, GBB::Gawi]);
    }
}
