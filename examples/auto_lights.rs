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

use timemachine::{Time, TimeMachine, Clock};
use console::Term;
use dialoguer::Input;
#[allow(deprecated)]
use std::thread::sleep_ms;
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    NightDark,
    DuskDawnRed,
    DayWhite,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tm = TimeMachine::<State>::new();
    let (dawn1, dawn2, dusk1, dusk2) = handle_input(&mut tm)?;
    tm.add_transition(dawn1, State::DuskDawnRed);
    tm.add_transition(dawn2, State::DayWhite);
    tm.add_transition(dusk1, State::DuskDawnRed);
    tm.add_transition(dusk2, State::NightDark);
    let term = Term::stdout();
    term.write_line("")?;
    term.hide_cursor()?;
    for t in Clock::minutes() {
        term.write_line(&format!("The time is {:#.2}    ", &t))?;
        term.write_str(match tm.get_state(&t).ok().unwrap() {
            State::NightDark   => "it is dark           ",
            State::DuskDawnRed => "the red light is on  ",
            State::DayWhite    => "the white light is on",
        })?;
        sleep_ms(50);
        term.move_cursor_up(1)?;
        term.move_cursor_left(21)?;
    }
    term.show_cursor()?;
    term.move_cursor_down(2)?;
    Ok(())
}

fn handle_input(tm: &mut TimeMachine<State>) -> std::io::Result<(Time, Time, Time, Time)> {
    let time_re = Regex::new(r"(\d?\d):(\d?\d)(?: ?([AP]M))?").unwrap();
    let dawn2: String = Input::new()
        .with_prompt("What time should the white light turn on in the morning?")
        .default("8:00".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            if time_re.is_match(input) {
                Ok(())
            } else {
                Err("This is not a time")
            }
        })
        .interact_text()?;
    let dawn2 = {
        let captures = time_re.captures(&dawn2).unwrap();
        let mut hour: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
        let minute: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
        match captures.get(3) {
            Some(m) => {
                hour = hour % 12;
                if m.as_str() == "PM" {
                    hour += 12;
                }
            },
            None => {
                hour = hour % 24;
            },
        };
        Time::new_hm(hour, minute)
    };
    let dawn_length: u16 = Input::new()
        .with_prompt("How many minutes before should the red light turn on?")
        .default(60)
        .interact_text()?;
    let dawn1 = Time::from_seconds(
        dawn2.as_seconds() - (dawn_length * 60) as u32
    );
    let dusk2: String = Input::new()
        .with_prompt("What time should the white light turn off in the evening?")
        .default("21:00".into())
        .validate_with(|input: &String| -> Result<(), &str> {
            if time_re.is_match(input) {
                Ok(())
            } else {
                Err("This is not a time")
            }
        })
        .interact_text()?;
    let dusk1 = {
        let captures = time_re.captures(&dusk2).unwrap();
        let mut hour: u8 = captures.get(1).unwrap().as_str().parse().unwrap();
        let minute: u8 = captures.get(2).unwrap().as_str().parse().unwrap();
        match captures.get(3) {
            Some(m) => {
                hour = hour % 12;
                if m.as_str() == "PM" {
                    hour += 12;
                }
            },
            None => {
                hour = hour % 24;
            },
        };
        Time::new_hm(hour, minute)
    };
    let dusk_length: u16 = Input::new()
        .with_prompt("How many minutes after should the red light stay on?")
        .default(60)
        .interact_text()?;
    let dusk2 = Time::from_seconds(
        dusk1.as_seconds() + (dusk_length * 60) as u32
    );
    Ok((dawn1, dawn2, dusk1, dusk2))
}
