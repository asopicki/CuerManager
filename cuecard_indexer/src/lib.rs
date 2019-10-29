extern crate diesel;
extern crate regex;
extern crate serde_json;
extern crate walkdir;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate cuer_database;
extern crate filetime;
extern crate uuid as uuidcrate;

use self::cuer_database::*;
use self::diesel::prelude::*;
use self::models::*;
use self::walkdir::{DirEntry, WalkDir};
use filetime::{set_file_mtime, FileTime};
use regex::Regex;
use uuidcrate::Uuid;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::vec::Vec;

pub struct Config {
    pub basepath: String,
    pub database_url: String,
}

struct IndexFile {
    path: DirEntry,
    content: String,
    meta: HashMap<String, String>,
    audio_file: String,
    file_path: String
}

impl IndexFile {
    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    fn set_meta(&mut self, key: &str, value: &str) {
        self.meta.insert(String::from(key), String::from(value));
    }

    fn get_meta(&self, key: String) -> Option<&String> {
        self.meta.get(&key)
    }

    fn index_file(&self) -> Option<PathBuf> {
        let mut filename = ".de.sopicki.cuelib.".to_string();
        filename.push_str(self.path.file_name().to_str().unwrap());
        filename = filename.to_string();
        let path = Path::new(&filename).to_owned();
        let parent = self.path.path().parent().unwrap();
        Some(parent.join(&path))
    }
}

fn is_allowed(filename: &str) -> bool {
    if filename.ends_with(".md") && !filename.starts_with(".de.sopicki.cuelib") {
        return true;
    }

    false
}

fn process(entry: DirEntry, base_path: &str) -> IndexFile {
    lazy_static! {
        static ref TITLE_PATTERN: Regex = Regex::new(r"^#\s+(?P<title>.*)$").unwrap();
        static ref META_PATTERN: Regex =
            Regex::new(r"^[\*]\s+[\*][\*](?P<metaname>\w+)[\*][\*]:\s+(?P<metatext>.*)$").unwrap();
        static ref PHASE_PATTERN: Regex = Regex::new(r"^(I|II|III|IV|V|VI)\s*(\+.*)?$").unwrap();
        static ref AUDIO_FILE_PATTERN: Regex =
            Regex::new("^<meta\\s+name=\"x:audio-file\"\\s+content=\"(?P<filename>.*)\"\\s*>")
                .unwrap();
        static ref MAIL_PATTERN: Regex = Regex::new(r"\[(?P<name>.+)\]\(mailto.*\)").unwrap();
    }

    let filename = entry.path().to_str().unwrap().to_owned();
    let content = std::fs::read_to_string(entry.path()).unwrap();
    let file_path = entry.path().strip_prefix(base_path).expect("not a relative path")
        .to_str().expect("string conversion failed").to_string();
    let mut index_file = IndexFile {
        path: entry,
        content: "".to_owned(),
        meta: HashMap::new(),
        audio_file: "".to_owned(),
        file_path
    };
    let mut has_title = false;

    for line in content.lines() {
        if !has_title {
            if let Some(caps) = TITLE_PATTERN.captures(line) {
                index_file.set_meta("title", caps.name("title").unwrap().as_str());
                has_title = true;
            }
        }

        if let Some(caps) = META_PATTERN.captures(line) {
            let key = caps.name("metaname").unwrap().as_str();
            let mut text = caps.name("metatext").unwrap().as_str();

            if MAIL_PATTERN.is_match(text) {
                //info!("Mail detected. Extracting name.");
                let mail_caps = MAIL_PATTERN.captures(text).unwrap();
                text = mail_caps.name("name").unwrap().as_str();
                //info!("Name extracted: {:?}", text);
            }

            index_file.set_meta(&key.to_lowercase(), text);
        }

        if let Some(caps) = AUDIO_FILE_PATTERN.captures(line) {
            let filename = caps.name("filename").unwrap().as_str();
            index_file.audio_file = filename.to_string();
        }
    }
    index_file.set_content(&content);

    let default = "unphased".to_string();

    let phase = index_file
        .get_meta("phase".to_string())
        .unwrap_or(&default)
        .clone();

    match PHASE_PATTERN.captures(&phase) {
        Some(caps) => {
            let p = match caps.get(1) {
                Some(m) => m.as_str(),
                _ => "unphased",
            };

            let plusfigures = match caps.get(2) {
                Some(m) => m.as_str(),
                _ => "",
            };

            index_file.set_meta("phase", p);
            index_file.set_meta("plusfigures", plusfigures);
        }
        _ => {
            index_file.set_meta("phase", "unphased");
            index_file.set_meta("plusfigures", "");
        }
    }

    index_file.set_meta("_cuesheetpath", &filename);

    index_file
}

fn index(connection: &SqliteConnection, file: &IndexFile) {
    let u = Uuid::new_v4();
    let unphased = "unphased".to_string();
    let unknown = "unknown".to_string();
    let empty = "".to_string();

    let values = CuecardData {
        uuid: &u.to_hyphenated().to_string(),
        phase: file.get_meta("phase".to_string()).unwrap_or(&unphased),
        rhythm: file.get_meta("rhythm".to_string()).unwrap_or(&unknown),
        title: file.get_meta("title".to_string()).unwrap_or(&unknown),
        choreographer: file
            .get_meta("choreographer".to_string())
            .unwrap_or(&unknown),
        steplevel: file.get_meta("steplevel".to_string()).unwrap_or(&empty),
        difficulty: file.get_meta("difficulty".to_string()).unwrap_or(&empty),
        meta: &serde_json::to_string(&file.meta).unwrap_or_else(|_| "{}".to_string()),
        content: &file.content,
        karaoke_marks: "",
        music_file: &file.audio_file,
        file_path: &file.file_path,
    };
    values.create(connection).unwrap();

    let index_file = file.index_file().unwrap();

    std::fs::write(index_file, u.to_hyphenated().to_string()).unwrap();
}

