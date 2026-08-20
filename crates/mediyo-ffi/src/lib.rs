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
        Arc::new(Self { inner: Mutex::new(Session::new().with_cookies(cookie, sapisid)) })
    }

    #[uniffi::constructor]
    pub fn with_all(cookie: String, sapisid: Option<String>, visitor_data: String, page_id: Option<String>) -> Arc<Self> {
        let mut client = mediyo_core::context::Client::new().with_visitor_data(visitor_data);
        if let Some(pid) = page_id { client = client.with_page_id(pid); }
        let ctx = mediyo_core::context::Context::new().with_client(client);
        let s = Session::new().with_context(ctx).with_cookies(cookie, sapisid);
        Arc::new(Self { inner: Mutex::new(s) })
    }

    pub fn fetch_visitor_data(&self) -> Result<String, MediyoError> {
        let mut guard = self.inner.lock().unwrap();
        Ok(guard.fetch_visitor_data()?)
    }
}

// ── search ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiSearchFilter {
    pub label: String,
    pub query: String,
    pub params: Option<String>,
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
    pub filters: Vec<FfiSearchFilter>,
    pub continuation: Option<String>,
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
    Ok(FfiSearchResponse {
        results: resp.results.into_iter().map(to_ffi_search).collect(),
        filters: resp.filters.into_iter().map(|f| FfiSearchFilter { label: f.label, query: f.query, params: f.params }).collect(),
        continuation: resp.continuation,
    })
}

#[uniffi::export]
pub fn search_with_params(session: Arc<MediyoSession>, query: String, params: String) -> Result<FfiSearchResponse, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let resp = mediyo_core::api::search::search_with_params(&guard, &query, Some(&params))?;
    Ok(FfiSearchResponse {
        results: resp.results.into_iter().map(to_ffi_search).collect(),
        filters: resp.filters.into_iter().map(|f| FfiSearchFilter { label: f.label, query: f.query, params: f.params }).collect(),
        continuation: resp.continuation,
    })
}

#[uniffi::export]
pub fn search_continuation(session: Arc<MediyoSession>, token: String) -> Result<FfiSearchResponse, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let resp = mediyo_core::api::search::search_continuation(&guard, &token)?;
    Ok(FfiSearchResponse {
        results: resp.results.into_iter().map(to_ffi_search).collect(),
        filters: Vec::new(),
        continuation: resp.continuation,
    })
}

// ── browse ─────────────────────────────────────────────────────────────────

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
    Ok(FfiArtistPage { name: p.name, subscriber_count: p.subscriber_count, monthly_audience: p.monthly_audience, description: p.description })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiBrowsePage {
    pub title: String,
    pub tracks: Vec<String>,
    pub continuation: Option<String>,
}

#[uniffi::export]
pub fn browse_album(session: Arc<MediyoSession>, browse_id: String) -> Result<FfiBrowsePage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::browse::album(&guard, &browse_id)?;
    Ok(FfiBrowsePage { title: p.title, tracks: p.tracks.into_iter().map(|t| t.title).collect(), continuation: p.continuation })
}

#[uniffi::export]
pub fn browse_playlist(session: Arc<MediyoSession>, browse_id: String) -> Result<FfiBrowsePage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::browse::playlist(&guard, &browse_id)?;
    Ok(FfiBrowsePage { title: p.title, tracks: p.tracks.into_iter().map(|t| t.title).collect(), continuation: p.continuation })
}

#[uniffi::export]
pub fn browse_explore(session: Arc<MediyoSession>) -> Result<FfiExplorePage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::browse::explore(&guard)?;
    Ok(FfiExplorePage {
        nav_buttons: p.nav_buttons.into_iter().map(|b| FfiNavButton { label: b.label, browse_id: b.browse_id }).collect(),
        carousels: p.carousels.into_iter().map(|c| c.title).collect(),
    })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiExplorePage {
    pub nav_buttons: Vec<FfiNavButton>,
    pub carousels: Vec<String>,
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiNavButton {
    pub label: String,
    pub browse_id: String,
}

// Podcasts: browse show detail via generic browseId (MPSP...)
#[uniffi::export]
pub fn browse_podcast(session: Arc<MediyoSession>, browse_id: String) -> Result<FfiBrowsePage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    // Podcast shows are like albums — reuse playlist browse for episodes
    let resp = guard.request("browse", serde_json::json!({"browseId": browse_id}))?;
    let page = mediyo_core::model::browse::parse_list_page(&resp)?;
    Ok(FfiBrowsePage { title: browse_id, tracks: page.items.into_iter().map(|i| i.title).collect(), continuation: page.continuation })
}

// ── watch ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiQueue {
    pub titles: Vec<String>,
    pub continuation: Option<String>,
}

#[uniffi::export]
pub fn get_queue(session: Arc<MediyoSession>, video_id: String, playlist_id: Option<String>) -> Result<FfiQueue, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let q = mediyo_core::api::watch::get_queue(&guard, &video_id, playlist_id.as_deref())?;
    Ok(FfiQueue { titles: q.items.into_iter().map(|i| i.title).collect(), continuation: q.continuation })
}

// ── library ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiAccountInfo { pub name: String, pub handle: Option<String>, pub photo_url: Option<String> }

#[uniffi::export]
pub fn account_info(session: Arc<MediyoSession>) -> Result<FfiAccountInfo, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let info = mediyo_core::api::library::account_info(&guard)?;
    Ok(FfiAccountInfo { name: info.name, handle: info.handle, photo_url: info.photo_url })
}

#[derive(Debug, Clone, uniffi::Record)]
pub struct FfiLibraryPage { pub titles: Vec<String>, pub continuation: Option<String> }

#[uniffi::export]
pub fn library_playlists(session: Arc<MediyoSession>) -> Result<FfiLibraryPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::library::playlists(&guard)?;
    Ok(FfiLibraryPage { titles: p.items.into_iter().map(|i| i.title).collect(), continuation: p.continuation })
}

#[uniffi::export]
pub fn library_songs(session: Arc<MediyoSession>) -> Result<FfiLibraryPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let p = mediyo_core::api::library::songs(&guard)?;
    Ok(FfiLibraryPage { titles: p.items.into_iter().map(|i| i.title).collect(), continuation: p.continuation })
}

#[uniffi::export]
pub fn library_podcasts(session: Arc<MediyoSession>) -> Result<FfiLibraryPage, MediyoError> {
    let guard = session.inner.lock().unwrap();
    let resp = guard.request("browse", serde_json::json!({"browseId": "FEmusic_library_non_music_audio_list"}))?;
    let page = mediyo_core::model::library::parse_library_songs(&resp)?; // pod casts are grid like playlists
    Ok(FfiLibraryPage { titles: page.items.into_iter().map(|i| i.title).collect(), continuation: page.continuation })
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
pub fn add_many_to_playlist(session: Arc<MediyoSession>, playlist_id: String, video_ids: Vec<String>) -> Result<(), MediyoError> {
    let guard = session.inner.lock().unwrap();
    let refs: Vec<&str> = video_ids.iter().map(|s| s.as_str()).collect();
    mediyo_core::api::library::add_many_to_playlist(&guard, &playlist_id, &refs)?;
    Ok(())
}

#[uniffi::export]
pub fn create_playlist(session: Arc<MediyoSession>, title: String, description: String) -> Result<String, MediyoError> {
    let guard = session.inner.lock().unwrap();
    Ok(mediyo_core::api::library::create_playlist(&guard, &title, &description, "PRIVATE")?)
}
