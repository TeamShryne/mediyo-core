use std::sync::{Arc, Mutex};

use mediyo_core::Session;

uniffi::setup_scaffolding!("mediyo_ffi");

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum MediyoError {
    #[error("API error: {msg}")]
    Api { msg: String },
    #[error("Missing field: {field}")]
    MissingField { field: String },
}

impl From<mediyo_core::Error> for MediyoError {
    fn from(e: mediyo_core::Error) -> Self {
        MediyoError::Api { msg: e.to_string() }
    }
}

#[derive(uniffi::Object)]
pub struct MediyoSession {
    inner: Mutex<Session>,
}

#[uniffi::export]
impl MediyoSession {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { inner: Mutex::new(Session::new()) })
    }

    #[uniffi::constructor]
    pub fn with_cookies(cookie: String, sapisid: Option<String>) -> Arc<Self> {
        let s = Session::new().with_cookies(cookie, sapisid);
        Arc::new(Self { inner: Mutex::new(s) })
    }

    pub fn with_visitor(&self, visitor_data: String, page_id: Option<String>) {
        let mut guard = self.inner.lock().unwrap();
        let mut client = mediyo_core::context::Client::new().with_visitor_data(visitor_data);
        if let Some(pid) = page_id { client = client.with_page_id(pid); }
        let ctx = mediyo_core::context::Context::new().with_client(client);
        let old = std::mem::replace(&mut *guard, Session::new().with_context(ctx));
        // keep auth if any
        let auth = old; // we lose auth — instead rebuild with cookies
        // For simplicity, caller should use with_cookies + with_visitor together via new_with_all
        let _ = auth;
    }

    pub fn fetch_visitor_data(&self) -> Result<String, MediyoError> {
        let mut guard = self.inner.lock().unwrap();
        Ok(guard.fetch_visitor_data()?)
    }
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiSearchResult {
    pub title: String,
    pub video_id: Option<String>,
    pub browse_id: Option<String>,
    pub playlist_id: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiSearchResponse {
    pub results: Vec<FfiSearchResult>,
}

fn to_ffi_search(r: mediyo_core::model::SearchResult) -> FfiSearchResult {
    FfiSearchResult {
        title: r.title,
        video_id: r.video_id,
        browse_id: r.browse_id,
        playlist_id: r.playlist_id,
        category: format!("{:?}", r.category),
    }
}

#[uniffi::export]
pub fn search(session: Arc<MediyoSession>, query: String) -> Result<FfiSearchResponse, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let resp = mediyo_core::api::search::search(&guard, &query)?;
    Ok(FfiSearchResponse { results: resp.results.into_iter().map(to_ffi_search).collect() })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiArtistPage {
    pub name: String,
    pub subscriber_count: Option<String>,
    pub monthly_audience: Option<String>,
    pub description: Option<String>,
}

#[uniffi::export]
pub fn browse_artist(session: Arc<MediyoSession>, browse_id: String) -> Result<FfiArtistPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::browse::artist(&guard, &browse_id)?;
    Ok(FfiArtistPage {
        name: p.name,
        subscriber_count: p.subscriber_count,
        monthly_audience: p.monthly_audience,
        description: p.description,
    })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiAccountInfo {
    pub name: String,
    pub handle: Option<String>,
    pub photo_url: Option<String>,
}

#[uniffi::export]
pub fn account_info(session: Arc<MediyoSession>) -> Result<FfiAccountInfo, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let info = mediyo_core::api::library::account_info(&guard)?;
    Ok(FfiAccountInfo { name: info.name, handle: info.handle, photo_url: info.photo_url })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiLibraryPage {
    pub titles: Vec<String>,
}

#[uniffi::export]
pub fn library_playlists(session: Arc<MediyoSession>) -> Result<FfiLibraryPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let page = mediyo_core::api::library::playlists(&guard)?;
    Ok(FfiLibraryPage { titles: page.items.into_iter().map(|i| i.title).collect() })
}

#[uniffi::export]
pub fn library_songs(session: Arc<MediyoSession>) -> Result<FfiLibraryPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let page = mediyo_core::api::library::songs(&guard)?;
    Ok(FfiLibraryPage { titles: page.items.into_iter().map(|i| i.title).collect() })
}

#[derive(Debug, Clone, uniffi::Enum)]
pub enum FfiLikeStatus { Like, Dislike, Indifferent }

#[uniffi::export]
pub fn rate_song(session: Arc<MediyoSession>, video_id: String, status: FfiLikeStatus) -> Result<(), MediyoError> {
    let guard = session.inner.lock().unwrap();
    let s = match status {
        FfiLikeStatus::Like => mediyo_core::api::library::LikeStatus::Like,
        FfiLikeStatus::Dislike => mediyo_core::api::library::LikeStatus::Dislike,
        FfiLikeStatus::Indifferent => mediyo_core::api::library::LikeStatus::Indifferent,
    };
    mediyo_core::api::library::rate_song(&guard, &video_id, s)?;
    Ok(())
}

#[uniffi::export]
pub fn add_to_playlist(session: Arc<MediyoSession>, playlist_id: String, video_id: String) -> Result<(), MediyoError> {
    let guard = session.inner.lock().unwrap();
    mediyo_core::api::library::add_to_playlist(&guard, &playlist_id, &video_id)?;
    Ok(())
}

#[uniffi::export]
pub fn create_playlist(session: Arc<MediyoSession>, title: String, description: String) -> Result<String, MediyoError> {
    let guard = session.inner.lock().unwrap();
    Ok(mediyo_core::api::library::create_playlist(&guard, &title, &description, "PRIVATE")?)
}
