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

use timemachine::{Clock, Time, TimeMachine};

// fn tm_1(t: &Time, s: &Schedule) -> Option<State> {
//     fn state_from(n: usize) -> Option<State> {
//         match n {
//             0 => Some(State::DawnRed),
//             1 => Some(State::DayWhite),
//             2 => Some(State::DuskRed),
//             3 => Some(State::NightDark),
//             _ => None,
//         }
//     }
//     let times: Vec<&Time> = vec![&s.dawn.0, &s.dawn.1, &s.dusk.0, &s.dusk.1];
//     let mut r: Option<State> = None;
//     let mut st = Time::midnight();
//     let mut lt = 0;
//     for i in 0..4 {
//         let swtime = times[i];
//         if t >= swtime && swtime >= &st {
//             r = state_from(i);
//             st = swtime.clone();
//         }
//         if swtime > times[lt] {
//             lt = i;
//         }
//     }
//     if r.is_none() {
//         r = state_from(lt);
//     }
//     r
// }

// fn tm_2(t: &Time, s: &Schedule) -> Option<State> {
//     let mut tm = TimeMachine::<State>::new();
//     tm.add_transition(s.dawn.0.clone(), State::DawnRed);
//     tm.add_transition(s.dawn.1.clone(), State::DayWhite);
//     tm.add_transition(s.dusk.0.clone(), State::DuskRed);
//     tm.add_transition(s.dusk.1.clone(), State::NightDark);
//     tm.get_state(t).ok()
// }

// fn arduino_current(t: &Time, s: &Schedule) -> Option<State> {
//     let t = t.as_minutes();
//     let dawn1 = s.dawn.0.as_minutes();
//     let dawn2 = s.dawn.1.as_minutes();
//     let dusk1 = s.dusk.0.as_minutes();
//     let dusk2 = s.dusk.1.as_minutes();
//     let mut ndawn1 = dawn1;
//     let mut nt = t;
//     if dusk2 > dawn1 {
//         ndawn1 += 24*60;
//         nt += 24*60;
//     }
//     if t >= dawn1 && t < dawn2 {
//         Some(State::DawnRed)
//     } else if t >= dusk1 && t < dusk2 {
//         Some(State::DuskRed)
//     } else if (nt >= dusk2 && nt < ndawn1) || (t >= dusk2 && t < ndawn1) {
//         Some(State::NightDark)
//     } else {
//         Some(State::DayWhite)
//     }
// }

fn main() {
    //     let time1 = Time::new_h(7);
    //     let sched1 = Schedule::new(7, 0, 8, 0, 20, 30, 22, 0);
    //     println!("Hello, world!");
    //     println!("{:?}", time1);
    //     println!("{:?}", sched1);
    //     println!("{:#4?}", sched1);
    //     println!("");
    //     let scheds = vec![
    //         Schedule::new(7, 0, 8, 0, 20, 30, 22, 0),    // NightDark
    //         Schedule::new(7, 0, 8, 0, 18, 0, 22, 0),     // NightDark
    //         Schedule::new(7, 0, 8, 0, 23, 0, 23, 59),    // NightDark
    //         Schedule::new(7, 0, 8, 0, 1, 0, 1, 30),      // DayWhite

    //         // Midnight after dawn
    //         Schedule::new(22, 30, 23, 30, 7, 0, 8, 0),   // DayWhite
    //         Schedule::new(23, 0, 0, 0, 7, 0, 8, 0),      // DayWhite
    //         // Midnight during dawn
    //         Schedule::new(23, 30, 0, 30, 7, 0, 8, 0),    // DawnRed
    //         Schedule::new(0, 0, 1, 0, 7, 0, 8, 0),       // DawnRed
    //         // Midnight before dawn
    //         Schedule::new(0, 30, 1, 30, 7, 0, 8, 0),     // NightDark

    //         // Midnight after dusk
    //         Schedule::new(7, 0, 8, 0, 22, 30, 23, 30),   // NightDark
    //         Schedule::new(7, 0, 8, 0, 23, 0, 0, 0),      // NightDark
    //         // Midnight during dusk
    //         Schedule::new(7, 0, 8, 0, 23, 30, 0, 30),    // DuskRed
    //         Schedule::new(7, 0, 8, 0, 0, 0, 1, 0),       // DuskRed
    //         // Midnight before dusk
    //         Schedule::new(7, 0, 8, 0, 0, 30, 1, 30),     // DayWhite
    //     ];
    //     // println!("time_based_switching");
    //     // let test = mktest(arduino_current);
    //     // for sched in scheds.iter() {
    //     //     test(&sched);
    //     // }
    //     println!("\ntm_1");
    //     let test = mktest(tm_1);
    //     for sched in scheds.iter() {
    //         test(&sched);
    //     }
    //     println!("\ntm_2");
    //     let test = mktest(tm_2);
    //     for sched in scheds.iter() {
    //         test(&sched);
    //     }
}

// fn mktest(f: fn(&Time, &Schedule) -> Option<State>) -> Box<dyn Fn(&Schedule)> {
//     Box::new(move |s: &Schedule| {
//         let starting_state: Option<State> = s.has_time(&Time::midnight()).or_else(|| {
//             let mut laststate: Option<State> = None;
//             for t in Clock::minutes() {
//                 if let Some(state) = s.has_time(&t) {
//                     laststate = Some(state);
//                 }
//             }
//             laststate
//         });
//         let mut ref_state = starting_state.expect(&format!("NO VALID MIDNIGHT STATE FOR SCHEDULE {:?}", s));
//         println!("  - Testing {:?}", s);
//         let mut errs: Vec<String> = vec![];
//         for t in Clock::every(0, 15, 0) {
//             if let Some(state) = s.has_time(&t) {
//                 ref_state = state;
//             }
//             if let Some(test_state) = f(&t, s) {
//                 if ref_state != test_state {
//                     // errs.push(format!("      - {}, ref = {:?}, fn() = {:?}", t, ref_state, test_state));
//                     println!("      - {}, ref = {:?}, fn() = {:?}", t, ref_state, test_state);
//                 }
//             } else {
//                 // errs.push(format!("      - {}, ref = {:?}, fn() = NONE", t, ref_state));
//                 println!("      - {}, ref = {:?}, fn() = NONE", t, ref_state);
//             }
//         }
//         // if errs.len() == 0 {
//         //     println!("  - Success Testing {:?}", s);
//         // } else {
//         //     println!("  - Errors Testing {:?}", s);
//         //     for err in errs {
//         //         println!("{}", err);
//         //     }
//         // }
//     })
// }
