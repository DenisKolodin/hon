use std::mem;
use permission::HasPermission;
use authorize::{Role, Authorize};
use mould::{Session, Builder};
use mould_file::FileAccessPermission;
use mould_nfd::DialogPermission;

pub enum UserRole {
    Administrator,
}

impl Role for UserRole { }

pub struct UserSession {
    role: Option<UserRole>,
}

impl Session for UserSession { }

impl UserSession {
    fn authorized(&self) -> bool {
        match self.role {
            Some(UserRole::Administrator) => true,
            None => false,
        }
    }
}

impl Authorize<UserRole> for UserSession {
    fn set_role(&mut self, role: Option<UserRole>) -> Option<UserRole> {
        mem::replace(&mut self.role, role)
    }
}

impl<'a> HasPermission<FileAccessPermission<'a>> for UserSession {
    fn has_permission(&self, _: &FileAccessPermission) -> bool {
        self.authorized()
    }
}

impl HasPermission<DialogPermission> for UserSession {
    fn has_permission(&self, _: &DialogPermission) -> bool {
        self.authorized()
    }
}

pub struct UserBuilder { }

impl UserBuilder {
    pub fn new() -> Self {
        UserBuilder { }
    }
}

impl Builder<UserSession> for UserBuilder {
    fn build(&self) -> UserSession {
        UserSession {
            role: None,
        }
    }
}
