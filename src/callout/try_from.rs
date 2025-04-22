use super::callout_type::CalloutType;
use super::content::CalloutContent;
use super::error::CalloutError;
use crate::Callout;
use regex::Regex;
use std::sync::LazyLock;

static RE_HEADER: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:> )?> \[!(.+?)\][+-]? ?([\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337F\u3131-\u3132\u3132-\u3134\u3134-\u3137\u3137-\u3139\u3139-\u3141\u3141-\u3142\u3142-\u3145\u3145-\u3146\u3146-\u3147\u3147-\u3148\u3148-\u314A\u314A-\u314B\u314B-\u314C\u314C-\u314D\u314D-\u314E\u314E-\u3163A-Za-z0-9.,?!'"()\[\]{}\-+|*_/\\]+(?: [\u4E00-\u9FFF\u3400-\u4DBF\uF900-\uFAFF\u3040-\u309F\u30A0-\u30FF\u31F0-\u31FF\uAC00-\uD7AF\u2E80-\u2FD5\uFF5F-\uFF9F\u3000-\u303F\u31F0-\u31FF\u3220-\u3243\u3280-\u337F\u3131-\u3132\u3132-\u3134\u3134-\u3137\u3137-\u3139\u3139-\u3141\u3141-\u3142\u3142-\u3145\u3145-\u3146\u3146-\u3147\u3147-\u3148\u3148-\u314A\u314A-\u314B\u314B-\u314C\u314C-\u314D\u314D-\u314E\u314E-\u3163A-Za-z0-9.,?!'"()\[\]{}\-+|*_/\\]+)*)?(  [A-Za-zÀ-ÖØ-öø-ÿĀ-ſƀ-ɏ ]*)? *(.*?)?$"#).unwrap()
});

impl TryFrom<Vec<&str>> for Callout {
    type Error = CalloutError;
    fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
        let content_length = (value.len() + 1).max(3);
        let mut value_iter = value.iter();

        let header_line = match value_iter.next() {
            Some(line) => line,
            None => panic!("{:?}", CalloutError::EmptyString),
        };

        let caps = RE_HEADER
            .captures(header_line)
            .ok_or(CalloutError::FailedToParseHeader)?;

        // .expect(
        // );

        let callout_type: CalloutType =
            caps[1].try_into().map_err(|_| CalloutError::UnknownType)?;
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

        let mut content: Vec<CalloutContent> = Vec::with_capacity(content_length);
        if !emoji.is_empty() {
            content.push(CalloutContent::Text(emoji));
        }
        if !transliteration.is_empty() {
            content.push(CalloutContent::Text(transliteration));
        }

        let mut markdown_id: &str = "";
        let mut sub_callouts: Vec<Callout> = Vec::with_capacity(content_length);
        let mut prev: &str = "";
        let mut line: &str;
        let mut next: &str = "";

        'split_loop: loop {
            if !prev.is_empty() {
                if prev.starts_with("> ^") {
                    markdown_id = prev.strip_prefix("> ^").unwrap_or("");
                    break 'split_loop;
                }
                content.push(CalloutContent::Text(
                    prev.to_string()
                        .strip_prefix(">")
                        .unwrap_or(prev)
                        .trim()
                        .to_string(),
                ));
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
                    sub_callout_vector
                        .push(next_line.strip_prefix(">").unwrap_or(next_line).trim());
                }
                let sub_callout: Callout = sub_callout_vector.try_into()?;
                if !(sub_callout.content.is_empty()
                    || (sub_callout.content.len() == 1
                        && match &sub_callout.content[0] {
                            CalloutContent::Text(text) => text.is_empty(),
                            _ => false,
                        }))
                {
                    sub_callouts.push(sub_callout);
                    content.push(CalloutContent::SubCalloutIndex(sub_callouts.len() - 1));
                }
            } else {
                line = line.strip_prefix(">").unwrap_or(line).trim();
                if line.starts_with('^') {
                    markdown_id = line.strip_prefix("^").unwrap_or("").trim();
                    break 'split_loop;
                }
                content.push(CalloutContent::Text(line.trim().to_string()));
            }
        }
        // Trim leading empty lines
        let mut to_slice = 0;
        let mut content_iter = content.iter();
        while let Some(CalloutContent::Text(first)) = content_iter.next() {
            if first.is_empty() || first.eq("---") {
                to_slice += 1;
            } else {
                break;
            }
        }

        content = content[to_slice..].to_vec();

        // Trim trailing empty lines
        while let Some(CalloutContent::Text(last)) = content.last() {
            if last.is_empty() {
                content.pop();
            } else {
                break;
            }
        }

        Ok(Callout::new(
            "".into(),
            markdown_id.into(),
            callout_type,
            header,
            content,
            sub_callouts,
        ))
    }
}
