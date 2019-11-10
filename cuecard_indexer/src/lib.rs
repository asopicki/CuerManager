extern crate diesel;
extern crate regex;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate walkdir;
extern crate once_cell;
#[macro_use]
extern crate log;
extern crate cuer_database;
extern crate filetime;
extern crate uuid as uuidcrate;
extern crate chrono;

use self::cuer_database::*;
use self::diesel::prelude::*;
use self::models::*;
use self::walkdir::{DirEntry, WalkDir};
use filetime::{set_file_mtime, FileTime};
use regex::Regex;
use uuidcrate::Uuid;
use once_cell::unsync::Lazy;
use chrono::prelude::*;

use std::boxed::Box;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::vec::Vec;
use std::fmt;
use std::str::FromStr;

pub struct Config {
    pub basepath: String,
    pub database_url: String,
}

struct IndexFileData {
    path: DirEntry,
    content: String,
    meta: Box<HashMap<MetaDataType, String>>,
    file_path: String
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Serialize)]
#[serde(untagged)]
enum MetaDataType {
    Title,
    Choreographer,
    Phase,
    Difficulty,
    Rhythm,
    Plusfigures,
    Steplevel,
    Music,
    MusicFile,
    Extra(String)
}

impl FromStr for MetaDataType {
    type Err = ();

    fn from_str(s: &str) -> Result<MetaDataType, ()> {
        match s {
            "title" => Ok(MetaDataType::Title),
            "choreographer" => Ok(MetaDataType::Choreographer),
            "phase" => Ok(MetaDataType::Phase),
            "difficulty" => Ok(MetaDataType::Difficulty),
            "rhythm" => Ok(MetaDataType::Rhythm),
            "plusfigures" => Ok(MetaDataType::Plusfigures),
            "steplevel" => Ok(MetaDataType::Steplevel),
            "music" => Ok(MetaDataType::Music),
            "music_file" => Ok(MetaDataType::MusicFile),
            s => Ok(MetaDataType::Extra(s.to_owned())),
        }
    }
}

impl fmt::Display for MetaDataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &MetaDataType::Title => write!(f, "title"),
            &MetaDataType::Choreographer => write!(f, "choreographer"),
            &MetaDataType::Phase => write!(f, "phase"),
            &MetaDataType::Difficulty => write!(f, "difficulty"),
            &MetaDataType::Rhythm => write!(f, "rhythm"),
            &MetaDataType::Plusfigures => write!(f, "plusfigures"),
            &MetaDataType::Steplevel => write!(f, "steplevel"),
            &MetaDataType::Music => write!(f, "music"),
            &MetaDataType::MusicFile => write!(f, "music_file"),
            &MetaDataType::Extra(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaData {
    choreographer: String,
    phase: String,
    difficulty: Option<String>,
    rhythm: String,
    plusfigures: Option<String>,
    steplevel: Option<String>,
    music: Option<String>,
    music_file: Option<String>
}

impl IndexFileData {
    fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }

    fn metadata(&mut self) -> &mut HashMap<MetaDataType, String> {
        self.meta.as_mut()
    }

    fn get_meta(&self, key: MetaDataType) -> Option<&String> {
        self.meta.as_ref().get(&key)
    }

    fn index_file(&self) -> Option<PathBuf> {
        let mut filename = ".de.sopicki.cuelib.".to_string();
        filename.push_str(self.path.file_name().to_str().unwrap());
        filename = filename.to_string();
        let path = Path::new(&filename).to_owned();
        let parent = self.path.path().parent().unwrap();
        Some(parent.join(&path))
    }

    fn metadata_file(&self) -> PathBuf {
        self.path.path().with_extension("meta.json")
    }
}

fn is_allowed(filename: &str) -> bool {
    if filename.ends_with(".md") && !filename.starts_with(".de.sopicki.cuelib") {
        return true;
    }

    false
}

