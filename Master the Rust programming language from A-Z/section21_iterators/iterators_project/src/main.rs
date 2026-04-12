use std::{env, process};

#[derive(Debug)]
struct Setting {
    video_file: String,
    subtitle: bool,
    high_definition: bool,
}

fn main() {
    let settings = collect_settings();
    println!("{settings:?}");

    
}

fn collect_settings() -> Setting {
    // target\debug\iterators_project.exe rust.mp4 true false nonsense
    //       using the skips method    -> rust.mp4 true false nonsense
    //       using the take method     -> rust.mp4 true false  \\ it limit us to only take 3 arguments 
    let mut args = env::args().skip(1).take(3);

    let video_file_1 = args.next().unwrap_or_else(|| {
        eprintln!("No video file specified btich");
        process::exit(1);
    });

    let mut settings = args
        .map(|setting| setting.parse::<bool>().unwrap_or(false));

    let subtitle_1 = settings.next().unwrap_or(false);
    let high_definition_1 = settings.next().unwrap_or(false);

    Setting { 
        video_file: video_file_1,
        subtitle: subtitle_1, 
        high_definition: high_definition_1, 
    }


}
