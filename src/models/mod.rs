use async_trait::async_trait;
use strum_macros::EnumIter;
use core::fmt;
use std::str::FromStr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::libs::db::Track;

#[async_trait]
pub trait SearchAgent: Send + Sync {
    async fn search(&self, search_items: &[LLMResult]) -> Result<Vec<Song>, AppError>;
    fn create_playlist(&self) -> String;
}

#[derive(Debug, Serialize)]
pub enum AppError {
    SearchAgentError(String),
    AppEngineError(String),
}

#[derive(Debug, Deserialize, Default, JsonSchema, Serialize)]
pub struct SearchResult {
    pub results: Vec<LLMResult>,
}

#[derive(Debug, Deserialize, Clone, JsonSchema, PartialEq, PartialOrd, Serialize)]
pub struct LLMResult {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<u32>,
    pub genre: Option<String>,
}

#[derive(Debug, Hash, Default, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq)]
pub struct Song {
    pub uuid: String,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub cover_art: String,
    pub preview_url: String,
}

impl Song {
    pub fn into_track(&self, app_data_id: &str) -> Track {
        Track { 
            uuid: self.uuid.clone(), 
            app_data_id: app_data_id.to_string(),
            title: self.title.clone(), 
            artist: self.artist.clone(), 
            album: self.album.clone(), 
            cover_art: self.cover_art.clone(),
            preview_url: self.preview_url.clone(),
        }
    }
}

#[derive(Debug, Clone, EnumIter, PartialEq)]
pub enum MusicEra {
    Any,
    Modern,
    Contemporary,
    Early2000,
    Mid2000,
    N90sEra,
    T80sEra,
    Oldies,
}

impl fmt::Display for MusicEra {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicEra::Any => "Any",
                MusicEra::Modern => "Modern",
                MusicEra::Contemporary => "Contemporary",
                MusicEra::Early2000 => "Early 2000",
                MusicEra::Mid2000 => "Mid 2000",
                MusicEra::N90sEra => "90s Era",
                MusicEra::T80sEra => "80s Era",
                MusicEra::Oldies => "Oldies",
            }
        )
    }
}

impl FromStr for MusicEra {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Any" => Ok(MusicEra::Any),
            "Modern" => Ok(MusicEra::Modern),
            "Contemporary" => Ok(MusicEra::Contemporary),
            "Early 2000" => Ok(MusicEra::Early2000),
            "Mid 2000" => Ok(MusicEra::Mid2000),
            "90s Era" => Ok(MusicEra::N90sEra),
            "80s Era" => Ok(MusicEra::T80sEra),
            "Oldies" => Ok(MusicEra::Oldies),
            _ => Err(()),
        }
    }
}

#[derive(Clone, EnumIter, Debug, PartialEq)]
pub enum MusicGenre {
    Any,
    Afro,
    AfroBeats,
    HipHop,
    Alternative,
    Classical, // TODO: add more genres
}

impl fmt::Display for MusicGenre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MusicGenre::Any => "Any",
                MusicGenre::Afro=> "Afro",
                MusicGenre::AfroBeats=> "AfroBeats",
                MusicGenre::HipHop => "HipHop",
                MusicGenre::Alternative => "Alternative",
                MusicGenre::Classical => "Classical", 
            }
        )
    }
}

impl FromStr for MusicGenre {
    type Err = ();
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Any" => Ok(MusicGenre::Any),
            "Afro" => Ok(MusicGenre::Afro),
            "AfroBeats" => Ok(MusicGenre::AfroBeats),
            "HipHop" => Ok(MusicGenre::HipHop),
            "Alternative" => Ok(MusicGenre::Alternative),
            "Classical" => Ok(MusicGenre::Classical),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct Config<'a> {
    pub model: &'a str,
    pub context: String,
    pub track_count: u16,
}

impl<'a> Config<'a> {
    pub fn build(model: &'a str, context: String, track_count: u16) -> Self {
        Config { model, context, track_count }
    }
}
