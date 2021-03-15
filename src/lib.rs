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
#![allow(missing_docs)]
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

#[derive(Debug, Default)]
pub struct TimeMachine<S: Clone> {
    edges: HashMap<Time, S>,
}
impl<S: Clone + Default> TimeMachine<S> {
    pub fn add_state(&mut self, start_time: Time, end_time: Time, state: S) {
        self.add_transition(start_time, state);
        if !self.edges.contains_key(&end_time) {
            self.add_transition(end_time, S::default());
        }
    }
    #[cfg(feature = "napchart")]
    pub fn from_napchart<F>(lane: &napchart::ChartLane, mapping: F) -> Self
    where
        F: Fn(&napchart::ChartElement) -> Option<S>,
    {
        let mut tm = Self::new();
        for elem in lane.elements.iter() {
            if let Some(mapped) = mapping(elem) {
                tm.add_state(
                    Time::from_minutes(elem.start),
                    Time::from_minutes(elem.end),
                    mapped,
                );
            }
        }
        tm
    }
}
impl<S: Clone> TimeMachine<S> {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }
    pub fn add_transition(&mut self, time: Time, state: S) {
        self.edges.insert(time, state);
    }
    pub fn get_state(&self, time: &Time) -> Result<S> {
        if self.edges.is_empty() {
            return Err(Error::EmptyTimeMachine);
        }
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
