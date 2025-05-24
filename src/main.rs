use anyhow::Result;
use chrono::Local;
use clap::Parser;
use obws::{Client, responses::StatusCode};
use tokio::time;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    password: Option<String>,
    #[arg(short, long)]
    ip: Option<String>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(short, long)]
    start_time: Option<u64>,
    #[arg(short, long)]
    end_time: u64,
}

#[derive(Debug, PartialEq)]
enum RecordingState {
    Started,
    Stopped,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let ip = args.ip.unwrap_or("localhost".to_string());
    let port = args.port.unwrap_or(4455);
    let client = Client::connect(ip, port, args.password).await?;

    let mut recording_state = get_recording_status(&client).await?;

    let start_time = args
        .start_time
        .unwrap_or(Local::now().timestamp_millis() as u64);
    let end_time = args.end_time;

    println!("Starting {}s video", (end_time - start_time) / 1000);

    loop {
        // Get the recording status on every loop.
        // Staying in sync with OBS is important so we don't send the wrong command when interacting with it.
        let updated_recording_state = get_recording_status(&client).await?;

        // if updated_recording_state != recording_state {
        //     // We changed recording state outside of this script. If we need to update anything here, we can.
        //     recording_state = updated_recording_state;
        // }

        let now = Local::now().timestamp_millis();

        // Milliseconds remaining.
        let time_remaining = end_time as i64 - now;
        if time_remaining > 0 && (time_remaining / 1000) % 10 == 0 {
            println!("Time remaining: {}s", time_remaining / 1000);
        }

        match (&recording_state, &updated_recording_state) {
            (RecordingState::Stopped, RecordingState::Stopped) => {
                if now >= start_time as i64 {
                    match client.recording().start().await {
                        Ok(_) => {}
                        Err(error) => {
                            if let obws::error::Error::Api { code, .. } = &error {
                                match code {
                                    StatusCode::OutputRunning => {
                                        println!(
                                            "Looks like OBS is already recording, no need to start."
                                        );
                                    }
                                    _ => panic!("{}", error),
                                }
                            }
                        }
                    };

                    recording_state = RecordingState::Started;
                }
            }
            (RecordingState::Started, RecordingState::Started) => {
                if now >= end_time as i64 {
                    match client.recording().stop().await {
                        Ok(_) => {}
                        Err(error) => {
                            if let obws::error::Error::Api { code, .. } = &error {
                                match code {
                                    StatusCode::OutputNotRunning => {
                                        println!("OBS is not recording, no need to stop.");
                                    }
                                    _ => panic!("{}", error),
                                }
                            }
                        }
                    };

                    break;
                }
            }
            // If we are started and OBS was stopped outside of this script, stop the script.
            (RecordingState::Started, RecordingState::Stopped) => {
                println!(
                    "OBS stopped recording, stopping script ({}s).",
                    (end_time - now as u64) / 1000
                );
                return Ok(());
            }
            // If we are stopped and OBS was started outside of this script, do nothing.
            (RecordingState::Stopped, RecordingState::Started) => {}
        }

        time::sleep(tokio::time::Duration::from_secs(1)).await;
    }

    println!(
        "Finished recording {}s video",
        (end_time - start_time) / 1000
    );

    Ok(())
}

async fn get_recording_status(client: &Client) -> Result<RecordingState> {
    // Check OBS to see what the recording state is.
    match client.recording().status().await {
        Ok(recording_status) => {
            if recording_status.active {
                Ok(RecordingState::Started)
            } else {
                Ok(RecordingState::Stopped)
            }
        }
        Err(err) => Err(err.into()),
    }
}
