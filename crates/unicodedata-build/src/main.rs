use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use anyhow::{Result, bail};
use quick_xml::events::attributes::Attribute;
use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;

fn get_attrs<'a>(elem: &'a BytesStart<'a>) -> impl Iterator<Item = (String, String)> + 'a {
    elem.attributes().map(|a| {
        let Attribute { key, value } = a.unwrap();
        (
            unsafe { String::from_utf8_unchecked(key.0.to_vec()) },
            unsafe { String::from_utf8_unchecked(value.to_vec()) },
        )
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NameAlias {
    alias: String,
    type_: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RepertoireItem {
    tag: String,
    attrs: HashMap<String, String>,
    name_aliases: Vec<NameAlias>,
}

impl RepertoireItem {
    fn get_codepoint_range(&self) -> Result<(u32, u32)> {
        let start_cp_str = self
            .attrs
            .get("cp")
            .or_else(|| self.attrs.get("first-cp"))
            .ok_or_else(|| anyhow::anyhow!("No codepoint found"))?;
        let start_cp = u32::from_str_radix(start_cp_str, 16)?;

        let end_cp = if let Some(end_cp_str) = self.attrs.get("last_cp") {
            u32::from_str_radix(end_cp_str, 16)?
        } else {
            start_cp
        };

        Ok((start_cp, end_cp))
    }
}

impl PartialOrd for RepertoireItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let (self_cp, _) = self.get_codepoint_range().ok()?;
        let (other_cp, _) = other.get_codepoint_range().ok()?;
        Some(self_cp.cmp(&other_cp))
    }
}

impl Ord for RepertoireItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn process_repertoire(reader: &mut Reader<BufReader<File>>) -> Result<Vec<RepertoireItem>> {
    let mut items = Vec::new();

    let mut buf = Vec::new();
    let mut group_attrs = HashMap::<String, String>::new();
    let mut item: Option<RepertoireItem> = None;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) | Event::Empty(element) => match element.name().as_ref() {
                b"group" => {
                    group_attrs.extend(get_attrs(&element));
                }
                b"char" | b"noncharacter" | b"reserved" | b"surrogate" => {
                    if let Some(item) = item.take() {
                        items.push(item);
                    }
                    let mut attrs = group_attrs.clone();
                    attrs.extend(get_attrs(&element));
                    item = Some(RepertoireItem {
                        tag: String::from_utf8(element.name().as_ref().to_vec()).unwrap(),
                        attrs,
                        name_aliases: Vec::new(),
                    });
                }
                b"name-alias" => {
                    if let Some(ref mut item) = item {
                        let mut alias = String::new();
                        let mut type_ = String::new();
                        for (key, value) in get_attrs(&element) {
                            match key.as_str() {
                                "alias" => alias = value,
                                "type" => type_ = value,
                                _ => {
                                    bail!("Unexpected attribute '{}' in name-alias", key);
                                }
                            }
                        }
                        item.name_aliases.push(NameAlias { alias, type_ });
                    } else {
                        bail!("name-alias found outside of char/noncharacter/reserved/surrogate");
                    }
                }
                _ => {}
            },
            Event::End(element) => match element.name().as_ref() {
                b"repertoire" => break,
                b"group" => group_attrs.clear(),
                _ => {}
            },
            _ => {}
        }
    }

    if let Some(item) = item.take() {
        items.push(item);
    }

    items.sort();
    println!("Processed {} repertoire items", items.len());

    Ok(items)
}

fn main() -> Result<()> {
    let mut reader = Reader::from_file("data/unicodedata/ucd.nounihan.grouped.xml")?;
    let mut buf = Vec::new();

    let mut items: Option<Vec<_>> = None;

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf)? {
            Event::Start(element) => {
                if let b"ucd" = element.name().as_ref() {
                    let mut buf = Vec::new();
                    loop {
                        buf.clear();
                        match reader.read_event_into(&mut buf)? {
                            Event::Start(element) => match element.name().as_ref() {
                                b"description" => {}
                                b"repertoire" => {
                                    items = Some(process_repertoire(&mut reader)?);
                                }
                                b"blocks" => {}
                                _ => {}
                            },
                            Event::End(element) => {
                                if element.name().as_ref() == b"ucd" {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    let mut names_idx: HashMap<String, u32> = HashMap::new();
    let mut name_list: Vec<String> = Vec::new();
    let mut cp_to_name_idx: Vec<(u32, u32)> = Vec::new();
    let mut i: u32 = 0;

    for item in items.unwrap().iter() {
        let (start_cp, end_cp) = item.get_codepoint_range()?;
        for cp in start_cp..=end_cp {
            if start_cp != end_cp {
                continue;
            }
            if let Some(name) = item.attrs.get("na") {
                if !names_idx.contains_key(name) {
                    names_idx.insert(name.clone(), i);
                    name_list.push(name.clone());
                    i += 1;
                }
                let name_idx = *names_idx.get(name).unwrap();
                cp_to_name_idx.push((cp, name_idx));
            }
        }
    }

    let output_vec = bincode::encode_to_vec(&name_list, bincode::config::standard()).unwrap();
    std::fs::write("name_list", &output_vec)?;
    let output_vec = bincode::encode_to_vec(&cp_to_name_idx, bincode::config::standard()).unwrap();
    std::fs::write("cp_to_name_idx", &output_vec)?;

    println!("{:#?}", name_list);
    println!("{:#?}", cp_to_name_idx);

    Ok(())
}
