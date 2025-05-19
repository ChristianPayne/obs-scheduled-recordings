#![allow(unused_assignments)]

use anyhow::Result;
use chrono::{Date, DateTime, Duration, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use obws::Client;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    start_time: u64,
    #[arg(short, long)]
    end_time: u64,
}

enum RecordingState {
    Started,
    Stopped,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::connect("localhost", 4455, Some("PVGxIseZDSIc2GKH")).await?;

    let mut recording_state = RecordingState::Stopped;

    // Check OBS to see what the recording state is.
    let recording_status = client.recording().status().await?;
    if recording_status.active {
        recording_state = RecordingState::Started;
    } else {
        recording_state = RecordingState::Stopped;
    }

    dbg!(Utc::now().timestamp_millis());

    loop {
        match recording_state {
            RecordingState::Stopped => {
                if Utc::now().timestamp_millis() > args.start_time as i64 {
                    client.recording().start().await?;
                    recording_state = RecordingState::Started;
                }
            }
            RecordingState::Started => {
                if Utc::now().timestamp_millis() > args.end_time as i64 {
                    client.recording().stop().await?;
                    recording_state = RecordingState::Stopped;
                    break;
                }
            }
        }
    }

    Ok(())
}