fn process(entry: DirEntry, base_path: &str) -> IndexFileData {
    let title_pattern = Lazy::new(|| Regex::new(r"^#\s+(?P<title>.*)$").unwrap());
    let meta_pattern = Lazy::new(|| Regex::new(r"^[\*]\s+[\*][\*](?P<metaname>\w+)[\*][\*]:\s+(?P<metatext>.*)$").unwrap());
    let phase_pattern = Lazy::new(|| Regex::new(r"^(I|II|III|IV|V|VI)\s*(\+.*)?$").unwrap());

    let mail_pattern = Lazy::new(|| Regex::new(r"\[(?P<name>.+)\]\(mailto.*\)").unwrap());

    let content = std::fs::read_to_string(entry.path()).unwrap();
    let file_path = entry.path().strip_prefix(base_path).expect("not a relative path")
        .to_str().expect("string conversion failed").to_string();
    let mut index_file = IndexFileData {
        path: entry,
        content: "".to_owned(),
        meta: Box::new(HashMap::new()),
        file_path
    };

    {
        let meta_data = index_file.metadata();
        let mut has_title = false;

        for line in content.lines() {
            if !has_title {
                if let Some(caps) = title_pattern.captures(line) {
                    meta_data.insert(MetaDataType::Title, caps.name("title").unwrap().as_str().to_string());
                    has_title = true;
                }
            }

            if let Some(caps) = meta_pattern.captures(line) {
                let name = caps.name("metaname").unwrap().as_str();
                let name = name.to_lowercase();
                let key = name.as_str();
                let mut text = caps.name("metatext").unwrap().as_str();

                if mail_pattern.is_match(text) {
                    debug!("Mail detected. Extracting name.");
                    let mail_caps = mail_pattern.captures(text).unwrap();
                    text = mail_caps.name("name").unwrap().as_str();
                    debug!("Name extracted: {:?}", text);
                }

                meta_data.insert(MetaDataType::from_str(key).unwrap(), text.to_string());
            }

        }

        let default = "unphased".to_string();

        let phase = meta_data.get(&MetaDataType::Phase)
            .unwrap_or(&default)
            .clone();

        match phase_pattern.captures(&phase) {
            Some(caps) => {
                let p = match caps.get(1) {
                    Some(m) => m.as_str(),
                    _ => "unphased",
                };

                let plusfigures = match caps.get(2) {
                    Some(m) => m.as_str(),
                    _ => "",
                };

                meta_data.insert(MetaDataType::Phase, p.to_string());
                meta_data.insert(MetaDataType::Plusfigures, plusfigures.to_string());
            }
            _ => {
                meta_data.insert(MetaDataType::Phase, "unphased".to_owned());
                meta_data.insert(MetaDataType::Plusfigures, "".to_owned());
            }
        }
    }
    index_file.set_content(&content);

    if index_file.metadata_file().exists() {
        process_metadata_file(&index_file.metadata_file(), index_file.metadata());
    } else {
        write_metadata_file(&index_file);
    }

    index_file
}

fn write_metadata_file(file: &IndexFileData) {
    let unphased = "unphased".to_string();
    let unknown = "unknown".to_string();
    let empty = "".to_string();

    let choreographer = file.get_meta(MetaDataType::Choreographer).unwrap_or(&unknown);
    let phase = file.get_meta(MetaDataType::Phase).unwrap_or(&unphased);
    let difficulty = file.get_meta(MetaDataType::Difficulty).unwrap_or(&empty);
    let rhythm = file.get_meta(MetaDataType::Rhythm).unwrap_or(&unknown);
    let plusfigures = file.get_meta(MetaDataType::Plusfigures).unwrap_or(&empty);
    let steplevel = file.get_meta(MetaDataType::Steplevel).unwrap_or(&empty);
    let music = file.get_meta(MetaDataType::Music).unwrap_or(&empty);
    let music_file = file.get_meta(MetaDataType::MusicFile).unwrap_or(&empty);

    let metadata = MetaData {
        choreographer: choreographer.to_string(),
        phase: phase.to_string(),
        difficulty: Some(difficulty.to_string()),
        rhythm: rhythm.to_string(),
        plusfigures: Some(plusfigures.to_string()),
        steplevel: Some(steplevel.to_string()),
        music: Some(music.to_string()),
        music_file: Some(music_file.to_string())
    };

    std::fs::write(file.metadata_file(), serde_json::to_string_pretty(&metadata).unwrap()).unwrap();
}

fn process_metadata_file(filepath: &PathBuf, data: &mut HashMap<MetaDataType, String>)  {
    if let Ok(metadata) = serde_json::from_str::<MetaData>(&std::fs::read_to_string(filepath).unwrap()) {

        data.insert(MetaDataType::Choreographer, metadata.choreographer);
        data.insert(MetaDataType::Phase, metadata.phase);
        data.insert(MetaDataType::Difficulty, metadata.difficulty.unwrap_or_default());
        data.insert(MetaDataType::Rhythm, metadata.rhythm);
        data.insert(MetaDataType::Plusfigures, metadata.plusfigures.unwrap_or_default());
        data.insert(MetaDataType::Steplevel, metadata.steplevel.unwrap_or_default());
        data.insert(MetaDataType::Music, metadata.music.unwrap_or_default());
        data.insert(MetaDataType::MusicFile, metadata.music_file.unwrap_or_default());
    }
}

