use colored::Colorize;
use jellyfin_rpc::{Button, DisplayFormat, MediaType, VERSION};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use std::env;

/// Main struct containing every other struct in the file.
///
/// The config file is parsed into this struct.
pub struct Config {
    /// Jellyfin configuration.
    ///
    /// Has every required part of the config, hence why its not an `Option<Jellyfin>`.
    pub jellyfin: Jellyfin,
    /// Discord configuration.
    pub discord: Discord,
    /// ImgBB configuration.
    pub imgbb: ImgBB,
    /// Images configuration.
    pub images: Images,
}

/// This struct contains every "required" part of the config.
pub struct Jellyfin {
    /// URL to the jellyfin server.
    pub url: String,
    /// Api key from the jellyfin server, used to gather what's being watched.
    pub api_key: String,
    /// Username of the person that info should be gathered from.
    pub username: Vec<String>,
    /// Contains configuration for Music display.
    pub music: DisplayOptions,
    /// Contains configuration for Movie display.
    pub movies: DisplayOptions,
    /// Contains configuration for Episode display.
    pub episodes: DisplayOptions,
    /// Blacklist configuration.
    pub blacklist: Blacklist,
    /// Self signed certificate option
    pub self_signed_cert: bool,
    /// Simple episode name
    pub show_simple: bool,
    /// Add "0" before season/episode number if lower than 10.
    pub append_prefix: bool,
    /// Add a divider between numbers
    pub add_divider: bool,
}

/// Contains configuration for Music/Movie display.
pub struct DisplayOptions {
    /// Display is where you tell the program what should be displayed.
    pub display: Option<DisplayFormat>,
    /// Separator is what should be between the artist(s) and the `display` options.
    pub separator: Option<String>,
}

/// Discord configuration
pub struct Discord {
    /// Set a custom Application ID to be used.
    pub application_id: Option<String>,
    /// Set custom buttons to be displayed.
    pub buttons: Option<Vec<Button>>,
    /// Show status when media is paused
    pub show_paused: bool,
    /// Text when mouse hovers status image
    pub image_text: String,
}

/// Images configuration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Images {
    /// Url for the pause status icon
    pub pause_icon_image: Option<String>,
    /// image url that is displayed when enable_images is disabled or when API fails
    pub default_image: Option<String>,
    /// Override the default image when media is an episode
    pub episode_image: Option<String>,
    /// Override the default image when media is a movie
    pub movie_image: Option<String>,
    /// Override the default image when media is live tv
    pub tv_image: Option<String>,
    /// Override the default image when media is music
    pub music_image: Option<String>,
    /// Override the default image when media is an audio book
    pub audio_book_image: Option<String>,
    /// Override the default image when media is a book
    pub book_image: Option<String>,
    /// Enables images, not everyone wants them so its a toggle.
    pub enable_images: bool,
    /// Enables imgbb images.
    pub imgbb_images: bool,
}

