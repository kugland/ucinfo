use lazy_static::lazy_static;

const HTMLENTITIES_BIN: &[u8] = include_bytes!(env!("HTMLENTITIES_BIN_FILE"));

lazy_static! {
    static ref HTMLENTITIES: Vec<(u32, Vec<String>)> =
        bincode::decode_from_slice(HTMLENTITIES_BIN, bincode::config::standard())
            .unwrap()
            .0;
}

pub fn get_entities(cp: u32) -> Vec<String> {
    let mut entities: Vec<String> = HTMLENTITIES
        .binary_search_by_key(&cp, |&(c, _)| c)
        .ok()
        .map(|idx| {
            HTMLENTITIES[idx].1
                .clone()
                .into_iter()
                .map(|s| format!("&{};", s))
                .collect()
        })
        .unwrap_or_default();
    entities.push(format!("&#{};", cp));
    entities.push(format!("&#x{:X};", cp));
    entities
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_htmlentities_loaded() {
        assert!(!HTMLENTITIES.is_empty());
    }

    #[test]
    fn test_get_entities() {
        let entities = get_entities(0x0020);
        assert_eq!(entities, vec!["&#32;", "&#x20;"]);

        let entities = get_entities(0x0022);
        assert_eq!(entities, vec!["&quot;", "&#34;", "&#x22;"]);

        let entities = get_entities(0x0026);
        assert_eq!(entities, vec!["&amp;", "&#38;", "&#x26;"]);

        let entities = get_entities(0x00a0);
        assert_eq!(
            entities,
            vec!["&nbsp;", "&NonBreakingSpace;", "&#160;", "&#xA0;"]
        );

        let entities = get_entities(0x00a9);
        assert_eq!(entities, vec!["&copy;", "&#169;", "&#xA9;"]);

        let entities = get_entities(0x00fe);
        assert_eq!(entities, vec!["&thorn;", "&#254;", "&#xFE;"]);

        let entities = get_entities(0x00de);
        assert_eq!(entities, vec!["&THORN;", "&#222;", "&#xDE;"]);

        let entities = get_entities(0x20ac);
        assert_eq!(entities, vec!["&euro;", "&#8364;", "&#x20AC;"]);

        let entities = get_entities(0x226c);
        assert_eq!(
            entities,
            vec!["&between;", "&twixt;", "&#8812;", "&#x226C;"]
        );
    }
}
