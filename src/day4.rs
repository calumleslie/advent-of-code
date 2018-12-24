use chrono::prelude::*;
use chrono::Duration;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Action {
    ShiftStart(u32),
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
    guard: u32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

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
        let guard_id = u32::from_str(guard_id_str).expect("Can't parse guard ID");
        Action::ShiftStart(guard_id)
    }
}

impl SleepSpan {
    fn new(guard: u32, start: DateTime<Utc>, end: DateTime<Utc>) -> SleepSpan {
        SleepSpan { guard, start, end }
    }

    fn duration(&self) -> Duration {
        self.end - self.start
    }

    // The minutes of the hour for which the guard was asleep
    fn sleep_minutes(&self) -> Vec<u32> {
        let mut time = self.start;
        let mut result: Vec<u32> = Vec::new();

        loop {
            if time.hour() == 0 {
                result.push(time.minute());
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
    let mut guard: Option<u32> = None;
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

pub fn part1() -> Result<u32, Box<Error>> {
    let timeline = read_timeline()?;
    let sleep_spans = build_sleep_spans(&timeline);

    let mut guard_sleep_times: HashMap<u32, Duration> = HashMap::new();
    for span in sleep_spans.iter() {
        let guard_sleep_time = guard_sleep_times
            .entry(span.guard)
            .or_insert_with(Duration::zero);

        *guard_sleep_time = *guard_sleep_time + span.duration();
    }

    let longest_guard: u32 = *guard_sleep_times
        .iter()
        .max_by_key(|pair| pair.1)
        .expect("No sleep times")
        .0;

    let guard_spans = sleep_spans
        .iter()
        .filter(|span| span.guard == longest_guard);

    let mut minutes_commonality: BTreeMap<u32, u32> = BTreeMap::new();
    for span in guard_spans {
        for minute in span.sleep_minutes() {
            let counter = minutes_commonality.entry(minute).or_insert(0);
            *counter += 1;
        }
    }

    let most_common_minute: u32 = *minutes_commonality
        .iter()
        .max_by_key(|pair| pair.1)
        .expect("No minutes")
        .0;

    println!(
        "Guard: {}, Minute: {}, Product: {}",
        longest_guard,
        most_common_minute,
        longest_guard * most_common_minute
    );

    Ok(longest_guard * most_common_minute)
}
