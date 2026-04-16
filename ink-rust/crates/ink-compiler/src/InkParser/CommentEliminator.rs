// Source: ink-c-sharp/compiler/InkParser/CommentEliminator.cs

#[derive(Clone, Debug, Default)]
pub struct CommentEliminator {
    input: String,
}

impl CommentEliminator {
    // C# signature: public CommentEliminator (string input)
    pub fn new(input: String) -> Self {
        Self { input }
    }

    // C# signature: public string Process()
    pub fn Process(&self) -> Option<String> {
        let input = normalize_newlines(&self.input);
        let chars: Vec<char> = input.chars().collect();
        let mut output = String::with_capacity(input.len());
        let mut index = 0;

        while index < chars.len() {
            let current = chars[index];
            let next = chars.get(index + 1).copied();

            if current == '/' && next == Some('/') {
                index += 2;
                while index < chars.len() && chars[index] != '\n' {
                    index += 1;
                }
                continue;
            }

            if current == '/' && next == Some('*') {
                index += 2;
                let mut newline_count = 0;

                while index < chars.len() {
                    if chars[index] == '*' && chars.get(index + 1) == Some(&'/') {
                        index += 2;
                        break;
                    }

                    if chars[index] == '\n' {
                        newline_count += 1;
                    }

                    index += 1;
                }

                output.extend(std::iter::repeat('\n').take(newline_count));
                continue;
            }

            output.push(current);
            index += 1;
        }

        Some(output)
    }
}

fn normalize_newlines(input: &str) -> String {
    input.replace("\r\n", "\n").replace('\r', "\n")
}

#[cfg(test)]
mod tests {
    use super::CommentEliminator;

    #[test]
    fn removes_line_comments_and_normalizes_newlines() {
        let processed = CommentEliminator::new("one // comment\r\ntwo\rthree".to_string())
            .Process()
            .unwrap();

        assert_eq!(processed, "one \ntwo\nthree");
    }

    #[test]
    fn removes_block_comments_but_preserves_line_count() {
        let processed = CommentEliminator::new("a/* x\n y\n z */b".to_string())
            .Process()
            .unwrap();

        assert_eq!(processed, "a\n\nb");
    }
}
