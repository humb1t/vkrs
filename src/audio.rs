use std::borrow::Borrow;
use std::convert::AsRef;
use std::string::ToString;
use serde::de;
use api::{AlbumId, Bool, Collection, Duration, FullId, Group, Id, OwnerId, Sort, Timestamp};
use users::User;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Audio {
    pub id: Id,
    pub owner_id: OwnerId,
    pub artist: String,
    pub title: String,
    pub date: Timestamp,
    pub url: String, // Url !!!
    pub lyrics_id: Option<Id>,
    pub album_id: Option<Id>,
    pub genre_id: Option<Genre>,
    pub duration: Duration,
    #[serde(default)]
    pub no_search: Bool, // FIXME: bool
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Lyrics {
    pub lyrics_id: Id,
    pub text: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct Album {
    pub id: Id,
    pub owner_id: OwnerId,
    pub title: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Clone)]
pub struct UploadUrl {
    pub upload_url: String,
}

request! {
    #[derive(Eq)]
    struct Get for ["audio.get"](v => 5.37, need_user => 0) -> Collection<Audio> [Audio] {
        owner_id: OwnerId = () => {},
        album_id: Option<Id> = () => { |value| value.as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or("") },
        audio_ids: Vec<Id> = () => { Vec },
        offset: usize = (0) => {},
        count: usize = (100) => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Search for ["audio.search"](v => 5.44) -> Collection<Audio> [Audio] {
        sized {
            auto_complete: bool = () => {bool},
            lyrics: bool = () => {bool},
            performer_only: bool = () => {bool},
            sort: Sort = (Sort::Popularity) => {AsRef},
            search_own: bool = () => {bool},
            offset: usize = (0) => {},
            count: usize = (30) => {},
        }
        unsized {
            q: str = ("") => {=},
        }
    }
}

request! {
    struct GetUploadServer for ["audio.getUploadServer"](v => 5.44) -> UploadUrl [Audio];
}

request_ref! {
    #[derive(Eq, Copy)]
    struct Save for ["audio.save"](v => 5.44) -> Vec<Audio> [Audio] {
        sized {
            server: Id = () => {},
        }
        unsized {
            audio: str = ("") => {=},
            hash: str = ("") => {=},
            artist: str = ("") => {=},
            title: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Add for ["audio.add"](v => 5.44) -> Id [Audio] {
        audio_id: Id = () => {},
        owner_id: OwnerId = () => {},
        group_id: Option<Id> = () => {Option},
        album_id: Option<Id> = () => {Option},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct Delete for ["audio.delete"](v => 5.44) -> Bool [Audio] {
        audio_id: Id = () => {},
        owner_id: OwnerId = () => {},
    }
}

request_ref! {
    struct Edit for ["audio.edit"](v => 5.44) -> Id [Audio] {
        sized {
            owner_id: OwnerId = () => {},
            audio_id: Id = () => {},
            genre_id: Option<Genre> = () => {Option},
            no_search: bool = () => {bool},
        }
        unsized {
            artist: str = ("") => {=},
            title: str = ("") => {=},
            text: str = ("") => {=},
        }
    }
}


request! {
    struct Reorder for ["audio.reorder"](v => 5.44) -> Bool [Audio] {
        audio_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
        before: Option<Id> = () => {Option},
        after: Option<Id> = () => {Option},
    }
}

request! {
    struct Restore for ["audio.restore"](v => 5.44) -> Audio [Audio] {
        audio_id: Id = () => {},
        owner_id: Option<OwnerId> = () => {Option},
    }
}

request_ref! {
    #[derive(Copy, Eq)]
    struct GetById for ["audio.getById"](v => 5.44) -> Collection<Audio> [Audio] {
        audios: [FullId] = (&[][..]) => {Vec},
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetLyrics for ["audio.getLyrics"](v => 5.44) -> Lyrics [Audio] {
        lyrics_id: Id = () => {}
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetCount for ["audio.getCount"](v => 5.44) -> u64 [Audio] {
        owner_id: OwnerId = () => {}
    }
}

request! {
    #[derive(Copy, Eq)]
    struct GetAlbums for ["audio.getAlbums"](v => 5.44) -> Collection<Album> [Audio] {
        owner_id: OwnerId = () => {},
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct AddAlbum for ["audio.addAlbum"](v => 5.44) -> AlbumId [Audio] {
        sized {
            group_id: Option<Id> = () => {Option},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct EditAlbum for ["audio.editAlbum"](v => 5.44) -> Bool [Audio] {
        sized {
            group_id: Option<Id> = () => {Option},
            album_id: Id = () => {},
        }
        unsized {
            title: str = ("") => {=},
        }
    }
}

request! {
    #[derive(Eq, Copy)]
    struct DeleteAlbum for ["audio.deleteAlbum"](v => 5.44) -> Bool [Audio] {
        group_id: Option<Id> = () => {Option},
        album_id: Id = () => {},
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct MoveToAlbum for ["audio.moveToAlbum"](v => 5.44) -> Bool [Audio] {
        sized {
            group_id: Option<Id> = () => {Option},
            album_id: Id = () => {},
        }
        unsized {
            audio_ids: [Id] = (&[][..]) => {Vec},
        }
    }
}

request_ref! {
    #[derive(Eq, Copy)]
    struct SetBroadcast for ["audio.setBroadcast"](v => 5.44) -> Vec<Id> [Status] {
        sized {
            audio: FullId = () => {},
        }
        unsized {
            target_ids: [OwnerId] = (&[][..]) => {Vec},
        }
    }
}

// TODO: join up into a single request with Vec<enum { User, Group }> type?
/// Unstable: may be joined into `GetBroadcastList` with `GetGroupsBroadcastList`
request! {
    #[derive(Eq, Copy)]
    struct GetFriendsBroadcastList for ["audio.getBroadcastList"](v => 5.44, filter => "friends") -> Vec<User> {
        active: bool = () => {bool}
    }
}

/// Unstable: may be joined into `GetBroadcastList` with `GetFriendsBroadcastList`
request! {
    #[derive(Eq, Copy)]
    struct GetGroupsBroadcastList for ["audio.getBroadcastList"](v => 5.44, filter => "groups") -> Vec<Group> {
        active: bool = () => {bool}
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetPopular for ["audio.getPopular"](v => 5.44) -> Vec<Audio> [Audio] {
        only_eng: bool = () => {bool},
        genre_id: Option<Genre> = (None) => {
            |value| value.map(Into::<u32>::into).as_ref().map(ToString::to_string).as_ref().map(Borrow::borrow).unwrap_or("")
        },
        offset: usize = (0) => {},
        count: usize = (30) => {},
    }
}

request! {
    #[derive(Eq, Copy)]
    struct GetRecommendations for ["audio.getRecommendations"](v => 5.44) -> Collection<Audio> [Audio] {
        target_audio: Option<FullId> = () => {Option},
        user_id: Option<Id> = () => {Option},
        offset: usize = (0) => {},
        count: usize = (30) => {},
        shuffle: bool = () => {bool},
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Genre {
    Rock, // 1
    Pop, // 2
    RapHipHop, // 3
    EasyListen, // 4
    DanceHouse, // 5
    Instrumental, // 6
    Metal, // 7
    Alternative, // 21
    Dubstep, // 8
    JazzBlues, // 9
    DrumBass, // 10
    Trance, // 11
    Chanson, // 12
    Ethnic, // 13
    AcousticVocal, // 14
    Reggae, // 15
    Classical, // 16
    IndiePop, // 17
    Speech, // 19
    ElectropopDisco, // 22
    Other, // 18
    Unknown(u32),
}

impl fmt::Display for Genre {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Genre::*;
        match *self {
            Rock => f.write_str("rock"),
            Pop => f.write_str("pop"),
            RapHipHop => f.write_str("rap & hiphop"),
            EasyListen => f.write_str("easy listening"),
            DanceHouse => f.write_str("dance & house"),
            Instrumental => f.write_str("dance & house"),
            Metal => f.write_str("metal"),
            Alternative => f.write_str("alternative"),
            Dubstep => f.write_str("dubstep"),
            JazzBlues => f.write_str("jazz & blues"),
            DrumBass => f.write_str("drum & bass"),
            Trance => f.write_str("trance"),
            Chanson => f.write_str("chanson"),
            Ethnic => f.write_str("ethnic"),
            AcousticVocal => f.write_str("acoustic & vocal"),
            Reggae => f.write_str("reggae"),
            Classical => f.write_str("classical"),
            IndiePop => f.write_str("indie pop"),
            Speech => f.write_str("speech"),
            ElectropopDisco => f.write_str("electro pop & disco"),
            Other => f.write_str("other"),
            Unknown(id) => write!(f, "unknown (#{})", id),
        }
    }
}

impl<'de> de::Deserialize<'de> for Genre {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Genre, D::Error> {
        use self::Genre::*;
        de::Deserialize::deserialize(d).and_then(|v: u32| {
            match v {
                1 => Ok(Rock),
                2 => Ok(Pop),
                3 => Ok(RapHipHop),
                4 => Ok(EasyListen),
                5 => Ok(DanceHouse),
                6 => Ok(Instrumental),
                7 => Ok(Metal),
                21 => Ok(Alternative),
                8 => Ok(Dubstep),
                9 => Ok(JazzBlues),
                10 => Ok(DrumBass),
                11 => Ok(Trance),
                12 => Ok(Chanson),
                13 => Ok(Ethnic),
                14 => Ok(AcousticVocal),
                15 => Ok(Reggae),
                16 => Ok(Classical),
                17 => Ok(IndiePop),
                19 => Ok(Speech),
                22 => Ok(ElectropopDisco),
                18 => Ok(Other),
                v => Ok(Unknown(v)),
            }
        })
    }
}

impl Into<u32> for Genre {
    fn into(self) -> u32 {
        use self::Genre::*;
        match self {
            Rock => 1,
            Pop => 2,
            RapHipHop => 3,
            EasyListen => 4,
            DanceHouse => 5,
            Instrumental => 6,
            Metal => 7,
            Alternative => 21,
            Dubstep => 8,
            JazzBlues => 9,
            DrumBass => 10,
            Trance => 11,
            Chanson => 12,
            Ethnic => 13,
            AcousticVocal => 14,
            Reggae => 15,
            Classical => 16,
            IndiePop => 17,
            Speech => 19,
            ElectropopDisco => 22,
            Other => 18,
            Unknown(v) => v,
        }
    }
}

enum_str! { Filter {
    All = "all",
    Friends = "friends",
    Groups = "groups",
}}
