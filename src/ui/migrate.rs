//! UI definitions for explore
use std::{fs, path::{Path, PathBuf}, char::decode_utf16};

use bevy::prelude::*;
use regex::bytes::{Regex, Captures};
use zstd::zstd_safe::WriteBuf;
use hex::FromHex;
use crate::typedef::{state::AppState, migration::facebook::{comments, self}, event::SyncFragments, resource::FragmentClone, component::{FragmentContents, Fragment}};

use super::inner::use_default_2d_camera;

pub fn migrate_systems_enter () -> SystemSet {
    return SystemSet::on_enter(AppState::Migrate).with_system(use_default_2d_camera).with_system(migrate_enter);
}

pub fn migrate_systems_exit () -> SystemSet {
    return SystemSet::on_exit(AppState::Migrate).with_system(migrate_exit);
}

pub fn migrate_systems_update () -> SystemSet {
    return SystemSet::on_update(AppState::Migrate).with_system(migrate_update).with_system(system_drag_and_drop);
}

fn migrate_enter () {

}

fn migrate_exit () {

}

fn migrate_update () {

}

fn try_facebook (path_buf: &PathBuf) -> Option<(PathBuf, PathBuf)> {
    let path_comments = path_buf.join("comments_and_reactions").join("comments.json");
    let path_posts = path_buf.join("posts").join("your_posts_1.json");
    if path_comments.exists() && path_posts.exists() {
        return Some((path_comments, path_posts));
    } else {
        return None;
    }
}

fn hello (str: Vec<u8>) -> String {
    let re = Regex::new(r"\\u00([\da-f]{2})").unwrap();
    let b = re.replace_all(str.as_slice(), |x: &Captures| {
        let y = Vec::from_hex(String::from_utf8(x[1].to_vec()).unwrap()).unwrap();
        y
    });
    String::from_utf8_lossy(&b).to_string()
}

/// Event listener for file drag and drop event.
fn system_drag_and_drop(
    mut dnd_ev: EventReader<FileDragAndDrop>,
    mut frag_ev: EventWriter<SyncFragments>
) {
    for ev in dnd_ev.iter() {
        println!("{:?}", ev);
        match ev {
            FileDragAndDrop::DroppedFile { id, path_buf  } => {
                if let Some((path_comments, path_posts)) = try_facebook(path_buf) {
                    let str_comment = hello(fs::read(path_comments).unwrap());
                    fs::write(PathBuf::new().join("hellohello"), str_comment.clone());
                    let res_comment: Result<facebook::comments::Root, serde_json::Error> = serde_json::from_str(&str_comment);
                    let str_posts = hello(fs::read(path_posts).unwrap());
                    let res_posts: Result<facebook::posts::Root, serde_json::Error> = serde_json::from_str(&str_posts);

                    
                    if let Ok(comments) = res_comment {
                        for c in comments.comments_v2 {
                            for d in c.data {
                                println!("Timestamp {} Comment: {}", d.comment.timestamp, d.comment.comment);
                                frag_ev.send(SyncFragments {
                                    original_entries: Vec::new(),
                                    entry_clone: d.comment.comment.split("\n").filter(|x|!x.is_empty())
                                    .map(|x| FragmentClone::Modified { fragment: Fragment {
                                        timestamp: d.comment.timestamp as u64,
                                        contents: FragmentContents::TextData { data: x.into() },
                                    }, original_id: None }).collect(),
                                });
                            }
                        }
                    }

                    
                    if let Ok(posts) = res_posts {
                        for p in posts {
                            for d in p.data {
                                println!("Timestamp {:?} Post: {}", d.clone().update_timestamp, d.clone().post.unwrap_or("".into()));

                                frag_ev.send(SyncFragments {
                                    original_entries: Vec::new(),
                                    entry_clone: d.post.unwrap_or("".into()).split("\n").filter(|x|!x.is_empty())
                                    .map(|x| FragmentClone::Modified { fragment: Fragment {
                                        timestamp: d.update_timestamp.unwrap_or(0) as u64,
                                        contents: FragmentContents::TextData { data: x.into() },
                                    }, original_id: None }).collect(),
                                });
                            }
                        }
                    }
                }
            }
            FileDragAndDrop::HoveredFile { id, path_buf  } => {
                if try_facebook(path_buf).is_some() {
                    println!("Facebook detected!");
                }
            },
            FileDragAndDrop::HoveredFileCancelled { .. } => {},
        }
    }
}