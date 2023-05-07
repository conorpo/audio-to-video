//! # audio-to-video
//! 
//! Converts an audio file to a video file with a still image.
//! 
//! Usage:
//! `npx audio-to-video ([audio_file]) ([image_file]) (-- [ffmpeg-args])` 
//! `npx audio-to-video`
//! Ommitting fields will cause the program to search the current directory for files.
//! To specify ffmpeg arguments, use `--` to separate the files from the arguments.

use std::collections::{HashSet};
use std::iter::Peekable;
use std::env;
use std::ffi::OsString;
use std::process::{Command, Stdio};

fn main() {
    let audio_formats = HashSet::from(["3gp","aa","aac","aax","act","aiff","alac","amr","ape","au","awb","dct","dss","dvf","flac","gsm","iklax","ivs","m4a","m4b","m4p","mmf","mp3","mpc","msv","nmf","ogg","oga","mogg","opus","ra","rm","raw","rf64","sln","tta","voc","vox","wav","webm","wma","wv","xwma"]);
    let image_formats = HashSet::from(["bmp", "gif", "jpeg", "jpg", "png", "apng", "svg", "tiff", "tif", "webp", "ico", "heif", "heic", "avif", "raw", "arw", "cr2", "nef", "orf", "rw2", "pef", "x3f", "srw", "dng", "jxr", "wdp", "hdp", "jp2", "j2k", "jpf", "jpx", "mj2"]);

    // Makes sure ffmpeg is in PATH
    match env::var("PATH") {
        Ok(paths) => {
            assert!(paths.contains("ffmpeg"), "ffmpeg not found in PATH");
        }
        Err(e) => panic!("couldn't interpret PATH: {e}"),
    }

    // let audio_file: String = args.skip(1).next().unwrap();
    let mut args: Peekable<env::Args> = env::args().peekable();
    
    args.next(); // Skip the path argument

    let mut audio_file: Option<OsString> = None;
    let mut image_file: Option<OsString> = None;

    // Check if files are provided is provided
    while let Some(arg) = args.next() {
        println!("current arg: {:?}", arg);
        if arg == "--" { break; }
        
        let file_extension = arg.split(".").last().unwrap();

        if audio_formats.contains(file_extension) {
            println!("audio file found");
            audio_file = Some(OsString::from(arg));
        } else if image_formats.contains(file_extension) {
            println!("image file found");
            image_file = Some(OsString::from(arg));
        }
    }

    // If no files are provided, search current directory
    let current_dir = env::current_dir().expect("Couldn't get current directory");

    for entry in current_dir.read_dir().expect("Couldn't read directory") {
        if audio_file.is_some() && image_file.is_some() { break; }
        let entry = entry.expect("Couldn't get entry");
        //Get File Extension
        let file_name = entry.file_name();
        let file_name = file_name.to_str().expect("Couldn't convert file name to string");
        let file_extension = file_name.split(".").last().expect("Couldn't get file extension");

        if audio_formats.contains(file_extension) {
            audio_file = Some(entry.file_name());
        } else if image_formats.contains(file_extension) {
            image_file = Some(entry.file_name());
        }
    }  

    audio_file.as_ref().expect("No audio file found");
    image_file.as_ref().expect("No image file found");

    // Get Output File Name
    let file_name: &str = audio_file.as_ref().unwrap().to_str().unwrap_or("default").split(".").next().unwrap();
    let output_file = format!{"{}{}", file_name,".mp4"};

    //Create the ffmpeg command
    let mut command = Command::new("ffmpeg");

    command.arg("-loop").arg("1")
           .arg("-i").arg(image_file.unwrap())
           .arg("-i").arg(audio_file.unwrap())
           .arg("-shortest")
           .arg("-tune").arg("stillimage")
           .arg("-y")
           .arg("-c:v").arg("libx264")
           .arg("-c:a").arg("flac");
        
    for arg in args {
        command.arg(arg);
    }
    
    //Run the ffmpeg command
    let output: std::process::Child = command.arg(output_file)
                                             .stdout(Stdio::piped())
                                             .spawn()
                                             .expect("failed to start process");

    output.wait_with_output().expect("failed to wait on child");
}


        