impl Config {
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub struct ConfigBuilder {
    pub jellyfin: JellyfinBuilder,
    pub discord: Option<DiscordBuilder>,
    pub imgbb: Option<ImgBB>,
    pub images: Option<ImagesBuilder>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct JellyfinBuilder {
    pub url: String,
    pub api_key: Option<String>, // Option since you can overwrite it with a key file
    pub username: Username,
    pub music: Option<DisplayOptionsBuilder>,
    pub movies: Option<DisplayOptionsBuilder>,
    pub episodes: Option<DisplayOptionsBuilder>,
    pub blacklist: Option<Blacklist>,
    pub self_signed_cert: Option<bool>,
    pub show_simple: Option<bool>,
    pub append_prefix: Option<bool>,
    pub add_divider: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Username {
    /// If the username is a `Vec<String>`.
    Vec(Vec<String>),
    /// If the username is a `String`.
    String(String),
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DisplayOptionsBuilder {
    pub display: Option<Display>,
    pub separator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(untagged)]
pub enum Display {
    /// If the Display is a `Vec<String>`.
    Vec(Vec<String>),
    /// If the Display is a comma separated `String`.
    String(String),
    /// If the Display is a `DisplayFormat` struct.
    CustomFormat(DisplayFormat),
}

/// Blacklist MediaTypes and libraries.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Blacklist {
    /// `Vec<String>` of MediaTypes to blacklist
    pub media_types: Option<Vec<MediaType>>,
    /// `Vec<String>` of libraries to blacklist
    pub libraries: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DiscordBuilder {
    pub application_id: Option<String>,
    pub buttons: Option<Vec<Button>>,
    pub show_paused: Option<bool>,
    pub image_text: Option<String>,
}

/// ImgBB configuration
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImgBB {
    /// Contains the api token used to upload images to imgbb.
    pub api_token: Option<String>,
    /// Set the expiration before the image is deleted(in seconds)
    pub expiration: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ImagesBuilder {
    pub pause_icon_image: Option<String>,
    pub default_image: Option<String>,
    pub episode_image: Option<String>,
    pub movie_image: Option<String>,
    pub tv_image: Option<String>,
    pub music_image: Option<String>,
    pub audio_book_image: Option<String>,
    pub book_image: Option<String>,
    pub enable_images: Option<bool>,
    pub imgbb_images: Option<bool>,
}

/// Find urls.json in filesystem, used to store images that were already previously uploaded to imgbb.
///
/// This is to avoid the user having to specify a filepath on launch.
///
/// Default urls.json path depends on OS
/// Windows: `%appdata%\jellyfin-rpc\urls.json`
/// Linux/macOS: `~/.config/jellyfin-rpc/urls.json`
pub fn get_urls_path() -> Result<String, Box<dyn std::error::Error>> {
    if cfg!(not(windows)) {
        debug!("Platform is not Windows");
        let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config_home) => xdg_config_home,
            Err(_) => env::var("HOME")? + "/.config",
        };

        Ok(xdg_config_home + ("/jellyfin-rpc/urls.json"))
    } else {
        debug!("Platform is Windows");
        let app_data = env::var("APPDATA")?;
        Ok(app_data + r"\jellyfin-rpc\urls.json")
    }
}

/// Find default config path (main.json) in filesystem.
///
/// This is to avoid the user having to specify a filepath on launch.
///
/// Default config path depends on OS
/// Windows: `%appdata%\jellyfin-rpc\main.json`
/// Linux/macOS: `~/.config/jellyfin-rpc/main.json`
pub fn get_config_path() -> Result<String, Box<dyn std::error::Error>> {
    debug!("Getting config path");
    if cfg!(not(windows)) {
        debug!("Platform is not Windows");
        let xdg_config_home = match env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config_home) => xdg_config_home,
            Err(_) => env::var("HOME")? + "/.config",
        };

        Ok(xdg_config_home + "/jellyfin-rpc/main.json")
    } else {
        debug!("Platform is Windows");
        let app_data = env::var("APPDATA")?;
        Ok(app_data + r"\jellyfin-rpc\main.json")
    }
}

pub enum ConfigBuilderLoaderError {
    InvalidJellyfinKeyPath,
    InvalidImgBBKeyPath,
    InvalidConfigPath,
    InvalidConfig,
    MissingJellyfinKey
}

impl ConfigBuilder {
    fn new() -> Self {
        Self {
            jellyfin: JellyfinBuilder {
                url: "".to_string(),
                username: Username::String("".to_string()),
                api_key: None,
                music: None,
                movies: None,
                episodes: None,
                blacklist: None,
                self_signed_cert: None,
                show_simple: Some(false),
                append_prefix: Some(false),
                add_divider: Some(false),
            },
            discord: None,
            imgbb: None,
            images: None,
        }
    }

    /// Loads the config from the given paths.
    pub fn load(self, config_path: &str, jellyfin_key_path: &Option<String>, imgbb_key_path: &Option<String>) -> Result<Self, ConfigBuilderLoaderError> {
        debug!("Config path is: {}", config_path);

        let config_data = std::fs::read_to_string(config_path)
            .map_err(|_| ConfigBuilderLoaderError::InvalidConfigPath)?;
        let mut config: ConfigBuilder = serde_json::from_str(&config_data)
            .map_err(|_| ConfigBuilderLoaderError::InvalidConfig)?;

        if let Some(p) = jellyfin_key_path {
            debug!("Jellyfin key path is: {}", p);
            let key_data = std::fs::read_to_string(p)
                .map_err(|_| ConfigBuilderLoaderError::InvalidJellyfinKeyPath)?
                .trim()
                .to_string();

            if config.jellyfin.api_key.is_some() {
                warn!("{}", "Overwriting Jellyfin key from config!".yellow().bold());
            }

            config.jellyfin.api_key = Some(key_data);
        }

        if let Some(p) = imgbb_key_path {
            debug!("ImgBB key path is: {}", p);
            let key_data = std::fs::read_to_string(p)
                .map_err(|_| ConfigBuilderLoaderError::InvalidImgBBKeyPath)?
                .trim()
                .to_string();

            match &mut config.imgbb {
                Some(imgbb) => {
                    if imgbb.api_token.is_some() {
                        warn!("{}", "Overwriting ImgBB key from config!".yellow().bold());
                    }

                    imgbb.api_token = Some(key_data)
                },
                None => config.imgbb = Some(ImgBB { api_token: Some(key_data), expiration: None })
            }
        }

        if config.jellyfin.api_key.is_none() {
            return Err(ConfigBuilderLoaderError::MissingJellyfinKey);
        }

        debug!("Config loaded successfully");

        Ok(config)
    }