fn index(connection: &SqliteConnection, file: &IndexFileData) {
    let u = Uuid::new_v4();
    let unphased = "unphased".to_string();
    let unknown = "unknown".to_string();
    let empty = "".to_string();

    let keys = file.meta.keys().map(|key| key.to_string()).collect::<Vec<String>>();
    let values = file.meta.values().map(|value| value.to_string()).collect::<Vec<String>>();

    let data = keys.iter().zip(values.iter()).collect::<HashMap<&String, &String>>();
    let metadata = match serde_json::to_string(&data) {
        Ok(result) => result,
        Err(err) => {
            error!("Serializing metadata failed with error: {:?}", err);
            String::from("{}")
        }
    };

    let time = Utc::now();

    let values = CuecardData {
        uuid: &u.to_hyphenated().to_string(),
        phase: file.get_meta(MetaDataType::Phase).unwrap_or(&unphased),
        rhythm: file.get_meta(MetaDataType::Rhythm).unwrap_or(&unknown),
        title: file.get_meta(MetaDataType::Title).unwrap_or(&unknown),
        choreographer: file
            .get_meta(MetaDataType::Choreographer)
            .unwrap_or(&unknown),
        steplevel: file.get_meta(MetaDataType::Steplevel).unwrap_or(&empty),
        difficulty: file.get_meta(MetaDataType::Difficulty).unwrap_or(&empty),
        meta: &metadata,
        content: &file.content,
        karaoke_marks: "",
        music_file: &file.get_meta(MetaDataType::MusicFile).unwrap_or(&empty),
        file_path: &file.file_path,
        date_created: &time.format("%FT%T%.3fZ").to_string(),
        date_modified: &time.format("%FT%T%.3fZ").to_string()
    };
    values.create(connection).unwrap();

    let index_file = file.index_file().unwrap();

    std::fs::write(index_file, u.to_hyphenated().to_string()).unwrap();
}

fn update(connection: &SqliteConnection, file: &IndexFileData, cuecard: &Cuecard) {
    let unphased = "unphased".to_string();
    let unknown = "unknown".to_string();
    let empty = "".to_string();

    let indexfile = file.index_file().unwrap();
    let fileuuid = std::fs::read_to_string(indexfile).unwrap();

    let keys = file.meta.keys().map(|key| key.to_string()).collect::<Vec<String>>();
    let values = file.meta.values().map(|value| value.to_string()).collect::<Vec<String>>();

    let data = keys.iter().zip(values.iter()).collect::<HashMap<&String, &String>>();
    let metadata = match serde_json::to_string(&data) {
        Ok(result) => result,
        Err(err) => {
            error!("Serializing metadata failed with error: {:?}", err);
            String::from("{}")
        }
    };

    let time = Utc::now();

    let values = CuecardData {
        uuid: &fileuuid,
        phase: file.get_meta(MetaDataType::Phase).unwrap_or(&unphased),
        rhythm: file.get_meta(MetaDataType::Rhythm).unwrap_or(&unknown),
        title: file.get_meta(MetaDataType::Title).unwrap_or(&unknown),
        choreographer: file
            .get_meta(MetaDataType::Choreographer)
            .unwrap_or(&unknown),
        steplevel: file.get_meta(MetaDataType::Steplevel).unwrap_or(&empty),
        difficulty: file.get_meta(MetaDataType::Difficulty).unwrap_or(&empty),
        meta: &metadata,
        content: &file.content,
        karaoke_marks: "",
        music_file: &file.get_meta(MetaDataType::MusicFile).unwrap_or(&empty),
        file_path: &file.file_path,
        date_created: &cuecard.date_created,
        date_modified: &time.format("%FT%T%.3fZ").to_string()
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

fn should_index(connection: &SqliteConnection, file: &IndexFileData) -> (IndexAction, Option<Cuecard>) {
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

fn get_cuecard(connection: &SqliteConnection, fileuuid: &str, file: &IndexFileData) -> Option<Cuecard> {
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

fn get_index_files_list(basepath: &str, min_depth: usize) -> Vec<IndexFileData> {
    let walkdir = WalkDir::new(basepath)
        .min_depth(min_depth)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| is_allowed(&e.file_name().to_str().unwrap().to_lowercase()));

    let mut files: Vec<IndexFileData> = vec![];

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
