use clap::Parser;
use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;

// Structure to take the regex and extensions as arguments
// add the defaults
#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, default_value = "srt")]
    subtitle_extension: String,
    #[clap(short, long, default_value = "mkv")]
    video_extension: String,
    #[clap(long, default_value = "E(\\d{2})")]
    video_regex_episode: String,
    #[clap(long, default_value = "(\\d{2})")]
    subtitle_regex_episode: String,
}

fn main() {
    // get the arguments
    let args = Args::parse();

    // get the videos and subtitles
    let (videos, mut subtitles) = get_videos_and_subs(&args);

    // loop over the videos
    for (episode_number, video) in videos {
        // find the subtitle that matches the episode number
        let subtitle = subtitles.remove(&episode_number).unwrap();

        // Rename the subtitle to the same name as the video
        let new_subtitle = format!("{}.{}", video, args.subtitle_extension);

        // log
        println!(
            "Renaming {} to {}",
            subtitle.red(),       // This will color the `subtitle` variable red
            new_subtitle.green()  // This will color the `new_subtitle` variable green
        );

        fs::rename(subtitle.clone(), new_subtitle.clone()).unwrap();
    }
}

// get the episode number of a video
fn get_episode_number(file: &str, reg: &str) -> Result<u32, String> {
    // Attempt to compile the regex, return error if it fails
    let re = Regex::new(reg).map_err(|e| format!("Failed to compile regex: {}", e))?;

    // Attempt to find matches in the video string, return error if it fails
    let captures = re
        .captures(file)
        .ok_or_else(|| format!("No matches found for file: {}", file))?;

    // Extract the episode number, return error if no such capture group exists
    let episode_str = captures
        .get(1)
        .ok_or_else(|| "Episode number capture group not found".to_string())?
        .as_str();

    // Attempt to parse the episode number, return error if parsing fails
    episode_str
        .parse::<u32>()
        .map_err(|e| format!("Failed to parse episode number: {}", e))
}

// Get the videos and subtitles of the directory
fn get_videos_and_subs(args: &Args) -> (HashMap<u32, String>, HashMap<u32, String>) {
    let current_dir = env::current_dir().unwrap();

    let entries = fs::read_dir(current_dir).unwrap();

    // videos
    let mut videos = HashMap::new();
    // subtitles
    let mut subtitles = HashMap::new();

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_name = file_name.to_string();

            if file_name.ends_with(args.video_extension.as_str()) {
                // add the video to the list of videos
                match get_episode_number(&file_name, args.video_regex_episode.as_str()) {
                    Ok(episode_number) => {
                        videos.insert(
                            episode_number,
                            file_name.split('.').collect::<Vec<&str>>()[0].to_string(),
                        );
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            } else if file_name.ends_with(args.subtitle_extension.as_str()) {
                // add the subtitle to the list of subtitles
                match get_episode_number(&file_name, args.subtitle_regex_episode.as_str()) {
                    Ok(episode_number) => {
                        subtitles.insert(episode_number, file_name);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }

    (videos, subtitles)
}

// Go through all the files in the directory
// gather the subtitles
// gather the shows
// match on the episode number

// loop over each episode
// find the related subtitle with the episode number
// rename the subtitle to the episode name
