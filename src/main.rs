#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate ncurses;

use ncurses::*;
use std::env;
use std::io::prelude::*;
use std::io::Result;
use std::net::TcpStream;

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

enum ServerMessageCode {
    Login = 1,
    SetListenPort = 2,
    GetPeerAddress = 3,
    WatchUser = 5,
    UnwatchUser = 6,
    GetUserStatus = 7,
    SayInChatRoom = 13,
    JoinRoom = 14,
    LeaveRoom = 15,
    UserJoinedRoom = 16,
    UserLeftRoom = 17,
    ConnectToPeer = 18,
    PrivateMessages = 22,
    AcknowledgePrivateMessage = 23,
    FileSearch = 26,
    SetOnlineStatus = 28,
    Ping = 32,
    SharedFoldersAndFiles = 35,
    GetUserStats = 36,
    KickedFromServer = 41,
    UserSearch = 42,
    RoomList = 64,
    GlobalAdminMessage = 66,
    PrivilegedUsers = 69,
    HaveNoParents = 71,
    ParentMinSpeed = 83,
    ParentSpeedRatio = 84,
    CheckPrivileges = 92,
    EmbeddedMessage = 93,
    AcceptChildren = 100,
    PossibleParents = 102,
    WishlistSearch = 103,
    WishlistInterval = 104,
    RoomTickers = 113,
    RoomTickerAdd = 114,
    RoomTickerRemove = 115,
    SetRoomTicker = 116,
    RoomSearch = 120,
    SendUploadSpeed = 121,
    GivePrivileges = 123,
    BranchLevel = 126,
    BranchRoot = 127,
    PrivateRoomUsers = 133,
    PrivateRoomAddUser = 134,
    PrivateRoomRemoveUser = 135,
    PrivateRoomDropMembership = 136,
    PrivateRoomDropOwnership = 137,
    PrivateRoomAdded = 139,
    PrivateRoomRemoved = 140,
    PrivateRoomToggle = 141,
    NewPassword = 142,
    PrivateRoomAddOperator = 143,
    PrivateRoomRemoveOperator = 144,
    PrivateRoomOperatorAdded = 145,
    PrivateRoomOperatorRemoved = 146,
    PrivateRoomOwned = 148,
    MessageUsers = 149,
    CantConnectToPeer = 1001,
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

struct Config<'a> {
    username: &'a str,
    password: &'a str,
    shared_folder: &'a str,
    download_folder: &'a str,
}

struct Client<'a> {
    host: &'a str,
    config: Config<'a>,
}
impl<'a> Client<'a> {
    fn new(
        username: &'a str,
        password: &'a str,
        shared_folder: &'a str,
        download_folder: &'a str,
    ) -> Self {
        Self {
            host: "server.slsknet.org: 2242",
            config: Config {
                username,
                password,
                shared_folder,
                download_folder,
            },
        }
    }

    fn connect(self, server_code: ServerMessageCode, value: u32) -> Result<()> {
        let mut stream = TcpStream::connect(self.host)?;


        let mut buffer = [0, 128];
        stream.read(&mut buffer)?;

        println!("Received: {:?}", String::from_utf8_lossy(&buffer[..]));

        Ok(())
    }
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
    let client = Client::new(
        args[0].as_str(),
        args[1].as_str(),
        args[2].as_str(),
        args[3].as_str(),
    );

    client.connect(ServerMessageCode::Login, 0).unwrap();
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
