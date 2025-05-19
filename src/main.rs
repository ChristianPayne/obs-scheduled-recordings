#![allow(unused_assignments)]

use anyhow::Result;
use chrono::Local;
use clap::Parser;
use obws::Client;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    start_time: Option<u64>,
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

    let start_time = args
        .start_time
        .unwrap_or(Local::now().timestamp_millis() as u64);
    let end_time = args.end_time;

    loop {
        match recording_state {
            RecordingState::Stopped => {
                if Local::now().timestamp_millis() > start_time as i64 {
                    client.recording().start().await?;
                    recording_state = RecordingState::Started;
                }
            }
            RecordingState::Started => {
                if Local::now().timestamp_millis() > end_time as i64 {
                    client.recording().stop().await?;
                    recording_state = RecordingState::Stopped;
                    break;
                }
            }
        }
    }

    Ok(())
}
