use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/**
 * ref: https://github.com/jhass/nodeinfo/blob/2.1/PROTOCOL.md
 */
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Discovery {
    pub links: Vec<DiscoveryItem>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DiscoveryItem {
    pub rel: String,
    pub href: String,
}

/**
 * ref: https://nodeinfo.diaspora.software/ns/schema/2.1
 */
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct NodeInfo {
    pub version: String,
    pub software: SoftwareItems,
    pub protocols: Vec<String>,
    pub services: ServicesItems,
    #[serde(rename = "openRegistrations")]
    pub open_registrations: bool,
    pub usage: UsageItems,
    pub metadata: MetadataItems,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct SoftwareItems {
    pub name: String,
    pub version: String,
    pub repository: Option<String>,
    pub homepage: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct ServicesItems {
    pub inbound: Vec<String>,
    pub outbound: Vec<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UsageItems {
    pub users: UsersItems,
    #[serde(rename = "localPosts")]
    pub local_posts: Option<usize>,
    #[serde(rename = "localComments")]
    pub local_comments: Option<usize>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct UsersItems {
    pub total: Option<usize>,
    #[serde(rename = "activeHalfyear")]
    pub active_halfyear: Option<usize>,
    #[serde(rename = "activeMonth")]
    pub active_month: Option<usize>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct MetadataItems {
    // Misskey extensions
    #[serde(rename = "nodeName")]
    pub node_name: Option<String>,
    #[serde(rename = "nodeDescription")]
    pub node_description: Option<String>,
    pub maintainer: Option<MaintainerItems>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct MaintainerItems {
    pub name: Option<String>,
    pub email: Option<String>,
}
