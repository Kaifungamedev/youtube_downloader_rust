use futures::StreamExt;
use rustube::*;
use std::env;
use std::io;
use std::io::Cursor;
use std::path::Path;
use std::process::Command;
use std::*;
type Ree<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
async fn fetch_url(url: String, file_name: String) -> Ree<()> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
async fn converter(vid_title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = env::current_dir()?;
    let _convert_to_mp3 = Command::new(format!("{}\\ffmpeg.exe", path.display()))
        .args(&[
            "-hide_banner",
            "-i",
            &format!(r#"{}\\{vid_title}.tmp"#, path.display()),
            &format!(r#"{}\\{vid_title}.mp3"#, path.display()),
        ])
        .spawn()?
        .wait();
    fs::remove_file(format!("{vid_title}.tmp"))?;
    println!("conversion complete");
    Ok(())
}
async fn mp3(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();
    let path_to_converter = "./ffmpeg.exe";
    if Path::new(path_to_converter).exists() {
        println!("true");
    } else if Path::new(path_to_converter).exists() != true {
        println!("setting up");
        fetch_url(
            "https://github.com/Kaifungamedev/youtube_downloader_rust/releases/download/0.1.3/ffmpeg.exe".to_string(),
            "ffmpeg.exe".to_string(),
        )
        .await
        .unwrap();
    }
    print!("detecting type...");
    if url.contains(&"playlist?list=") {
        println!("playlist");
        let playlist = client.playlist(url.parse()?).await?;
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
                    let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
                    println!("Downloading: {vid_title}");
                    let video = descrambler.descramble()?;
                    let _path_to_video = video
                        .streams()
                        .iter()
                        .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
                        .max_by_key(|stream| stream.quality_label)
                        .unwrap()
                        .download_to(format!("{vid_title}.tmp"))
                        .await?;
                    println!("done downloading");
                    converter(&vid_title).await?;
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
        let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
        println!("Downloading: {vid_title}");
        let video = descrambler.descramble()?;
        let _path_to_video = video
            .streams()
            .iter()
            .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
            .max_by_key(|stream| stream.quality_label)
            .unwrap()
            .download_to(format!("{vid_title}.tmp"))
            .await?;
        println!("done downoading");
        converter(&vid_title).await?;
        println!("]");
    }
    Ok(())
}
async fn mp4(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();
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
                    println!("done downloading");
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

        println!("]");
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Os: {}", env::consts::OS);
    loop {
        println!("YouTube url (press e to exit)");
        let mut url = String::new();
        io::stdin()
            .read_line(&mut url)
            .expect("Failed to read line");
        url = url.replace("\r\n", "");
        if url.contains(&"youtube.com/") {
            println!("mode:\n 1. mp4\n 2. mp3 (windows only)");
            let mut mode = String::new();
            io::stdin()
                .read_line(&mut mode)
                .expect("Failed to read line");
            mode = mode.replace("\r\n", "");
            if mode.to_string() == "1" {
                print!("detecting type...");
                mp4(&url).await?;
            } else if mode.to_string() == "2" {
                if env::consts::OS == "windows" {
                    mp3(&url).await?
                }
                else {
                    println!("Only windows is supported for mp4 downloads")
                }
            } 
            else {
                println!("Command not fount")
            }
        } else if url == "e" ||  url == "E"{
            println!("cleaning up");
            fs::remove_file("./ffmpeg.exe")?;
            break;
        } else {
            println!("NOT A YOUTUBE LINK!");
        }
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
