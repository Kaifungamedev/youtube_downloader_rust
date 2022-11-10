use futures::StreamExt;
use rustube::*;
use std::env;
use std::io;
use std::process::Command;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();
    println!("Os: {}", env::consts::OS);
    let dq = '"';
    let path = env::current_dir()?;
    println!("playlist url:");
    let mut url = String::new();
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    url = url.replace("\r\n", "");
    print!("detecting type...");
    if url.contains(&"playlist?list=") {
        println!("playlist");
        let playlist = client.playlist(url.parse()?).await?;
        let _playlist_title = playlist.title();
        println!("{:#?}", playlist.title());
        let videos = playlist.videos();
        futures::pin_mut!(videos);
        println!("Videos: [");
        while let Some(item) = videos.next().await {
            match item {
                Ok(video) => {
                    let vid_title = safetitle(video.title());
                    let vid_id = video.id();
                    let vid_url = format!("https://www.youtube.com/watch?v={}", vid_id);
                    let id = Id::from_raw(&vid_url)?;
                    let _path = format!("{vid_title}.mp4");
                    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
                    println!("Downloading: {vid_title}");
                    let video = descrambler.descramble()?;
                    let _path_to_video = video
                        .streams()
                        .iter()
                        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
                        .max_by_key(|stream| stream.quality_label)
                        .unwrap()
                        .download_to(format!("{vid_title}.mp4"))
                        .await?;
                    println!("done downoading");
                    

                    if env::consts::OS == "windows" {
                        Command::new(format!(
                            "{dq}{}/ffmpeg.exe{dq} {dq}{}/{}.mp4{dq} {dq}{}/{}.mp3{dq}",
                            path.display(),
                            path.display(),
                            vid_title,
                            path.display(),
                            vid_title
                        ))
                        .spawn()
                        .expect("command failed");
                    }
                }
                Err(err) => println!("{:#?},", err),
            }
        }
        println!("]");
    } else if url.contains("watch?v=") {
        println!("video");
        println!("Videos: [");
        let video = client.video(url.parse()?).await?;
        let vid_title = safetitle(video.title());
        let vid_id = video.id();
        let vid_url = format!("https://www.youtube.com/watch?v={}", vid_id);
        let id = Id::from_raw(&vid_url)?;
        let _path = format!("{vid_title}.mp4");
        let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
        println!("Downloading: {vid_title}");
        let video = descrambler.descramble()?;
        let _path_to_video = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
            .unwrap()
            .download_to(format!("{vid_title}.mp4"))
            .await?;
        println!("done downoading");
        if env::consts::OS == "windows" {
            Command::new(format!(
                "{dq}{}/ffmpeg.exe{dq} {dq}{}/{}.mp4{dq} {dq}{}/{}.mp3{dq}",
                path.display(),
                path.display(),
                vid_title,
                path.display(),
                vid_title
            ))
            .spawn()
            .expect("command failed");
        }

        println!("]");
    }

    Ok(())
}
fn safetitle(title: &str) -> String {
    let var = title.replace(".", "");
    let var2 = var.replace("'", "");
    let var3 = var2.replace('"', "");
    let var4 = var3.replace("<", "");
    let var5 = var4.replace(">", "");
    let var6 = var5.replace(":", "");
    let var7 = var6.replace("/", "");
    let var8 = var7.replace("|", "");
    let var9 = var8.replace("?", "");
    let var10 = var9.replace("*", "");
    let var11 = var10.replace(",", "");
    println!("{var11}");
    return format!("{}", &var11);
}
