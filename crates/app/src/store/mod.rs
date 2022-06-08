// SPDX-FileCopyrightText: 2022 Profian Inc. <opensource@profian.com>
// SPDX-License-Identifier: AGPL-3.0-only

mod entity;
mod node;
mod repo;
mod tag;

pub use entity::*;
pub use node::*;
pub use repo::*;
pub use tag::*;

use drawbridge_type::RepositoryName;

use cap_async_std::fs_utf8::Dir;

#[derive(Debug)]
pub struct Store {
    root: Dir,
}

impl Store {
    pub fn repository(&self, name: &RepositoryName) -> Repository<'_> {
        Repository::new(Entity::new(&self.root, ""), name)
    }
}

impl From<std::fs::File> for Store {
    fn from(dir: std::fs::File) -> Self {
        Store {
            root: Dir::from_std_file(dir.into()),
        }
    }
}
