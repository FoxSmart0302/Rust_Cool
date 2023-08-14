// Need to be able to:
//  - add new keys
//  - remove invalid keys
//  -

use crate::new_key_cache::SHUFFLE_EVERY;
use governor::clock::DefaultClock;
use governor::state::keyed::DefaultKeyedStateStore;
use governor::{Quota, RateLimiter};
use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::num::NonZeroU32;
use std::time::Duration;

/// The reduction in documented API request rates in order to try and accomodate
/// for clock issues, etc.
const REQUESTS_SAFETY_BUFFER: f64 = 0.05;

pub struct KeyLimiter {
    /// A vec of the selling_partner_key_ids
    keys: Vec<i64>,
    rates: RateLimiter<i64, DefaultKeyedStateStore<i64>, DefaultClock>,
    last: usize,
    cnt: usize,
}

impl KeyLimiter {
    pub fn new(
        selling_partner_key_ids: Vec<i64>,
        requests_per_second: f64,
        burst_rate: usize,
    ) -> Self {
        let burst = NonZeroU32::new(burst_rate as u32).unwrap();

        let replenish_dur =
            Duration::from_secs_f64(1.0 / (requests_per_second * (1.0 - REQUESTS_SAFETY_BUFFER)));
        let quota = Quota::with_period(replenish_dur)
            .unwrap()
            .allow_burst(burst);

        let mut t = Self {
            keys: selling_partner_key_ids,
            rates: RateLimiter::keyed(quota),
            last: 0,
            cnt: 0,
        };
        t.shuffle();

        t
    }

    /// Adds the given selling_partner_key_id to the keys vec.
    ///
    /// This is meant to be called when a new set of keys is
    /// added to the database, but it is not yet wired in.
    #[allow(dead_code)]
    pub fn add(&mut self, selling_partner_key_id: i64) {
        if !self.keys.contains(&selling_partner_key_id) {
            self.keys.push(selling_partner_key_id);
        }
    }

    pub fn remove(&mut self, selling_partner_key_id: i64) {
        // Remove the given id from the keys vec
        self.keys.retain(|x| *x != selling_partner_key_id);

        // Reset the last and cnt, so we aren't pointing to an invalid index
        self.last = 0;
        self.cnt = 0;
        self.shuffle();

        // We don't have any way to remove the invalid key from the rates,
        // but it shouldn't be accessed anymore, since access is controlled
        // via the keys vec.
    }

    pub fn shuffle(&mut self) {
        self.keys.shuffle(&mut thread_rng());
    }

    /// Returns the next selling_partner_key_id that can use n tokens
    ///
    /// If no keys are available, returns None
    pub fn take(&mut self, n: NonZeroU32) -> Option<i64> {
        if n == NonZeroU32::new(1).unwrap() {
            for i in 0..self.keys.len() {
                let index = (i + self.last) % self.keys.len();
                let id = self.keys[index];

                if let Ok(()) = self.rates.check_key(&id) {
                    self.cnt += 1;
                    if self.cnt > SHUFFLE_EVERY {
                        self.shuffle();
                        self.cnt = 0;
                    }

                    self.last = index + 1;
                    return Some(id);
                }
            }
        } else {
            for i in 0..self.keys.len() {
                let index = (i + self.last) % self.keys.len();
                let id = self.keys[index];

                if let Ok(()) = self.rates.check_key_n(&id, n) {
                    self.cnt += 1;
                    if self.cnt > SHUFFLE_EVERY {
                        self.shuffle();
                        self.cnt = 0;
                    }

                    self.last = index + 1;
                    return Some(id);
                }
            }
        }

        None
    }
}
