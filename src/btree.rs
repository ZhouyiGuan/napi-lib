use std::{
    collections::BTreeMap,
    cell::RefCell,
    ops::Bound::{Included, Unbounded},
    time::{Duration, Instant},
};
use rand::Rng;

#[napi]
pub struct MyBTree {
    map: BTreeMap<i64, i64>,
}

#[napi]
impl MyBTree {
    #[napi(constructor)]
    pub fn new() -> Self {
        println!("BTree created");
        Self {
            map: BTreeMap::new(),
        }
    }

    #[napi(factory)]
    pub fn new_random(size: i64) -> Self {
        let mut btree = MyBTree::new();
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let random_num = rng.gen_range(0..(size*10));
            btree.insert(random_num, random_num);
        }
        println!("BTree created with {} random elements", size);
        btree
    }

    #[napi]
    pub fn insert(&mut self, key: i64, value: i64) {
        self.map.insert(key, value);
    }

    #[napi]
    pub fn find(&self, key: i64) -> Option<i64> {
        self.map.get(&key)
            .copied()
            .or_else(|| self.find_nearest_key(&key))
    }

    #[napi]
    pub fn remove(&mut self, key: i64) {
        self.map.remove(&key);
    }

    fn find_nearest_key(&self, key: &i64) -> Option<i64> {
        let mut lower = self.map.range((Unbounded, Included(key))).rev();
        let mut upper = self.map.range((Included(key), Unbounded));

        let lower_pair = lower.next();
        let upper_pair = upper.next();
        match (lower_pair, upper_pair) {
            (Some((lower_key, lower_value)), Some((upper_key, upper_value))) => {
                if (key-lower_key) < (upper_key-key) {
                    return Some(lower_value.to_owned());
                } else {
                    return Some(upper_value.to_owned());
                }
            },
            (Some((_, lower_value)), None) => {
                return Some(lower_value.to_owned());
            },
            (None, Some((_, upper_value))) => {
                return Some(upper_value.to_owned());
            },
            (None, None) => {
                return None;
            },
        }
    }
}