use log::error;
use std::io::*;
use xml::reader::{EventReader, XmlEvent};

fn matches_part_title(s: &str) -> bool {
    let t = s.to_lowercase();

    if t.starts_with("intro")
        || t.starts_with("part")
        || t.starts_with("int")
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

    let mut has_title = false;
    let mut relevant_element = false;
    let mut new_line = false;
    let mut new_part = false;
    let mut sequence = false;

    if let Ok(result) = archive.by_name("content.xml") {
        let reader = BufReader::new(result);

        let parser = EventReader::new(reader);

        for e in parser {
            match e {
                /*Ok(XmlEvent::StartDocument { encoding, ..}) => {
                    println!("File encoding {}", encoding);
                }*/
                Ok(XmlEvent::StartElement { name, .. }) => {
                    match name.local_name.as_ref() {
                        "s" => {
                            // depth += 1;
                            relevant_element = true;
                        }
                        "p" => {
                            // println!("{}", name.local_name);
                            // depth += 1;
                            relevant_element = true;
                            new_line = true;
                        }
                        "h" => {
                            // println!("{}",  name.local_name);
                            relevant_element = true;
                            new_part = true;
                        }
                        "span" => {
                            // println!("{}{}", indent(depth), name.local_name);
                            // depth += 1;
                            relevant_element = true;
                        }
                        _ => (),
                    }
                }
                Ok(XmlEvent::Characters(s)) => {
                    if relevant_element {
                        if s.to_lowercase().starts_with("seq") {
                            sequence = true;
                        }

                        if !has_title {
                            // println!("# {}", s);
                            writeln!(writer, "# {}", s).unwrap();
                            has_title = true;
                        } else if new_part {
                            // println!("");
                            // print!("# {}", s);
                            writeln!(writer).unwrap();
                            write!(writer, "# {}", s).unwrap();
                            new_part = false;
                        } else if matches_part_title(&s) && !sequence {
                            // println!("");
                            // print!("# {}", s);
                            writeln!(writer).unwrap();
                            write!(writer, "# {}", s).unwrap();
                        } else if s.contains(';') {
                            if new_line {
                                // println!("");
                                // print!("> ");
                                writeln!(writer).unwrap();
                                write!(writer, "> ").unwrap();
                                new_line = false;
                            }
                            let text = s.replace("\n", "").replace("\r", "");
                            // print!("{}", text);
                            write!(writer, "{}", text).unwrap();
                        } else {
                            if new_line {
                                // println!("");
                                writeln!(writer).unwrap();
                            }
                            let text = s.replace("\n", "").replace("\r", "");
                            // print!("{}", text);
                            write!(writer, "{}", text).unwrap();
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name }) => {
                    match name.local_name.as_ref() {
                        "p" => {
                            relevant_element = false;
                            // println!("");
                            writeln!(writer).unwrap();
                            sequence = false;
                        }
                        "h" => {
                            relevant_element = false;
                            // println!("");
                            writeln!(writer).unwrap();
                            sequence = false;
                        }
                        _ => (),
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    };

    Ok(())
}
