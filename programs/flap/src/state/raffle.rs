use anchor_lang::prelude::*;
use crate::MAX_ENTRANTS;

#[account(zero_copy)]
pub struct Raffle {
    pub ticket_cnt: u16,
    pub ticket_max: u16,
    pub cnt_entrants: u16,
    pub entrants: [Pubkey; MAX_ENTRANTS],
}

#[account]
pub struct RaffleInfo {
    pub timestamp_start: i64,
    pub timestampe_end: i64,
    pub ticket_price: u64,
    pub winner: Pubkey,
    pub claimed: bool,
    pub prize_amount: u64,
    pub raffle_type: u8, // 0: weekly, 1: monthly, 2: random
}

impl Raffle {
    pub fn append(&mut self, buyer: Pubkey, amount: u16) {
        for i in self.ticket_cnt..(self.ticket_cnt + amount) {
            self.entrants[i as usize] = buyer;
        }
        self.entrants[self.cnt_entrants as usize] = buyer;
        self.cnt_entrants += amount;
    }
}
