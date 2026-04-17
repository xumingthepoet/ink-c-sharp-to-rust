// Source: ink-c-sharp/compiler/InkParser/InkParser_Tags.cs

use crate::InkParser::InkParser::{CustomFlags, InkParser};
use crate::ParsedHierarchy::ContentList::{ContentList, ContentListItem};
use crate::ParsedHierarchy::Tag::Tag;

impl InkParser {
    // C# signature: protected Parsed.Object StartTag ()
    pub fn StartTag(&mut self) -> Option<ContentListItem> {
        self.Whitespace();

        if self.ParseString("#".to_string()).is_none() {
            return None;
        }

        if self.get_parsingStringExpression() {
            self.Error(
                "Tags aren't allowed inside of strings. Please use \\# if you want a hash symbol."
                    .to_string(),
            );
        }

        let result = if self.get_tagActive() {
            let mut content_list = ContentList::new_overload_2();
            content_list.AddContent(ContentListItem::Tag(Tag::new_with_flags(false, false)));
            content_list.AddContent(ContentListItem::Tag(Tag::new_with_flags(true, false)));
            ContentListItem::ContentList(Box::new(content_list))
        } else {
            ContentListItem::Tag(Tag::new_with_flags(true, false))
        };

        self.set_flag(CustomFlags::TagActive, true);

        self.Whitespace();

        Some(result)
    }

    // C# signature: protected void EndTagIfNecessary(List<Parsed.Object> outputContentList)
    pub fn EndTagIfNecessary(&mut self, outputContentList: &mut Vec<ContentListItem>) {
        if self.get_tagActive() {
            outputContentList.push(ContentListItem::Tag(Tag::new_with_flags(false, false)));
            self.set_flag(CustomFlags::TagActive, false);
        }
    }

    // C# signature: protected void EndTagIfNecessary(Parsed.ContentList outputContentList)
    pub fn EndTagIfNecessary_overload_2(&mut self, outputContentList: &mut ContentList) {
        if self.get_tagActive() {
            outputContentList.AddContent(ContentListItem::Tag(Tag::new_with_flags(false, false)));
            self.set_flag(CustomFlags::TagActive, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InkParser;
    use crate::ParsedHierarchy::ContentList::ContentListItem;
    use crate::ParsedHierarchy::Tag::Tag;

    #[test]
    fn parses_tag_starts_and_closes_active_tag_runs() {
        let mut parser = InkParser::new("#".to_string(), None, None, None);
        let tag = parser.StartTag().expect("expected tag");
        assert!(matches!(tag, ContentListItem::Tag(_)));
        assert!(parser.get_tagActive());

        let mut list = vec![ContentListItem::Tag(Tag::new_with_flags(true, false))];
        parser.EndTagIfNecessary(&mut list);
        assert_eq!(list.len(), 2);
        assert!(!parser.get_tagActive());
    }

    #[test]
    fn end_tag_updates_content_list_overload() {
        let mut parser = InkParser::new("#".to_string(), None, None, None);
        let _ = parser.StartTag();

        let mut content_list = crate::ParsedHierarchy::ContentList::ContentList::new_overload_2();
        parser.EndTagIfNecessary_overload_2(&mut content_list);
        assert_eq!(content_list.get_content().len(), 1);
    }
}
