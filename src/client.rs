use std::io::{ BufReader, Result, Write };
use std::str;
use std::net::TcpStream;
use std::io::prelude::*;

pub enum ServerMessageCode {
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

pub struct Config<'a> {
    username: &'a str,
    password: &'a str,
    shared_folder: &'a str,
    download_folder: &'a str,
}

pub struct Client<'a> {
    pub connection: Option<TcpStream>,
    config: Config<'a>,
}
impl<'a> Client<'a> {
    pub fn new(
        username: &'a str,
        password: &'a str,
        shared_folder: &'a str,
        download_folder: &'a str,
    ) -> Self {
        Self {
            connection: None,
            config: Config {
                username,
                password,
                shared_folder,
                download_folder,
            },
        }
    }

    pub fn connect(&mut self) -> Result<()> {
        let mut stream = TcpStream::connect("servnet.org:2242").unwrap();

        let input = "1";
        let mut buffer: Vec<u8> = Vec::new();
        stream.write(input.as_bytes()).unwrap();

        let mut reader = BufReader::new(&stream);
        reader.read_until(b'\n', &mut buffer).unwrap();
        println!("{}", str::from_utf8(&buffer).unwrap());

        Ok(())
    }
}
