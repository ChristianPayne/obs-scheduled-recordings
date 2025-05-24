# OBS Scheduled Recordings
A CLI tool designed to connect to OBS Websockets and schedule when to start and stop a recording. 

## How to use
### OBS Setup
For this tool to connect to OBS, OBS Websocket Server needs to be on. Go to `Tools > Websocket Server Settings > Enable Websocket Server`.

### Scene
Your OBS scene needs to be set up ahead of time. Whatever is set up will be recorded.

### Arguments
--password (-p): Get your OBS Websocket Server password from the tools panel and pass it in here. This flag is optional in case your OBS Websocket Server is configured to not have authentication.  

--ip (-i): The local ip of the computer that OBS is running on. This flag is optional and defaults to `localhost` if not provided.  

--port: The port that OBS Websocket Server is running on. This flag is optional and defaults to `4455` if not provided.  

--start-time (-s): The timestamp in milliseconds (epoch time) of when to start the recording. This flag is optional, if not provided then the recording will start when the script is run.  

--end-time (-e): The timestamp in milliseconds (epoch time) of when to end the recording.

### Examples
- obs-scheduled-recordings --password 1234567890abcdef --end-time 1748066073000  
- obs-scheduled-recordings --password 1234567890abcdef --start-time 1748066073000 --end-time 1748066088000  
- obs-scheduled-recordings --password 1234567890abcdef --start-time 1748066073000 --end-time 1748066088000 --ip 192.168.1.10 --port 4455  

## Notes
For help getting timestamps, check out [Timestamp Converter](https://www.timestamp-converter.com/).

Common timestamps are in seconds. This tool uses milliseconds.