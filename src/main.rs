#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;
use std::env;
use std::io::prelude::*;
use std::io::Result;

mod client;
use client::{ Client, ServerMessageCode };

enum ConnectionType {
    P, // Peer-to-peer
    F, // File Transfer
    D, // Distributed Network
}

enum UserStatus {
    Offline,
    Away,
    Online,
}

enum FileTypeAttributes {
    Bitrate,        // kbps
    Duration,       // seconds
    VBR,            // 0 or 1
    Encoder,        // unused
    SampleRate,     // Hz
    BitDepth        // bits
}

enum TransferDirection {
    DownloadFromPeer,
    UploadToPeer,
}

enum TransferStatus {
    Banned,
    Cancelled,
    Complete,
    DisallowedExtension,
    FileNotShared,
    PendingShutdown,
    Queued,
    RemoteFileError,
    TooManyFiles,
    TooManyMegabytes,
}

impl TransferStatus {
    fn value(&self) -> &'static str {
        match *self {
            TransferStatus::Banned => "Banned",
            TransferStatus::Cancelled => "Cancelled",
            TransferStatus::Complete => "Complete",
            TransferStatus::DisallowedExtension => "Disallowed extension",
            TransferStatus::FileNotShared => "File not shared.",
            TransferStatus::PendingShutdown => "Pending shutdown.",
            TransferStatus::Queued => "Queued",
            TransferStatus::RemoteFileError => "Remote file error",
            TransferStatus::TooManyFiles => "Too many files",
            TransferStatus::TooManyMegabytes => "Too many megabytes",
        }
    }
}

struct SlskProto {
    connection_type: ConnectionType,
}

struct UIState {
    page: u8,
    is_auth: bool,
}

struct UI {
    state: UIState,
}

impl UI {
    fn new() -> Self {
        Self {
            state: UIState {
                page: 0,
                is_auth: false,
            },
        }
    }

    fn init(self) -> Result<()> {
        initscr();
        keypad(stdscr(), true);

        noecho();

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut client = Client::new(
        args[0].as_str(),
        args[1].as_str(),
        args[2].as_str(),
        args[3].as_str(),
    );

    client.connect().unwrap();

    //let ui = UI::new();
    //ui.init().unwrap();

    //loop {
    //    let ch = getch();
    //    if ch == 'q' as i32 {
    //        break;
    //    }
    //}

    //endwin();
}
