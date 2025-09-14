use std::{
    fs::{self, File, OpenOptions},
    io::{Error, ErrorKind, Write},
    path::Path,
};

use serde::{Deserialize, Serialize};
use url::Url;

use crate::{Client, JfResult};

#[derive(Deserialize, Serialize)]
struct ImageUrl {
    id: String,
    url: String,
}

impl ImageUrl {
    fn new<T: Into<String>, Y: Into<String>>(id: T, url: Y) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
        }
    }
}


#[derive(Deserialize)]
pub struct ImgBBResponse {
    pub data: ImageData,
}

#[derive(Deserialize)]
pub struct ImageData {
    pub url: String,
}

pub fn get_image(client: &Client) -> JfResult<Url> {
    let mut image_urls = read_file(client)?;

    if let Some(image_url) = image_urls
        .iter()
        .find(|image_url| client.session.as_ref().unwrap().item_id == image_url.id)
    {
        Ok(Url::parse(&image_url.url)?)
    } else {
        let imgbb_url = upload(client)?;

        let image_url = ImageUrl::new(
            &client.session.as_ref().unwrap().item_id,
            imgbb_url.as_str(),
        );

        image_urls.push(image_url);

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&client.imgbb_options.urls_location)?;

        file.write_all(serde_json::to_string(&image_urls)?.as_bytes())?;

        let _ = file.flush();

        Ok(imgbb_url)
    }
}

fn read_file(client: &Client) -> JfResult<Vec<ImageUrl>> {
    if let Ok(contents_raw) = fs::read_to_string(&client.imgbb_options.urls_location) {
        if let Ok(contents) = serde_json::from_str::<Vec<ImageUrl>>(&contents_raw) {
            return Ok(contents);
        }
    }

    let path = Path::new(&client.imgbb_options.urls_location)
        .parent()
        .ok_or(Error::new(
            ErrorKind::Other,
            "Can't find parent folder of urls.json",
        ))?;

    fs::create_dir_all(path)?;

    let mut file = File::create(client.imgbb_options.urls_location.clone())?;

    let new: Vec<ImageUrl> = vec![];

    file.write_all(serde_json::to_string(&new)?.as_bytes())?;

    let _ = file.flush();

    Ok(new)
}

fn upload(client: &Client) -> JfResult<Url> {
    let image_bytes = client.reqwest.get(client.get_image()?).send()?.bytes()?;

    let imgbb_client = reqwest::blocking::Client::builder().build()?;

    let form = reqwest::blocking::multipart::Form::new()
        .part("image", reqwest::blocking::multipart::Part::bytes(image_bytes.to_vec())
        .file_name("test.jpg"));

    let res: ImgBBResponse = imgbb_client
        .post(format!("https://api.imgbb.com/1/upload?expiration=600&key={}", client.imgbb_options.api_token))
        .multipart(form)
        .send()?
        .json()?;

    Ok(Url::parse(res.data.url.as_str())?)
}
