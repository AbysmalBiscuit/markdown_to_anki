impl TryFrom<Vec<&str>> for Callout {
    type Error = CalloutError;
    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let content_length = (value.len() + 1).max(3);
        let mut value_iter = value.iter();

        let header_line = match value_iter.next() {
            Some(line) => line,
            None => panic!("{:?}", CalloutError::EmptyString),
        };

        let caps = RE_HEADER.captures(header_line).expect(
            "first line should be formatted as a callout '> [!TYPE] TEXT TRANSLITERATION EMOJI'",
        );

        let callout_type: CalloutType = caps[1].try_into()?;
        let header: String = caps
            .get(2)
            .map_or(String::new(), |m| m.as_str().to_string());
        let transliteration = caps
            .get(3)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or("".to_string());
        let emoji = caps
            .get(4)
            .map(|m| m.as_str().trim().to_string())
            .unwrap_or("".to_string());

        let mut content: Vec<String> = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(emoji);
        }
        if !transliteration.is_empty() {
            content.push(transliteration);
        }

        let mut id: &str = "";
        let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
        let mut prev: &str = "";
        let mut line: &str;
        let mut next: &str = "";

        // TODO: rewrite this to be a loop around indeces instead of iter
        'split_loop: loop {
            if !prev.is_empty() {
                if prev.starts_with("> ^") {
                    break 'split_loop;
                }
                content.push(prev.to_string());
                prev = "";
            }
            if !next.is_empty() {
                line = next;
                next = "";
            } else {
                line = *value_iter.next().unwrap_or(&"");
                next = *value_iter.next().unwrap_or(&"");
            }

            if line.is_empty() {
                break 'split_loop;
            }

            if line.starts_with("> > [!") {
                let mut sub_callout_vector: Vec<&str> = Vec::with_capacity(content_length);
                sub_callout_vector.push(line);
                if next.starts_with("> >") {
                    sub_callout_vector.push(next.strip_prefix("> ").unwrap_or(next));
                    next = "";
                }
                'sub_callout: loop {
                    let next_line = value_iter.next().unwrap_or(&"");
                    if next_line.is_empty() || next_line.trim().eq(">") {
                        break 'sub_callout;
                    }
                    if !next_line.starts_with("> >") {
                        prev = next_line;
                        break 'sub_callout;
                    }
                    sub_callout_vector.push(next_line);
                }
                sub_callouts.push(sub_callout_vector.try_into()?);
            } else {
                line = line.strip_prefix("> ").unwrap_or("");
                if line.starts_with('^') {
                    id = line.strip_prefix("^").unwrap_or("");
                    break 'split_loop;
                }

                content.push(line.trim().to_string());
            }
        }

        while let Some(last) = content.last() {
            if last.is_empty() {
                content.pop();
            } else {
                break;
            };
        }

        Ok(Callout::new(
            id.into(),
            callout_type,
            header,
            content,
            sub_callouts,
        ))
    }
}
