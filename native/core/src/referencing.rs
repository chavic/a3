use std::borrow::Cow;

use matrix_sdk::ruma::{EventId, OwnedEventId, OwnedRoomId, RoomId};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Display, EnumString, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum SectionIndex {
    #[strum(serialize = "news", serialize = "boosts")]
    Boosts = 0,
    Calendar,
    Pins,
    Stories,
    Tasks,
}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Display, EnumString, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum ObjectListIndex {
    Attachments,
    Comments,
    Reactions,
    ReadReceipt,
    Rsvp,
    Tasks,
    Invites,
}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Display, EnumString, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
#[repr(u8)]
pub enum SpecialListsIndex {
    MyOpenTasks,
    MyDoneTasks,
    InvitedTo,
    #[cfg(any(test, feature = "testing"))]
    Test1,
    #[cfg(any(test, feature = "testing"))]
    Test2,
    #[cfg(any(test, feature = "testing"))]
    Test3,
}

// We organize our Index by typed keys
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Serialize, Deserialize)]
pub enum IndexKey {
    RoomHistory(OwnedRoomId),
    RoomModels(OwnedRoomId),
    ObjectHistory(OwnedEventId),
    Section(SectionIndex),
    RoomSection(OwnedRoomId, SectionIndex),
    ObjectList(OwnedEventId, ObjectListIndex),
    Special(SpecialListsIndex),
    Redacted,
    AllHistory,
}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Display, EnumString, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum ModelParam {
    CommentsStats,
    AttachmentsStats,
    ReactionStats,
    RsvpStats,
    #[strum(to_string = "read_receipts")]
    ReadReceiptsStats,
    #[strum(to_string = "invites")]
    InviteStats,
}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Display, EnumString, Serialize, Deserialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum RoomParam {
    LatestMessage,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Clone, Serialize, Deserialize)]
pub enum ExecuteReference {
    Index(IndexKey),
    Model(OwnedEventId),
    Room(OwnedRoomId),
    RoomAccountData(OwnedRoomId, Cow<'static, str>),
    ModelParam(OwnedEventId, ModelParam),
    RoomParam(OwnedRoomId, RoomParam),
    AccountData(Cow<'static, str>),
    ModelType(Cow<'static, str>),
}

impl ExecuteReference {
    pub fn as_storage_key(&self) -> String {
        match self {
            ExecuteReference::Model(owned_event_id) => format!("acter::{owned_event_id}"),
            ExecuteReference::ModelParam(owned_event_id, model_param) => {
                format!("{owned_event_id}::{model_param}")
            }
            ExecuteReference::RoomParam(owned_room_id, room_param) => {
                format!("{owned_room_id}::{room_param}")
            }
            ExecuteReference::ModelType(model_type) => model_type.to_string(),
            ExecuteReference::Index(IndexKey::Special(SpecialListsIndex::InvitedTo)) => {
                "global_invited".to_owned() // this is a special case, we actively store and manage
            }
            // not actually supported
            ExecuteReference::Index(_index_key) => todo!(),
            ExecuteReference::Room(_owned_room_id) => todo!(),
            ExecuteReference::RoomAccountData(_owned_room_id, _cow) => todo!(),
            ExecuteReference::AccountData(_cow) => todo!(),
        }
    }
}

impl From<&'static str> for ExecuteReference {
    fn from(value: &'static str) -> Self {
        ExecuteReference::ModelType(Cow::Borrowed(value))
    }
}

impl From<&EventId> for ExecuteReference {
    fn from(value: &EventId) -> Self {
        ExecuteReference::Model(value.to_owned())
    }
}

impl From<OwnedEventId> for ExecuteReference {
    fn from(value: OwnedEventId) -> Self {
        ExecuteReference::Model(value)
    }
}

impl From<&RoomId> for ExecuteReference {
    fn from(value: &RoomId) -> Self {
        ExecuteReference::Room(value.to_owned())
    }
}

impl From<OwnedRoomId> for ExecuteReference {
    fn from(value: OwnedRoomId) -> Self {
        ExecuteReference::Room(value)
    }
}

impl From<IndexKey> for ExecuteReference {
    fn from(value: IndexKey) -> Self {
        ExecuteReference::Index(value)
    }
}

impl From<SectionIndex> for ExecuteReference {
    fn from(value: SectionIndex) -> Self {
        ExecuteReference::Index(IndexKey::Section(value))
    }
}
