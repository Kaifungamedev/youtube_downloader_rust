use futures::StreamExt;
use rustube::*;
use std::io;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ytextract::Client::new();
    println!("playlist url:");
    let mut url = String::new();
    
    io::stdin()
        .read_line(&mut url)
        .expect("Failed to read line");
    println!("{url}");
    url = url.replace("\r\n", "");
    let playlist = client.playlist(url.parse()?).await?;
    let _playlist_title = playlist.title();
    println!("{:#?}", playlist.title());
    let videos = playlist.videos();
    futures::pin_mut!(videos);
    println!("Videos: [");
    while let Some(item) = videos.next().await {
        match item {
            Ok(video) => {
                let vid_title = video.title();
                let vid_id = video.id();
                let vid_url = format!("https://www.youtube.com/watch?v={}", vid_id);
                let id = Id::from_raw(&vid_url)?;
                let _path = format!("{vid_title}.mp3");
                let descrambler = VideoFetcher::from_id(id.into_owned())?.fetch().await?;
                println!("Downloading : {vid_title}");
                let video = descrambler.descramble()?;
                let _path_to_video = video
                    .streams()
                    .iter()
                    .filter(|stream| stream.includes_video_track && stream.includes_audio_track)
                    .max_by_key(|stream| stream.quality_label)
                    .unwrap()
                    .download_to(format!("{vid_title}.mp3"))
                    .await?;
                println!("done downoading");

            }
            Err(err) => println!("{:#?},", err),
        }
    }

    println!("]");

    Ok(())
}
