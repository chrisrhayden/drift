///! # drift
///!
///! a music daemon using [rodio](git@github.com:chrisrhayden/drift.git) unix sockets
///!
///! it provides two binaries, drift (the daemon) and driftcli

extern crate rodio;

pub mod app;
pub mod daemon;
pub mod events;
pub mod status;
pub mod player;
