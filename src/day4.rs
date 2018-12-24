use chrono::prelude::*;
use chrono::Duration;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    ShiftStart(Guard),
    Wake,
    Sleep,
}

#[derive(Debug)]
struct Entry {
    when: DateTime<Utc>,
    action: Action,
}

#[derive(Debug)]
struct SleepSpan {
    guard: Guard,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Guard(u32);

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
struct Minute(u32);

use self::Action::*;

impl Entry {
    fn from_line(line: &str) -> Result<Entry, Box<Error>> {
        let datetime = Utc.datetime_from_str(&line[1..17], "%Y-%m-%d %H:%M")?;

        let action = match &line[19..] {
            "wakes up" => Action::Wake,
            "falls asleep" => Action::Sleep,
            // Guard #1901 begins shift
            shift_start => Entry::parse_shift_start(shift_start),
        };

        Ok(Entry {
            when: datetime,
            action,
        })
    }

    fn parse_shift_start(message: &str) -> Action {
        let tail = &message[7..];
        let guard_id_str = tail.split(' ').next().expect("Couldn't find guard ID");
        let guard = Guard(u32::from_str(guard_id_str).expect("Can't parse guard ID"));
        Action::ShiftStart(guard)
    }
}

impl SleepSpan {
    fn new(guard: Guard, start: DateTime<Utc>, end: DateTime<Utc>) -> SleepSpan {
        SleepSpan { guard, start, end }
    }

    fn duration(&self) -> Duration {
        self.end - self.start
    }

    // The minutes of the hour for which the guard was asleep
    fn sleep_minutes(&self) -> Vec<Minute> {
        let mut time = self.start;
        let mut result: Vec<Minute> = Vec::new();

        loop {
            if time.hour() == 0 {
                result.push(Minute(time.minute()));
            }

            time = time + Duration::minutes(1);
            if time >= self.end {
                break;
            }
        }

        result
    }
}

fn read_timeline() -> Result<Vec<Entry>, Box<Error>> {
    let file_contents = fs::read_to_string("inputs/day4-1")?;

    let mut entries: Vec<Entry> = file_contents
        .lines()
        .map(|line| Entry::from_line(line).expect("Can't parse line"))
        .collect();

    entries.sort_unstable_by_key(|e| e.when);

    Ok(entries)
}

fn build_sleep_spans(timeline: &[Entry]) -> Vec<SleepSpan> {
    let mut sleep_spans: Vec<SleepSpan> = Vec::new();
    let mut guard: Option<Guard> = None;
    let mut sleep_start: Option<DateTime<Utc>> = None;

    for entry in timeline {
        match entry.action {
            ShiftStart(g) => {
                assert!(sleep_start.is_none(), "Shift changed when guard asleep");
                guard = Some(g);
            }
            Sleep => {
                sleep_start = Some(entry.when);
            }
            Wake => {
                let guard_id = guard.expect("Waking when no guard on duty");
                let sleep_started = sleep_start.expect("Waking when guard not asleep");

                let span = SleepSpan::new(guard_id, sleep_started, entry.when);

                sleep_spans.push(span);

                sleep_start = None;
            }
        }
    }

    sleep_spans
}

fn make_minutes_map(spans: &mut Iterator<Item = &SleepSpan>) -> HashMap<(Guard, Minute), u32> {
    let mut result: HashMap<(Guard, Minute), u32> = HashMap::new();
    for span in spans {
        for minute in span.sleep_minutes() {
            let counter = result.entry((span.guard, minute)).or_insert(0);
            *counter += 1;
        }
    }
    result
}

pub fn part1() -> Result<u32, Box<Error>> {
    let timeline = read_timeline()?;
    let sleep_spans = build_sleep_spans(&timeline);

    let mut guard_sleep_times: HashMap<Guard, Duration> = HashMap::new();
    for span in sleep_spans.iter() {
        let guard_sleep_time = guard_sleep_times
            .entry(span.guard)
            .or_insert_with(Duration::zero);

        *guard_sleep_time = *guard_sleep_time + span.duration();
    }

    let longest_guard: Guard = *guard_sleep_times
        .iter()
        .max_by_key(|pair| pair.1)
        .expect("No sleep times")
        .0;

    let mut guard_spans = sleep_spans
        .iter()
        .filter(|span| span.guard == longest_guard);

    let minute_map = make_minutes_map(&mut guard_spans);

    let most_common_minute: Minute = (*minute_map
        .iter()
        .max_by_key(|pair| pair.1)
        .expect("No minutes")
        .0)
        .1;

    println!(
        "{:?}, {:?}, Product: {}",
        longest_guard,
        most_common_minute,
        longest_guard.0 * most_common_minute.0
    );

    Ok(longest_guard.0 * most_common_minute.0)
}

pub fn part2() -> Result<u32, Box<Error>> {
    let timeline = read_timeline()?;
    let sleep_spans = build_sleep_spans(&timeline);

    let minute_map = make_minutes_map(&mut sleep_spans.iter());

    let most_common_guard_minute: (Guard, Minute) = *minute_map
        .iter()
        .max_by_key(|pair| pair.1)
        .expect("No minutes")
        .0;

    println!(
        "{:?}, {:?}, Product: {}",
        most_common_guard_minute.0,
        most_common_guard_minute.1,
        (most_common_guard_minute.0).0 * (most_common_guard_minute.1).0
    );

    Ok((most_common_guard_minute.0).0 * (most_common_guard_minute.1).0)
}
