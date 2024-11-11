//! crate documentation, this is where a brief description of the crate should go. not to be confused with the [args::Command] documentation
#![feature(doc_cfg)]
#![deny(missing_docs)]
#![cfg(not(windows))]

/// trick rustdoc into documenting the standard library
pub extern crate std;
extern crate clap;
extern crate cursive;
mod args;

/// String path to the database.
pub const DB_PATH: &str = "/var/goofy-bbs/bbs.db";

use std::{
    sync::RwLock,
    ops::*,
};
use cursive::{
    View,         // thingy for rendering things
    event::Event, // thingy for things happening
    Cursive,      // thingy for obscuring handwriting
    CursiveExt,   // thingy for things actually working
    With as _,    // thingy for withing
};
use rusqlite::{
    Connection,
    OpenFlags,
};
use clap::Parser;


/// Primary entrypoint function called on start.
pub fn main() {
    use args::Command::{self, *};
    let args = Command::parse();
    match args {
        Install => todo!(),
        InitDb { uid, wipe } => {
            //use posix_acl::{PosixACL, Qualifier, ACL_READ, ACL_WRITE};
            use std::fs::{create_dir_all, Permissions, set_permissions, remove_file};
            use std::os::unix::fs::*;
            create_dir_all("/var/goofy-bbs").expect("creating workdir");
            chown("/var/goofy-bbs", Some(0), Some(0)).expect("chowning workdir");
            set_permissions("/var/goofy-bbs", Permissions::from_mode(0o5700)).expect("chmodding workdir");
            if wipe { remove_file(DB_PATH).expect("deleting database") }
            let mut db = Connection::open_with_flags(DB_PATH, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE).expect("database connection failed");
            chown("/var/goofy-bbs", Some(uid), Some(0)).expect("chowning db");
            set_permissions("/var/goofy-bbs", Permissions::from_mode(0o0600)).expect("chmodding db");
            //let mut acl = PosixACL::new(0o0600);
            //acl.set(Qualifier::User(uid), ACL_READ | ACL_WRITE);
            //acl.write_acl(DB_PATH).expect("setacl db");
            db.execute(r#"
                CREATE TABLE IF NOT EXISTS users (
                    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                    username VARCHAR(255) NOT NULL PRIMARY KEY,
                    password VARCHAR(255),
                );
                CREATE TABLE IF NOT EXISTS keys (
                    user INTEGER FOREIGN KEY REFERENCES users(id),
                    key VARCHAR(1024) NOT NULL PRIMARY KEY
                );
                CREATE TABLE IF NOT EXISTS utags (
                    owner INTEGER FOREIGN KEY REFERENCES users(id),
                    name VARCHAR(16),
                );
                CREATE TABLE IF NOT EXISTS posts (
                    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
                    parent INTEGER NULL FOREIGN KEY REFERENCES
                    body TEXT NOT NULL,
                );
            "#, ()).expect("setting up db");
        },
        Client { user } => {
            let mut siv = Cursive::new();
            let mut db = Connection::open_with_flags(DB_PATH, OpenFlags::SQLITE_OPEN_READ_WRITE);
            let user = user.unwrap_or_else(|| todo!());
            siv.run();
        },
        User { .. } => todo!(),
    }
}

