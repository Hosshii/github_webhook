// This file is copied from https://github.com/softprops/afterparty/blob/master/src/events.rs.in and edited.

// ************************************************************************
// Copyright (c) 2015-2016 Doug Tangren

// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:

// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
// ************************************************************************

use case::CaseExt;
pub fn patch_payload_json(event: &str, payload: &str) -> String {
    let mut patched_payload = "{\"".to_string();
    patched_payload.push_str(event.to_camel().as_ref());
    patched_payload.push_str("\":");
    patched_payload.push_str(payload);
    patched_payload.push_str("}");
    patched_payload
}

use serde::Deserialize;
#[derive(Clone, Debug, Deserialize, Default)]
pub struct Value {
    pub json: serde_json::Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommitCommentEvent {
    pub action: CommitCommentAction,
    pub comment: Comment,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateEvent {
    pub description: String,
    pub master_branch: String,
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    #[serde(rename = "ref_type")]
    pub ref_type: String,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeleteEvent {
    pub pusher_type: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub ref_type: String,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentEvent {
    pub deployment: Deployment,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DeploymentStatusEvent {
    pub deployment: Deployment,
    pub deployment_status: DeploymentStatus,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ForkEvent {
    pub forkee: Repository,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GollumEvent {
    pub pages: Vec<Pages>,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Clone, Debug, Deserialize)]
pub struct IssueCommentEvent {
    pub action: IssueCommentAction,
    pub comment: IssueCommentComment,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct IssuesEvent {
    pub action: IssuesAction,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MemberEvent {
    pub action: String,
    pub member: User,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MembershipEvent {
    pub action: String,
    pub member: User,
    pub organization: Organization,
    pub scope: String,
    pub sender: User,
    pub team: Team,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PageBuildEvent {
    pub build: PageBuild,
    pub id: u64,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PingEvent {
    pub hook: Hook,
    pub hook_id: u64,
    pub repository: Repository,
    pub sender: User,
    pub zen: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PublicEvent {
    pub hook: Hook,
    pub hook_id: u64,
    pub repository: Repository,
    pub sender: User,
    pub zen: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestEvent {
    pub action: PullRequestAction,
    pub number: u64,
    pub pull_request: PullRequestDetails,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewEvent {
    pub action: PullRequestReviewAction,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub review: Review,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PullRequestReviewCommentEvent {
    pub action: PullRequestReviewCommentAction,
    pub comment: PullRequestReviewComment,
    pub pull_request: PullRequest,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PushEvent {
    pub after: String,
    pub base_ref: Option<String>,
    pub before: String,
    pub commits: Vec<CommitStats>,
    pub compare: String,
    pub created: bool,
    pub deleted: bool,
    pub forced: bool,
    pub head_commit: Option<CommitStats>,
    pub pusher: UserRef, // note there aren't may fields here
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repository: PushRepository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseEvent {
    pub action: String,
    pub release: Release,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RepositoryEvent {
    pub action: String,
    pub organization: Organization,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StatusEvent {
    //branches: Vec<BranchRef>,
    pub commit: CommitRef,
    pub context: String,
    pub created_at: String,
    pub description: Option<String>,
    pub id: u64,
    pub name: String,
    pub repository: Repository,
    pub sender: User,
    pub sha: String,
    pub state: String,
    pub target_url: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamAddEvent {
    pub organization: Organization,
    pub repository: Repository,
    pub sender: User,
    pub team: Team,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WatchEvent {
    pub action: String,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Clone, Debug, Deserialize)]
pub enum Event {
    CommitComment(CommitCommentEvent),
    Create(CreateEvent),
    Delete(DeleteEvent),
    Deployment(DeploymentEvent),
    DeploymentStatus(DeploymentStatusEvent),
    Fork(ForkEvent),
    Gollum(GollumEvent),
    IssueComment(IssueCommentEvent),
    Issues(IssuesEvent),
    Member(MemberEvent),
    Membership(MembershipEvent),
    PageBuild(PageBuildEvent),
    Ping(PingEvent),
    Public(PublicEvent),
    PullRequest(PullRequestEvent),
    PullRequestReview(PullRequestReviewEvent),
    PullRequestReviewComment(PullRequestReviewCommentEvent),
    Push(PushEvent),
    Release(ReleaseEvent),
    Repository(RepositoryEvent),
    Status(StatusEvent),
    TeamAdd(TeamAddEvent),
    Watch(WatchEvent),
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Commit {
    author: GitUser,
    committer: GitUser,
    message: String,
    tree: GitRef,
    url: String,
    comment_count: u64,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct BranchRef {
    pub commit: GitRef,
    pub name: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PageBuild {
    pub commit: String,
    pub created_at: String,
    pub duration: u64,
    pub error: Error,
    pub pusher: User,
    pub status: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Comment {
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub html_url: String,
    pub id: u64,
    pub line: Option<String>,
    pub path: Option<String>,
    pub position: Option<String>,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct CommitRef {
    pub author: User,
    pub comments_url: String,
    pub commit: Commit,
    pub committer: User,
    pub html_url: String,
    pub parents: Vec<GitRef>,
    pub sha: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Deployment {
    pub created_at: String,
    pub creator: User,
    pub description: Option<String>,
    pub environment: String,
    pub id: u64,
    pub payload: Value,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repository_url: String,
    pub sha: String,
    pub statuses_url: String,
    pub task: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct DeploymentStatus {
    pub created_at: String,
    pub creator: User,
    pub deployment_url: String,
    pub description: Option<String>,
    pub id: u64,
    pub repository_url: String,
    pub state: String,
    pub target_url: Option<String>,
    pub updated_at: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct CommitStats {
    pub added: Vec<String>,
    pub author: GitUser,
    pub committer: GitUser,
    pub distinct: bool,
    pub id: String,
    pub message: String,
    pub modified: Vec<String>,
    pub removed: Vec<String>,
    pub timestamp: String,
    pub tree_id: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Hook {
    pub active: bool,
    pub config: Config,
    pub created_at: String,
    pub events: Vec<String>,
    pub id: u64,
    pub last_response: LastResponse,
    pub name: String,
    pub ping_url: String,
    pub test_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub updated_at: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Issue {
    pub assignee: Option<Assignee>,
    pub assignees: Vec<Assignee>,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments: u64,
    pub comments_url: String,
    pub created_at: String,
    pub events_url: String,
    pub html_url: String,
    pub id: u64,
    pub labels: Vec<Label>,
    pub labels_url: String,
    pub locked: bool,
    pub milestone: Option<MileStone>,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct IssueCommentComment {
    pub body: String,
    pub created_at: String,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Organization {
    pub avatar_url: String,
    pub events_url: String,
    pub id: u64,
    pub login: String,
    pub members_url: String,
    pub public_members_url: String,
    pub repos_url: String,
    pub url: String,
    pub description: Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Pages {
    pub action: String,
    pub html_url: String,
    pub page_name: String,
    pub sha: String,
    pub summary: Option<String>,
    pub title: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestDetails {
    pub _links: PullRequestLinks,
    pub assignee: Option<Assignee>,
    pub base: PullSource,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullSource,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub locked: bool,
    pub merge_commit_sha: Option<String>,
    pub merged_at: Option<String>,
    pub milestone: Option<String>,
    pub number: u64,
    pub patch_url: String,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: String,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
    pub merged: bool,
    pub mergeable: Option<bool>,
    pub mergeable_state: String,
    pub merged_by: Option<MergedBy>,
    pub comments: u64,
    pub review_comments: u64,
    pub commits: u64,
    pub additions: u64,
    pub deletions: u64,
    pub changed_files: u64,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequest {
    pub _links: PullRequestLinks,
    pub assignee: Option<Assignee>,
    pub base: PullSource,
    pub body: Option<String>,
    pub closed_at: Option<String>,
    pub comments_url: String,
    pub commits_url: String,
    pub created_at: String,
    pub diff_url: String,
    pub head: PullSource,
    pub html_url: String,
    pub id: u64,
    pub issue_url: String,
    pub locked: bool,
    pub merge_commit_sha: String,
    pub merged_at: Option<String>,
    pub milestone: Option<String>,
    pub number: u64,
    pub patch_url: String,
    pub review_comment_url: String,
    pub review_comments_url: String,
    pub state: String,
    pub statuses_url: String,
    pub title: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Review {
    pub id: i64,
    pub node_id: String,
    pub user: User,
    pub body: Option<String>,
    pub submitted_at: String,
    pub state: String,
    pub html_url: String,
    pub pull_request_url: String,
    pub _links: PullRequestReviewLinks,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestReviewLinks {
    pub html: Link,
    pub pull_request: Link,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestReviewComment {
    #[serde(rename = "_links")]
    pub _links: PullRequestReviewCommentLinks,
    pub body: String,
    pub commit_id: String,
    pub created_at: String,
    pub diff_hunk: String,
    pub html_url: String,
    pub id: u64,
    pub original_commit_id: String,
    pub original_position: u64,
    pub path: String,
    pub position: u64,
    pub pull_request_url: String,
    pub updated_at: String,
    pub url: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Release {
    pub assets: Vec<String>,
    pub assets_url: String,
    pub author: User,
    pub body: Option<String>,
    pub created_at: String,
    pub draft: bool,
    pub html_url: String,
    pub id: u64,
    pub name: Option<String>,
    pub prerelease: bool,
    pub published_at: String,
    pub tag_name: String,
    pub tarball_url: String,
    pub target_commitish: String,
    pub upload_url: String,
    pub url: String,
    pub zipball_url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct UserRef {
    pub name: String,
    pub email: Option<String>,
}

/// differs from Repository in owner type and some timestamp field types
#[derive(Clone, Default, Debug, Deserialize)]
pub struct PushRepository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: u64,
    pub default_branch: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub fork: bool,
    pub forks_count: u64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: u64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub notifications_url: String,
    pub open_issues: u64,
    pub open_issues_count: u64,
    pub owner: UserRef,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: u64,
    pub releases_url: String,
    pub size: u64,
    pub ssh_url: String,
    pub stargazers_count: u64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub watchers: u64,
    pub watchers_count: u64,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Repository {
    pub archive_url: String,
    pub assignees_url: String,
    pub blobs_url: String,
    pub branches_url: String,
    pub clone_url: String,
    pub collaborators_url: String,
    pub comments_url: String,
    pub commits_url: String,
    pub compare_url: String,
    pub contents_url: String,
    pub contributors_url: String,
    pub created_at: String,
    pub default_branch: String,
    pub description: Option<String>,
    pub downloads_url: String,
    pub events_url: String,
    pub forks: u64,
    pub forks_count: u64,
    pub forks_url: String,
    pub full_name: String,
    pub git_commits_url: String,
    pub git_refs_url: String,
    pub git_tags_url: String,
    pub git_url: String,
    pub has_downloads: bool,
    pub has_issues: bool,
    pub has_pages: bool,
    pub has_wiki: bool,
    pub homepage: Option<String>,
    pub hooks_url: String,
    pub html_url: String,
    pub id: u64,
    pub issue_comment_url: String,
    pub issue_events_url: String,
    pub issues_url: String,
    pub keys_url: String,
    pub labels_url: String,
    pub language: Option<String>,
    pub languages_url: String,
    pub merges_url: String,
    pub milestones_url: String,
    pub mirror_url: Option<String>,
    pub name: String,
    pub notifications_url: String,
    pub open_issues: u64,
    pub open_issues_count: u64,
    pub owner: User,
    pub private: bool,
    pub pulls_url: String,
    pub pushed_at: String,
    pub releases_url: String,
    pub size: u64,
    pub ssh_url: String,
    pub stargazers_count: u64,
    pub stargazers_url: String,
    pub statuses_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub svn_url: String,
    pub tags_url: String,
    pub teams_url: String,
    pub trees_url: String,
    pub updated_at: String,
    pub url: String,
    pub watchers: u64,
    pub watchers_count: u64,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Team {
    pub id: u64,
    pub members_url: String,
    pub name: String,
    pub permission: String,
    pub repositories_url: String,
    pub slug: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct GitUser {
    pub email: String,
    pub name: String,
    pub username: Option<String>,
    pub date: Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Config {
    pub content_type: String,
    pub insecure_ssl: String,
    pub secret: Option<String>,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Error {
    pub message: Option<String>,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullSource {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Label {
    pub color: String,
    pub name: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct LastResponse {
    pub code: Option<String>,
    pub message: Option<String>,
    pub status: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestLinks {
    pub comments: Link,
    pub commits: Link,
    pub html: Link,
    pub issue: Link,
    pub review_comment: Link,
    pub review_comments: Link,
    #[serde(rename = "self")]
    pub _self: Link,
    pub statuses: Link,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestInnerBase {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestInnerHead {
    pub label: String,
    #[serde(rename = "ref")]
    pub _ref: String,
    pub repo: Repository,
    pub sha: String,
    pub user: User,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct PullRequestReviewCommentLinks {
    pub html: Link,
    pub pull_request: Link,
    #[serde(rename = "self")]
    pub _self: Link,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct User {
    pub avatar_url: String,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub gravatar_id: String,
    pub html_url: String,
    pub id: u64,
    pub login: String,
    pub organizations_url: String,
    pub received_events_url: String,
    pub repos_url: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Link {
    pub href: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct GitRef {
    pub sha: String,
    pub url: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct MileStone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: i64,
    pub node_id: String,
    pub number: i64,
    pub state: String,
    pub title: String,
    pub description: String,
    pub creator: User,
    pub open_issues: i64,
    pub closed_issues: i64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: String,
    pub due_on: String,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct Assignee {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
}

#[derive(Clone, Default, Debug, Deserialize)]
pub struct MergedBy {
    pub login: String,
    pub id: i64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub site_admin: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitCommentAction {
    Created,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssueCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuesAction {
    Opened,
    Edited,
    Deleted,
    Pinned,
    Unpinned,
    Closed,
    Reopened,
    Assigned,
    Unassigned,
    Labeled,
    Locked,
    Unlocked,
    Transferred,
    Milestoned,
    Demilestoned,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewCommentAction {
    Created,
    Edited,
    Deleted,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestReviewAction {
    Submitted,
    Edited,
    Dismissed,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PullRequestAction {
    Opened,
    Edited,
    Closed,
    Assigned,
    Unassigned,
    ReviewRequested,
    ReviewRequestRemoved,
    ReadyForReview,
    ConvertToDraft,
    Labeled,
    Unlabeled,
    Synchronize,
    AutoMergeEnabled,
    AutoMergeDisabled,
    Locked,
    Unlocked,
    Reopened,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_event_deserialize() {
        let events = [
            "commit_comment",
            "issues",
            "issue_comment",
            "push",
            "pull_request",
            "pull_request_review",
            "pull_request_review_comment",
            "ping",
        ];

        for event in events.iter() {
            let filename = format!("data/{}.json", event);
            let content = fs::read_to_string(&filename).unwrap();
            let patched = patch_payload_json(event, &content);
            match serde_json::from_str::<Event>(&patched) {
                Ok(_) => (),
                Err(e) => {
                    println!("filename: {}", filename);
                    println!("{}", e);
                    panic!()
                }
            }
        }
    }
}
