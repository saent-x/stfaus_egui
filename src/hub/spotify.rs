#![allow(unused)] // thid is a temporary measure to prevent distractions from unused var...
use crate::models::{AppError, LLMResult, SearchAgent, Song};
use async_trait::async_trait;
use rspotify::{
    model::{IncludeExternal, SearchResult, SearchType},
    prelude::*,
    ClientCredsSpotify, Credentials,
};

pub struct SpotifyAgent {
    client: ClientCredsSpotify,
}

impl SpotifyAgent {
    pub async fn init() -> Result<Self, AppError> {
        let client_id = env!("RSPOTIFY_CLIENT_ID");
        let client_secret = env!("RSPOTIFY_CLIENT_SECRET");

        let creds = Credentials::new(client_id, client_secret);
        let client = ClientCredsSpotify::new(creds);
        client
            .request_token()
            .await
            .map_err(|err| AppError::SearchAgentError(err.to_string()))?;

        Ok(Self { client })
    }
}

#[async_trait]
impl SearchAgent for SpotifyAgent {
    async fn search(&self, search_items: &[LLMResult]) -> Result<Vec<Song>, AppError> {
        let mut results: Vec<Song> = vec![];

        for llmresult in search_items {
            let query = format!(
                "track:{} artist:{} year:{}",
                llmresult.title.clone().unwrap_or_default(),
                llmresult.artist.clone().unwrap_or_default(),
                llmresult.year.unwrap_or_default()
            );

            let result = self
                .client
                .search(
                    &query,
                    SearchType::Track,
                    None,
                    Some(IncludeExternal::Audio),
                    Some(1),
                    None,
                )
                .await
                .map_err(|err| AppError::SearchAgentError(err.to_string()))?;

            if let SearchResult::Tracks(tracks) = result {
                if let Some(track) = tracks.items.first() {
                    let song = Song {
                        uuid: track.id.as_ref().unwrap().to_string(), //TODO: better handling of option
                        title: track.name.clone(),
                        artist: track.artists[0].name.clone(),
                        album: track.album.name.clone(),
                        cover_art: track.album.images[0].url.clone(),
                        preview_url: "".to_string(),
                    };

                    results.push(song);
                }
            }
        }

        Ok(results)
    }

    fn create_playlist(&self) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{AppError, LLMResult, SearchAgent, Song};

    use super::SpotifyAgent;

    #[tokio::test]
    async fn test_spotify_search() {
        let search_agent = SpotifyAgent::init().await.expect("search query failed");

        let results = search_agent
            .search(&[LLMResult {
                artist: Some("Sasha Alex Sloan".to_string()),
                title: Some("Older".to_string()),
                album: Some("Single".to_string()),
                year: Some(2018),
                genre: Some("Pop".to_string()),
            }])
            .await;

        dbg!(&results);

        assert!(results.is_ok());
    }
}