    pub fn build(self) -> Config {
        let username = match self.jellyfin.username {
            Username::Vec(usernames) => usernames,
            Username::String(username) => username.split(',').map(|u| u.to_string()).collect(),
        };

        let music_display;
        let music_separator;

        if let Some(music) = self.jellyfin.music {
            if let Some(disp) = music.display {
                music_display = Some(match disp {
                    Display::Vec(display) => DisplayFormat::from(display),
                    Display::String(display) => DisplayFormat::from(display),
                    Display::CustomFormat(display) => display,
                });
            } else {
                music_display = None;
            }

            music_separator = music.separator;
        } else {
            music_display = None;
            music_separator = None;
        }

        let movie_display;
        let movie_separator;

        if let Some(movies) = self.jellyfin.movies {
            if let Some(disp) = movies.display {
                movie_display = Some(match disp {
                    Display::Vec(display) => DisplayFormat::from(display),
                    Display::String(display) => DisplayFormat::from(display),
                    Display::CustomFormat(display) => display,
                });
            } else {
                movie_display = None;
            }

            movie_separator = movies.separator;
        } else {
            movie_display = None;
            movie_separator = None;
        }

        let episode_display;
        let episode_separator;

        if let Some(episodes) = self.jellyfin.episodes {
            if let Some(disp) = episodes.display {
                episode_display = Some(match disp {
                    Display::Vec(display) => DisplayFormat::from(display),
                    Display::String(display) => DisplayFormat::from(display),
                    Display::CustomFormat(display) => display,
                });
            } else {
                episode_display = None;
            }

            episode_separator = episodes.separator;
        } else {
            episode_display = None;
            episode_separator = None;
        }

        let media_types;
        let libraries;

        if let Some(blacklist) = self.jellyfin.blacklist {
            media_types = blacklist.media_types;
            libraries = blacklist.libraries;
        } else {
            media_types = None;
            libraries = None;
        }

        let application_id;
        let buttons;
        let show_paused;
        let mut image_text = format!("tests{}", VERSION.unwrap_or("UNKNOWN"));

        if let Some(discord) = self.discord {
            application_id = discord.application_id;
            buttons = discord.buttons;
            show_paused = discord.show_paused.unwrap_or(true);
            if let Some(text) = discord.image_text {
                image_text = text
                    .replace("{version}", VERSION.unwrap_or("UNKNOWN"));
            }
        } else {
            application_id = None;
            buttons = None;
            show_paused = true;
        }

        let api_token;
        let expiration;

        if let Some(imgbb) = self.imgbb {
            api_token = imgbb.api_token;
            expiration = imgbb.expiration;
        } else {
            api_token = None;
            expiration = None;
        }

        let pause_icon_image;
        let default_image;
        let episode_image;
        let movie_image;
        let tv_image;
        let music_image;
        let audio_book_image;
        let book_image;
        let enable_images;
        let imgbb_images;

        if let Some(images) = self.images {
            pause_icon_image = images.pause_icon_image;
            default_image = images.default_image;
            episode_image = images.episode_image;
            movie_image = images.movie_image;
            tv_image = images.tv_image;
            music_image = images.music_image;
            audio_book_image = images.audio_book_image;
            book_image = images.book_image;
            enable_images = images.enable_images.unwrap_or(false);
            imgbb_images = images.imgbb_images.unwrap_or(false);
        } else {
            pause_icon_image = None;
            default_image = None;
            episode_image = None;
            movie_image = None;
            tv_image = None;
            music_image = None;
            audio_book_image = None;
            book_image = None;
            enable_images = false;
            imgbb_images = false;
        }

        let url;

        if self.jellyfin.url.ends_with("/") {
            url = self.jellyfin.url;
        } else {
             url = self.jellyfin.url + "/"
        }

        Config {
            jellyfin: Jellyfin {
                url,
                api_key: self.jellyfin.api_key.unwrap_or("".to_string()),
                username,
                music: DisplayOptions {
                    display: music_display,
                    separator: music_separator,
                },
                movies: DisplayOptions {
                    display: movie_display,
                    separator: movie_separator,
                },
                episodes: DisplayOptions {
                    display: episode_display,
                    separator: episode_separator,
                },
                blacklist: Blacklist {
                    media_types,
                    libraries,
                },
                self_signed_cert: self.jellyfin.self_signed_cert.unwrap_or(false),
                show_simple: self.jellyfin.show_simple.unwrap_or(false),
                append_prefix: self.jellyfin.append_prefix.unwrap_or(false),
                add_divider: self.jellyfin.add_divider.unwrap_or(false),
            },
            discord: Discord {
                application_id,
                buttons,
                show_paused,
                image_text,
            },
            imgbb: ImgBB {
                api_token,
                expiration
            },
            images: Images {
                pause_icon_image,
                default_image,
                episode_image,
                movie_image,
                tv_image,
                music_image,
                audio_book_image,
                book_image,
                enable_images,
                imgbb_images,
            },
        }
    }
}
