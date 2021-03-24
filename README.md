# timemachine

[![GitHub last commit](https://img.shields.io/github/last-commit/barrowsys/timemachine)](https://github.com/barrowsys/timemachine)
[![Crates.io](https://img.shields.io/crates/v/timemachine)](https://crates.io/crates/timemachine/)
[![Docs.rs](https://docs.rs/timemachine/badge.svg)](https://docs.rs/timemachine)
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

Handling of daily time-based state transitions made easy.

## About

Have you ever needed to set some state based on the time of day?
Say, be in one state from 7AM until 8AM,, then another state from 8AM until 9PM, then another state from 9PM until 7AM?
It can very quickly become a big pain! There are a lot of edge cases that have to be handled and the code to do so can quickly become spaghetti.

Why even try? The timemachine crate makes it easy to do right! :D  
(for a more thorough explanation of the use case, scroll down)

Feature "napchart" (used by two examples) uses the https://napchart.com API which is in beta.

## Usage

Add dependency to your Cargo.toml:
```
timemachine = "0.3.1"
```
to enable napchart support:
```
timemachine = [ version = "0.3.1", features = ["napchart"] ]
```

## Examples
- `cargo run --example auto_lights`  
    takes a configuration from stdin and simulates 24 hours of light colors

- `cargo run --example napchart_lights --features napchart`  
    similar to auto_lights, but gets a config from https://napchart.com/3tbkt  
    grey = lights off, red = lights red, otherwise lights white

- `cargo run --example napchart_lights_option --features napchart`  
    similar to napchart_lights, but gets its config from https://napchart.com/cse2j  
    grey = lights off, red = lights red, blue = lights white, otherwise output an error

## Wait, what is this for?
Lets say you have some RGB light bulbs, and you want them to automatically switch colors throughout the day.
During the day, you want them to be white, and at night, you want them to be off.
Let's assume that "day" occurs from 7AM until 8PM, and night is therefore from 8PM until 7AM.  
A simple way to accomplish this in pseudocode would be something like:
```
if now() == 7AM {
    lights.turn_on()
} else if now() == 8PM {
    lights.turn_off()
}
```
You write this code and run it\,\,\, but nothing happens!
This code only updates the lights' state at the transition times of 7AM and 8PM, so unless you run it at exactly those times, it wont work.  
That's an easy fix however:
```
if now() > 7AM && now() < 8PM {
	lights.turn_on()
} else if now() > 8PM && now() < 7AM {
	lights.turn_off()
}
```
You run it and\,\,\, it works!! The lights turn on!
You pat yourself on the back and give yourself a sticker.
Sure, we're now updating our lights' state *every second*,
but you could easily add a delay to slow it down to any level of precision you like.

Later, as you're playing your favorite
<abbr title="Massively Multiplayer Online Trading Card Role Playing Game">MMOTCRPG</abbr>,
[Wizard101(TM)](https://www.wizard101.com/), you glance at the clock and realize it's 11PM, long past your bed time!
How could you have done this *again*, didn't you write a program to turn off your lights at bed time so you wouldn't forget?
All of your lights are still on, and you're going to be tired at your meeting tomorrow!
On the other hand, you defeated Malistaire Drake, single-handedly saving the spiral from doom and destruction, so it's not all bad.
You hit the light switch (a neat built-in manual override for your IoT lightbulbs) and go to bed, determined to figure out the bug tomorrow.
(if you experienced physical or mental pain by reading this paragraph,
please contact our legal department at [barrow+legal@tilde.team](mailto:barrow+legal@tilde.team))

After sleeping on it, the bug is obvious. Nighttime isn't `8PM < now() < 7AM`!
11PM, for example, is after 8PM but *also* after 7AM. 1AM is *before* both times!
Again, easy fix
```
if now() > 7AM && now() < 8PM {
	lights.turn_on()
} else if now > 8PM || now() < 7AM {
	lights.turn_off()
}
```
Finally, you've done it. Your lights will be on from 7AM until 8PM, and will be off from 8PM until 7AM.
And, yes! This actually works! Good job!

A few weeks later, and you're starting to dislike your setup.
Every day at 8PM, all the lights in your house suddenly turn off with no warning!
In fact, every day at 7:59PM, your brain (ever the smart one) releases adrenaline in Pavlovian anticipation of the inky blackness.
This will not do! You can't get a good nights sleep if every night it's preceded by fear! There must be a solution.

In fact, there is! Your IoT lightbulbs aren't just connected to the internet, they're also RGB!
Perhaps you could add a sort of artificial dusk? Yes, that should help. You load up your code and get to work,
```
if now() > 7AM && now() < 7:30PM {
	lights.turn_on()
	lights.set_color("white")
} else if now() > 7:30PM && now() < 8PM {
	lights.turn_on()
	lights.set_color("red")
} else if now() > 8PM || now() < 6:30AM {
	lights.turn_off()
} else if now() > 6:30AM && now() < 7AM {
	lights.turn_on()
	lights.set_color("red")
}
```
Perfection. Absolute genius. Not only have you added an artificial dusk, but an artificial dawn as well!
You're so proud of your work that you decide to upload it to [GitHub(TM)](https://github.com) and let other people use your code.
Other people might not sleep exactly how you do, so you add some arguments to change things.

```
dawn_time = 6:30AM
day_time = 7AM
dusk_time = 7:30 PM
night_time = 8PM
if now() > day_time && now() < dusk_time {
	lights.turn_on()
	lights.set_color("white")
} else if now() > dusk_time && now() < night_time {
	lights.turn_on()
	lights.set_color("red")
} else if now() > night_time || now() < dawn_time {
	lights.turn_off()
} else if now() > dawn_time && now() < day_time {
	lights.turn_on()
	lights.set_color("red")
}
```

A few weeks later, and someone opens an issue on your repository.
"it doesnt work, pls help"
Luckily they uploaded their config file:
```
dawn_time = 10:30AM
day_time = 11AM
dusk_time = 12:30 AM
night_time = 1AM
```
You think for a moment and realize what the problem is.
You assumed that people would want to go to bed before midnight!
Should be an easy fix right?

Well, maybe, but it'll get messy, and even if you fix it your code will still break for a config like `dusk_time = 11:30 PM, night_time = 12:30 AM`.
You can't assume that any of the four states won't cross the midnight boundary.

You should've just used the `timemachine` library from the start!
Easy, simple, and deals with all the edge cases. Try `timemachine` today!
```
// Full version of this snippet can be found in examples/auto_lights.rs!
pub enum State {
    NightDark,
    DuskDawnRed,
    DayWhite,
}

fn main() {
    let (dawn_time, day_time, dusk_time, night_time) = get_config();
    let mut tm = TimeMachine::<State>::new();
    tm.add_transition(dawn_time, State::DuskDawnRed);
    tm.add_transition(day_time, State::DayWhite);
    tm.add_transition(dusk_time, State::DuskDawnRed);
    tm.add_transition(night_time, State::NightDark);
    loop {
        sleep_ms(1000);
        match tm.get_state(now()) {
            State::NightDark => lights.turn_off(),
            State::DuskDawnRed => {
                lights.set_color("red");
                lights.turn_on();
            },
            State::DayWhite => {
                lights.set_color("white");
                lights.turn_on();
            },
        }
    }
}
```
