#![doc = include_str!("../README.md")]
#![deny(clippy::all)]
#![warn(clippy::cargo)]
#![warn(clippy::complexity)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::perf)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::many_single_char_names)]
use core::cell::OnceCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::stdin;
use std::path::PathBuf;

use chrono::Days;
use chrono::TimeDelta;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use clap::ArgAction;
use clap::Parser;
use log::info;

// Symbols that indicate a skipped day.
static SKIP_CHARACTERS: [char; 2] = ['x', 'X'];

thread_local! {
// Date to use if none exits in the input.
static DEFAULT_DATE: OnceCell<NaiveDate> = const { OnceCell::new() };
}

static ONE_DAY: Days = Days::new(1);

// Occasionally want to apply Blender specific transform.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Blender compatibility mode.
    #[clap(long, short, action=ArgAction::SetTrue)]
    apply_blender_transform: bool,
    /// Name of the file to convert.
    file: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    env_logger::init();

    info!("main entry");
    let args = Args::parse();

    if let Some(file) = args.file {
        info!("File: {}", file.display());
        if file.exists() {
            if let Some(ext) = file.extension() {
                if ext == "md" {
                    info!("Reading markdown file");
                    let file = File::open(file)?;
                    let buffer = BufReader::new(file);
                    parse_markdown(buffer);
                } else {
                    eprintln!("File extension is not supported");
                }
            } else {
                eprintln!("File must have an extension");
            }
        } else {
            eprintln!("File does not exist");
        }
    } else {
        info!("Reading from stdin");
        let buffer = BufReader::new(stdin());
        parse_markdown(buffer);
    }

    Ok(())
}

fn parse_markdown<R>(buffer: BufReader<R>)
where
    R: Read,
{
    info!("parse_markdown");

    // Skip Header
    let mut lines = buffer.lines().skip(1).peekable();

    // Peek line 2 extract a date if possible
    let mut current_date = if let Some(Ok(date_string)) = lines.peek() {
        NaiveDate::parse_from_str(date_string, "%d %b %Y").unwrap_or_else(|_| {
            info!("invalid start date, using default");
            NaiveDate::parse_from_str("1 Jan 2000", "%d %b %Y").unwrap()
        })
    } else {
        info!("No line to look for date");
        return;
    };

    // A table has 9 '|' vertical separators.
    let table_iter = lines
        .filter_map(|line_result| {
            // tmp
            let line = line_result.expect("failed to read line");
            let count = line.chars().filter(|&c| c == '|').count();
            if count == 8 { Some(line) } else { None }
        })
        // Skip 2 .. skip the  head and formatting lines.
        .skip(2);

    // A list of fitness events.
    // intervals may skip a day
    let mut events = vec![];

    // Plot the results to a png
    for line in table_iter {
        let cells_by_week = line.split('|');

        // skip(1) ... everything before the first '|'
        for cell in cells_by_week.skip(1).take(7) {
            if cell.chars().any(|c| SKIP_CHARACTERS.contains(&c)) {
                // if first cell is X start from 00:00 on that day
                if events.is_empty() {
                    let midnight = NaiveTime::from_num_seconds_from_midnight_opt(0, 0).unwrap();
                    let event = current_date.and_time(midnight);
                    events.push(event);
                }
            } else {
                let stripped = cell.trim().trim_end();
                match NaiveTime::parse_from_str(stripped, "%H:%M") {
                    Ok(time) => {
                        // Combine start_date with time.
                        let event: NaiveDateTime = current_date.and_time(time);
                        events.push(event);
                    }
                    Err(_) => {
                        if stripped.is_empty() {
                            // blank line
                        } else {
                            panic!("Failed to parse cell X{stripped}X");
                        }
                    }
                }
            }
            // Advance current_date by 1 day
            let next_date = current_date;

            current_date = next_date.checked_add_days(ONE_DAY).unwrap();
        }
    }

    let mut events_iter = events.iter();
    let mut event_last = if let Some(first_event) = events_iter.next() {
        *first_event
    } else {
        info!("No events detected");
        return;
    };

    let mut sum = TimeDelta::zero();
    for event_current in events_iter.clone() {
        let duration = *event_current - event_last;
        sum += duration;
        println!("{}", duration.num_hours());
        event_last = *event_current;
    }

    // Fails on division by zero.
    let num_events = i32::try_from(events_iter.len())
        .expect("number of event to large: (Cannot convert usize to i31)");
    if let Some(average) = sum.checked_div(num_events) {
        info!("average interval:{} hrs", average.num_hours());
    }
}
