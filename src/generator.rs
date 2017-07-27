use rand;
use rand::distributions::{IndependentSample, Range};
use std::cmp::{min, max};

use {Account, AccountId};

/// Randomly moves money between all the accounts
pub struct RandomTransactions<'a, R> {
    accounts: &'a mut [Account],
    range: Range<usize>,
    rng: R,
}

impl<'a, R> RandomTransactions<'a, R> {
    pub fn new(accounts: &'a mut [Account], rng: R) -> Self {
        let len = accounts.len();
        Self {
            accounts,
            range: Range::new(0, len),
            rng,
        }
    }
}

impl<'a, R> Iterator for RandomTransactions<'a, R>
where
    R: rand::Rng,
{
    type Item = (AccountId, AccountId, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let a_idx = self.range.ind_sample(&mut self.rng);
            let b_idx = self.range.ind_sample(&mut self.rng);

            if a_idx == b_idx { continue }

            let lo_idx = min(a_idx, b_idx);
            let hi_idx = max(a_idx, b_idx);

            let (left, right) = self.accounts.split_at_mut(hi_idx);

            let a = &mut left[lo_idx];
            let b = &mut right[0];

            let (src, dest) = if self.rng.gen() { (a, b) } else { (b, a) };

            if src.balance == 0 { continue }
            let money = self.rng.gen_range(0, src.balance);
            if money == 0 { continue }

            src.balance -= money;
            dest.balance += money;

            return Some((src.id.clone(), dest.id.clone(), money));
        }
    }
}

/// Slowly moves all the money from the winner (first account) to the
/// loser (second account)
pub struct WinnerLoser<'a, R> {
    winner: &'a mut Account,
    loser: &'a mut Account,
    rng: R,
}

impl<'a, R> WinnerLoser<'a, R> {
    pub fn new(accounts: &'a mut [Account], rng: R) -> Self {
        let (left, right) = accounts.split_at_mut(1);
        Self {
            winner: &mut left[0],
            loser: &mut right[0],
            rng,
        }
    }
}

impl<'a, R> Iterator for WinnerLoser<'a, R>
where
    R: rand::Rng,
{
    type Item = (AccountId, AccountId, u64);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.loser.balance < 49_000 { return None }
            let max = self.loser.balance / 10000;
            let money = self.rng.gen_range(0, max);

            if money == 0 { continue }

            self.loser.balance -= money;
            self.winner.balance += money;

            return Some((self.loser.id.clone(), self.winner.id.clone(), money));
        }
    }
}
