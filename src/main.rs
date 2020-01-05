use chrono::prelude::*;
use docopt::Docopt;
use rusqlite::types::Null;
use rusqlite::{params, Connection};
use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;
use tempfile::NamedTempFile;

const USAGE: &str = "
dablog CLI

Usage:
  dablog init
  dablog create
  dablog read ID
  dablog update ID
  dablog delete ID
  dablog build
  dablog (-h | --help)
  dablog (-v | --version)

Options:
  -h --help     Show this screen.
  -v --version  Show version.
";

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Args {
    cmd_init: bool,
    cmd_create: bool,
    cmd_read: bool,
    cmd_update: bool,
    cmd_delete: bool,
    cmd_build: bool,
    flag_version: bool,
    arg_ID: isize,
}

/// Initialize a dablog in the current directory.alloc
///
/// Set up the database and create a "Hello, World!" post.
fn handle_init() {
    setup_database();
    create_test_post();
}

fn create_test_post() {
    let conn = Connection::open("dablog.db").unwrap();
    conn.execute(
        "INSERT INTO posts VALUES (?, ?, ?, ?)",
        params![
            Null,
            Utc::now().to_rfc3339(),
            "Hello, World!",
            "Hello, World! Welcome to my **dablog**."
        ],
    )
    .unwrap();
}

fn setup_database() {
    let conn = Connection::open("dablog.db").unwrap();
    conn.execute(
        "
    CREATE TABLE posts (
        id         INTEGER PRIMARY KEY AUTOINCREMENT,
        created_at TEXT NOT NULL,
        title      TEXT NOT NULL,
        body       TEXT NOT NULL
    )
    ",
        params![],
    )
    .unwrap();
}

fn launch_editor(path: &str) {
    let editor = env::var_os("EDITOR").unwrap();
    Command::new(editor).arg(path).status().unwrap();
}

fn handle_create() {
    let temp_file = NamedTempFile::new();
    let unwrapped = temp_file.unwrap();
    let path = unwrapped.path().to_str().unwrap();
    launch_editor(path);
    let contents = fs::read_to_string(path).unwrap();

    let conn = Connection::open("dablog.db").unwrap();
    conn.execute(
        "INSERT INTO posts VALUES (?, ?, ?, ?)",
        params![
            Null,
            Utc::now().to_rfc3339(),
            "Untitled post", // it's not possible to set a title yet
            contents
        ],
    )
    .unwrap();
}

fn handle_read(id: isize) {
    let conn = Connection::open("dablog.db").unwrap();
    conn.query_row("SELECT * FROM posts WHERE id = ?", params![id], |post| {
        let title: String = post.get_unwrap(2);
        let body: String = post.get_unwrap(3);

        println!("{}", title);
        println!("{}", body);

        Ok(())
    })
    .unwrap();
}

fn handle_update(id: isize) {
    unimplemented!()
}

fn handle_delete(id: isize) {
    let conn = Connection::open("dablog.db").unwrap();
    conn.execute("DELETE FROM posts WHERE ID = ?", params![id])
        .unwrap();
}

fn handle_build() {
    unimplemented!()
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match args {
        Args { cmd_init: true, .. } => handle_init(),
        Args {
            cmd_create: true, ..
        } => handle_create(),
        Args { cmd_read: true, .. } => handle_read(args.arg_ID),
        Args {
            cmd_update: true, ..
        } => handle_update(args.arg_ID),
        Args {
            cmd_delete: true, ..
        } => handle_delete(args.arg_ID),
        Args {
            cmd_build: true, ..
        } => handle_build(),
        Args {
            flag_version: true, ..
        } => {
            eprintln!("dablog v{}", env!("CARGO_PKG_VERSION"));
        }
        _ => unreachable!(),
    }
}
