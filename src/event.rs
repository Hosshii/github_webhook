use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ID {
    St(String),
    Num(u64),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct URL(String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Base {
    id: ID,

    #[serde(rename = "type")]
    _type: EventType,

    actor: Actor,
    repo: Repository,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EventType {
    CommitCommentEvent,
    CreateEvent,
    DeleteEvent,
    ForkEvent,
    GollumEvent,
    IssueCommentEvent,
    IssuesEvent,
    MemberEvent,
    PublicEvent,
    PullRequestEvent,
    PullRequestReviewCommentEvent,
    PushEvent,
    ReleaseEvent,
    SponsorshipEvent,
    WatchEvent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Actor {
    id: ID,
    login: String,
    display_login: String,
    gravatar_id: ID,
    url: URL,
    avatar_url: URL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Repository {
    id: ID,
    name: String,
    url: URL,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Event {
    WatchEvent {
        #[serde(flatten)]
        base: Base,

        payload: (),
    },
}
