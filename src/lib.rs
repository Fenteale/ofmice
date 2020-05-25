//! The backend stuff, shared by the ofmice launcher and the ofpatchtool
#![deny(clippy::all)]
pub mod platform;
pub mod download;
pub mod installation;
pub mod translate;
#[cfg(feature = "steam_wrangler")]
pub mod steam_wrangler;
pub mod progress;

#[derive(Debug, Clone, Copy)]
pub enum WranglerError{
    SteamNotRunning,
    SSDKNotInstalled,
    TF2NotInstalled,
}