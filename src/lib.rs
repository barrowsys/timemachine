/*
 * --------------------
 * THIS FILE IS LICENSED UNDER MIT
 * THE FOLLOWING MESSAGE IS NOT A LICENSE
 *
 * <barrow@tilde.team> wrote this file.
 * by reading this text, you are reading "TRANS RIGHTS".
 * this file and the content within it is the gay agenda.
 * if we meet some day, and you think this stuff is worth it,
 * you can buy me a beer, tea, or something stronger.
 * -Ezra Barrow
 * --------------------
 */
#![warn(missing_docs)]
//! Temporary Crate Docs

use std::collections::HashMap;

mod time;
pub use time::Clock;
#[doc(inline)]
pub use time::Time;

pub enum Error {
    Generic(String),
    EmptyTimeMachine,
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct TimeMachine<S: Clone> {
    edges: HashMap<Time, S>,
}
impl<S: Clone> TimeMachine<S> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    pub fn add_transition(&mut self, time: Time, new_state: S) {
        self.edges.insert(time, new_state);
    }
    pub fn get_state(&self, time: &Time) -> Result<S> {
        if self.edges.len() == 0 {
            return Err(Error::EmptyTimeMachine);
        }
        // let mut ret = self.edges.keys().next().unwrap().clone();
        let mut ret = Time::midnight();
        let mut lt = ret.clone();
        for edge in self.edges.keys() {
            if time >= edge && edge >= &ret {
                ret = edge.clone();
            }
            if edge > &lt {
                lt = edge.clone();
            }
        }
        // println!("{}, {}", time, ret);
        if let Some(state) = self.edges.get(&ret).cloned() {
            Ok(state)
        } else {
            Ok(self.edges.get(&lt).cloned().unwrap())
        }
    }
}
