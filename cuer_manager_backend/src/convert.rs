use log::{info, error};
use std::io::*;
use xml::reader::{EventReader, XmlEvent};

fn matches_part_title(s: &str) -> bool {
    let t = s.to_lowercase();

    if t.starts_with("intro")
        || t.starts_with("part")
        || t.starts_with("int") && !t.starts_with("into")
        || t.starts_with("bridge")
        || t.starts_with("bdg")
        || t.starts_with("end")
        || t.starts_with("a ")
        || t.starts_with("b ")
        || t.starts_with("c ")
        || t.starts_with("d ")
        || t.starts_with("e ")
        || t.starts_with("f ")
    {
        return true;
    }
    false
}

#[derive(Debug, Default)]
struct State {
    pub has_title: bool,
    pub relevant_element: bool,
    pub ignore_content: bool,
    pub new_line: bool,
    pub new_part: bool,
    pub sequence: bool
}

pub fn convert_to_markdown<R, W>(input: &mut R, writer: &mut W) -> Result<()>
where
    R: Read + Seek,
    W: Write,
{
    let mut archive = match zip::read::ZipArchive::new(input) {
        Ok(archive) => archive,
        Err(err) => {
            error!("Unable to read archive: {:?}", err);
            return Err(Error::new(ErrorKind::InvalidData, err));
        }
    };

    let mut state = State::default();

    if let Ok(result) = archive.by_name("content.xml") {
        let reader = BufReader::new(result);

        let parser = EventReader::new(reader);
        let capacity = 5;

        let mut text_parts: Vec<String> = Vec::with_capacity(capacity);

        for e in parser {
            match e {
                Ok(XmlEvent::StartDocument { encoding, ..}) => {
                    info!("File encoding {} for content", encoding);
                }
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.as_ref() {
                        "s" => {
                            state.relevant_element = true;
                        }
                        "bookmark-start" => {
                            state.relevant_element = true;
                        }
                        "tab" => {
                            text_parts.push(" ".to_owned());
                            write!(writer, " ").unwrap();
                            info!(" ");
                        }
                        "a" => {
                            state.ignore_content = true;
                        }
                        "p" => {
                            state.relevant_element = true;
                            state.new_line = true;
                        }
                        "h" => {
                            state.relevant_element = true;
                            state.new_part = true;
                        }
                        "span" => {
                            state.relevant_element = true;
                        }
                        _ => (),
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if state.relevant_element && !state.ignore_content {
                        text_parts.push(s);
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    match name.local_name.as_ref() {
                        "a" => {
                            state.ignore_content = false;
                            text_parts = Vec::with_capacity(capacity);
                        }
                        "p" => {
                            write_text_parts(&text_parts, &mut *writer, & mut state);
                            state.relevant_element = false;
                            writeln!(writer).unwrap();
                            state.sequence = false;
                            text_parts = Vec::with_capacity(capacity);
                        }
                        "h" => {
                            write_text_parts(&text_parts, &mut *writer, & mut state);
                            state.relevant_element = false;
                            writeln!(writer).unwrap();
                            state.sequence = false;
                            text_parts = Vec::with_capacity(capacity);
                        }
                        _ => (),
                    }
                }
                Err(e) => {
                    error!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    };

    Ok(())
}

fn write_text_parts<W>(text_parts: & Vec<String>, writer: &mut W, state: & mut State) 
    where W: Write {
    let s = text_parts.iter().map(|s| s.chars()).flatten().collect::<String>();

    if s.to_lowercase().contains("seq") {
        state.sequence = true;
    }

    if !state.has_title {
        writeln!(writer, "# {}", s).unwrap();
        info!("# {}", s);
        state.has_title = true;
    } else if state.new_part {
        writeln!(writer).unwrap();
        write!(writer, "# {}", s).unwrap();
        info!("# {}", s);
        state.new_part = false;
    } else if matches_part_title(&s) && !state.sequence {
        writeln!(writer).unwrap();
        write!(writer, "# {}", s).unwrap();
        info!("#: {}", s);
    } else if s.contains(';') {
        if state.new_line {
            writeln!(writer).unwrap();
            write!(writer, "> ").unwrap();
            
            state.new_line = false;
        }
        let text = s.replace("\n", "").replace("\r", "");
        write!(writer, "{}", text).unwrap();
        info!("> {}", text);
    } else {
        if state.new_line {
            writeln!(writer).unwrap();
        }
        let text = s.replace("\n", "").replace("\r", "");
        write!(writer, "{}", text).unwrap();
        info!("{}", text);
    }
}
