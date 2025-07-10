use tokio::fs;
use uuid::Uuid;
use std::path::Path;
use chrono::Utc;
use crate::models::FileMeta;
use std::error::Error;

pub async fn delete_file(id: &str) -> Result<(), Box<dyn Error>> {
    let read_file_data = fs::read_to_string("metadata.json").await?;
    let files = serde_json::from_str::<Vec<FileMeta>>(&read_file_data)?;
    let mut files = files;

    if let Some(file) = files.iter().find(|file| file.id == id) {
        fs::remove_file(&file.saved_path).await?;
        files.retain(|f| f.id != id);
        let new_json = serde_json::to_string_pretty(&files)?;
        fs::write("metadata.json", new_json).await?;
        println!("File with id {} deleted successfully.", id);
    } else {
        println!("File not found");
    }

    Ok(())
}

pub async fn list_files() -> Result<(), Box<dyn Error>> {
    let read_file_data = fs::read_to_string("metadata.json").await.unwrap_or_default();
    let files = serde_json::from_str::<Vec<FileMeta>>(&read_file_data)?;

    for file in files {
        println!("ID: {}", file.id);
        println!("Filename: {}", file.filename);
        println!("Saved Path: {}", file.saved_path);
        println!("Upload Time: {}", file.upload_time);
        println!("------------------------------");
    }

    Ok(())
}

pub async fn download_file(id: &str) -> Result<(), Box<dyn Error>> {
    let read_file_data = fs::read_to_string("metadata.json").await.unwrap_or_default();
    let files = serde_json::from_str::<Vec<FileMeta>>(&read_file_data)?;

    if let Some(file) = files.iter().find(|file| file.id == id) {
        fs::create_dir_all("downloads").await?;
        let download_path = format!("downloads/{}", file.filename);
        fs::copy(&file.saved_path, &download_path).await?;
        println!("File downloaded to {}", download_path);
    } else {
        println!("File with id {} not found.", id);
    }

    Ok(())
}

pub async fn upload_file(path: &str) -> Result<(), Box<dyn Error>> {
    if !fs::try_exists(path).await.unwrap_or(false) {
        println!("File does not exist: {}", path);
        return Ok(());
    }

    let id = Uuid::new_v4().to_string();
    let save_dir = "storage";
    fs::create_dir_all(save_dir).await?;
    let save_path = format!("{}/{}", save_dir, id);
    fs::copy(path, &save_path).await?;
    println!("File successfully copied to {}", save_path);

    let filename = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown")
        .to_string();

    let upload_time = Utc::now();

    let meta = FileMeta {
        id,
        filename,
        saved_path: save_path,
        upload_time,
    };

    let meta_path = "metadata.json";

    let mut existing = if let Ok(content) = fs::read(meta_path).await {
        let content_str = String::from_utf8_lossy(&content);
        serde_json::from_str::<Vec<FileMeta>>(&content_str).unwrap_or_default()
    } else {
        Vec::new()
    };

    existing.push(meta);

    let json = serde_json::to_string_pretty(&existing)?;
    fs::write(meta_path, json).await?;

    Ok(())
}
