use log::*;
use users;

#[derive(Debug)]
pub struct UserGroup {
    user: Option<String>,
    group: Option<String>,
}

impl Default for UserGroup {
    /// Returns current UserGroup
    fn default() -> UserGroup {
        let user = users::get_current_username()
            .or_else(|| {
                error!("unable get user name ");
                None
            })
            .and_then(|u| {
                u.to_str()
                    .or_else(|| {
                        error!("unable convert osstring to str user name");
                        None
                    })
                    .and_then(|s| Some(s.to_owned()))
            });
        let group = users::get_current_groupname()
            .or_else(|| {
                error!("unable get group name ");
                None
            })
            .and_then(|u| {
                u.to_str()
                    .or_else(|| {
                        error!("unable convert osstring to str group name");
                        None
                    })
                    .and_then(|s| Some(s.to_owned()))
            });
        UserGroup { user, group }
    }
}

impl UserGroup {
    pub fn get_user(&self) -> Option<String> {
        self.user.clone()
    }
    pub fn get_group(&self) -> Option<String> {
        self.group.clone()
    }
}
