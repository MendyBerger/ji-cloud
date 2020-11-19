use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Types for user audio library.
pub mod user {
    use serde::{Deserialize, Serialize};
    use url::Url;

    use super::AudioId;

    /// Response for listing.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct ListResponse {
        /// the audio file returned.
        pub audio: Vec<GetResponse>,
    }

    /// Response for getting a single audio file.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetResponse {
        /// The audio file's metadata.
        pub metadata: UserAudio,

        /// A url that can be used to retrieve the audio file.
        pub url: Url,
    }

    /// Over the wire representation of an audio file's metadata.
    #[derive(Serialize, Deserialize, Debug)]
    pub struct UserAudio {
        /// The audio file's ID.
        pub id: AudioId,
        // more fields to be added
    }
}

/// Wrapper type around [`Uuid`], represents the ID of a image.
///
/// [`Uuid`]: ../../uuid/struct.Uuid.html
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(feature = "backend", sqlx(transparent))]
pub struct AudioId(pub Uuid);

into_uuid![AudioId];
