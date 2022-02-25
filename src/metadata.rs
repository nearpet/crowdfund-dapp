use crate::*;
use std::fmt;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub owner_id: AccountId
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonProject {
    pub owner_id: AccountId,
    pub project_id: ProjectId,
    pub metadata: ProjectMetadata,
}

impl fmt::Display for JsonProject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "owner_id: {}\nproject_id: {}\n, metadata: {}\n", self.owner_id, self.project_id, self.metadata)
    }
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectMetadata {
    pub title: String, // tên dự án
    pub description: String, // miêu tả dự án
    pub target: U128, // số tiền muốn gọi vốn
    pub minimum_pledge: U128, // số tiền tối thiểu một người có thể góp fund
    pub started_at: Option<Timestamp>, // project created time
    pub ended_at: Option<Timestamp>, // project deadline
    pub funded: Option<U128>, // số tiền đã gọi vốn
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
    pub extra: Option<String>, // anything extra the Project object wants to store on-chain. Can be stringified JSON.
    pub reference: Option<String>, // URL to an off-chain JSON file with more info.
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
}

impl fmt::Display for ProjectMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "title: {}\ndescription: {}\ntarget: {}\nminimum_pledge: {}\nfunded: {:#?}\nmedia: {:#?}\n",
            self.title, self.description, u128::from(self.target), u128::from(self.minimum_pledge), self.funded, self.media)
    }
}