fn update(connection: &SqliteConnection, file: &IndexFile, cuecard: &Cuecard) {
    let unphased = "unphased".to_string();
    let unknown = "unknown".to_string();
    let empty = "".to_string();

    let indexfile = file.index_file().unwrap();
    let fileuuid = std::fs::read_to_string(indexfile).unwrap();

    let values = CuecardData {
        uuid: &fileuuid,
        phase: file.get_meta("phase".to_string()).unwrap_or(&unphased),
        rhythm: file.get_meta("rhythm".to_string()).unwrap_or(&unknown),
        title: file.get_meta("title".to_string()).unwrap_or(&unknown),
        choreographer: file
            .get_meta("choreographer".to_string())
            .unwrap_or(&unknown),
        steplevel: file.get_meta("steplevel".to_string()).unwrap_or(&empty),
        difficulty: file.get_meta("difficulty".to_string()).unwrap_or(&empty),
        meta: &serde_json::to_string(&file.meta).unwrap_or_else(|_| "{}".to_string()),
        content: &file.content,
        karaoke_marks: "",
        music_file: &file.audio_file,
        file_path: &file.file_path,
    };

    values.update(cuecard, connection).unwrap();
    let indexfile = file.index_file().unwrap();
    let filetime = FileTime::from_system_time(SystemTime::now());
    set_file_mtime(indexfile, filetime).unwrap();
}

#[derive(PartialEq, Eq, Debug)]
enum IndexAction {
    Index,
    Update,
    NotModified,
}

fn should_index(connection: &SqliteConnection, file: &IndexFile) -> (IndexAction, Option<Cuecard>) {
    let indexfile = file.index_file().unwrap();

    if indexfile.exists() {
        debug!("Found existing index file {:?}", indexfile);
        let modified = file.path.path().metadata().unwrap().modified().unwrap();
        let imodified = indexfile.metadata().unwrap().modified().unwrap();
        let fileuuid = std::fs::read_to_string(indexfile).unwrap();
        if modified > imodified {
            debug!(
                "File {:?} has been modified since last index run. Will update.",
                file.path
            );
            return (IndexAction::Update, get_cuecard(connection, &fileuuid, file));
        } else {
            
            let result = get_cuecard(connection, &fileuuid, file);
            
            if result.is_none() {
                return (IndexAction::Update, result);
            }

            let cuecard = result.unwrap();
            if cuecard.uuid == fileuuid {
                debug!("File {:?} has not been modified!", file.path);
                return (IndexAction::NotModified, Some(cuecard));
            }

            let index_file = file.index_file().unwrap();
            info!(
                "Cuecard found by file_path. Updating index file with UUID {} from the database",
                &cuecard.uuid
            );
            std::fs::write(index_file, &cuecard.uuid).unwrap();

            return (IndexAction::Update, Some(cuecard));
        }
    }

    debug!("No index file found. Will index file {:?}.", file.path);
    (IndexAction::Index, None)
}

fn get_cuecard(connection: &SqliteConnection, fileuuid: &str, file: &IndexFile) -> Option<Cuecard> {
    use self::schema::cuecards::dsl::*;
    match cuecards
        .filter(uuid.eq(fileuuid.clone()))
        .first::<Cuecard>(connection)
        {
            Ok(cuecard) => Some(cuecard),
            Err(_) => {
                info!(
                    "UUID {} not found in database. Will retry searching by file_path.",
                    fileuuid
                );

                match cuecards
                    .filter(
                        file_path.eq(&file.file_path),
                    )
                    .first::<Cuecard>(connection) {
                        Ok(cuecard) => Some(cuecard),
                        Err(_) => None
                    }
            }
        }
}

fn get_index_files_list(basepath: &str, min_depth: usize) -> Vec<IndexFile> {
    let walkdir = WalkDir::new(basepath)
        .min_depth(min_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_allowed(&e.file_name().to_str().unwrap().to_lowercase()));

    let mut files: Vec<IndexFile> = vec![];

    for entry in walkdir {
        debug!("{}", entry.path().display());
        let indexfile = process(entry, basepath);
        files.push(indexfile);
    }

    files
}

pub fn run(config: &Config) {
    let files = get_index_files_list(&config.basepath, 2);

    let connection = establish_connection(&config.database_url);
    for file in files {
        match should_index(&connection, &file) {
            (IndexAction::Update, Some(cuecard)) => {
                info!(
                    "Reindexing file: {:?}",
                    file.path.path().file_name().unwrap()
                );
                update(&connection, &file, &cuecard);
            }
            (IndexAction::Update, None) => {
                error!("Index file found but no related cuecard in the database. Remove stale indexfile {:?} and reindex", file.index_file().unwrap());
            },
            (IndexAction::Index, Some(_)) => {
                error!(
                    "Can't index existing cuecard: {:?}",
                    file.path.path().file_name().unwrap()
                );
            }
            (IndexAction::Index, None) => {
                info!(
                    "Indexing new file: {:?}",
                    file.path.path().file_name().unwrap()
                );
                index(&connection, &file);
            }
            _ => {
                debug!(
                    "File not modified: {:?}",
                    file.path.path().file_name().unwrap()
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
}
