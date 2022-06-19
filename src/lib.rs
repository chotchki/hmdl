pub mod dns;
pub mod web;

use git_version::git_version;
pub const GIT_VERSION: &str = git_version!();
