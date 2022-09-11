#![feature(string_remove_matches, iter_intersperse)]

#[macro_use]
extern crate lazy_static;

mod regex {
    use regex::Regex;

    const STR_TIMESTAMP_PATTERN: &str =
        r"^(\d{2}):(\d{2}):(\d{2}),(\d{3})\s-->\s(\d{2}):(\d{2}):(\d{2}),(\d{3})$";
    const STR_LINE_NUM_PATTERN: &str = r"^\d+$";

    lazy_static! {
        pub static ref SRT_TIMESTAMP_REGEX: Regex = Regex::new(STR_TIMESTAMP_PATTERN).unwrap();
        pub static ref SRT_LINE_NUM_REGEX: Regex = Regex::new(STR_LINE_NUM_PATTERN).unwrap();
    }

    pub const PERSIAN_NUMBERS: [(char, &str); 10] = [
        ('0', "۰"),
        ('1', "۱"),
        ('2', "۲"),
        ('3', "۳"),
        ('4', "۴"),
        ('5', "۵"),
        ('6', "۶"),
        ('7', "۷"),
        ('8', "۸"),
        ('9', "۹"),
    ];
}

pub struct Subtitle {
    contents: String,
}

impl Subtitle {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    pub fn fix(mut self) -> String {
        self.remove_italics();
        self.replace_arabic_chars();
        self.replace_question_mark();
        self.remove_rtl_char();
        self.fix_others();
        self.contents
    }

    fn remove_italics(&mut self) {
        self.contents.remove_matches("<i>");
        self.contents.remove_matches("</i>");
    }

    fn replace_arabic_chars(&mut self) {
        self.contents = self.contents.replace('ي', "ی");
        self.contents = self.contents.replace('ك', "ک");
    }

    fn replace_question_mark(&mut self) {
        self.contents = self.contents.replace('?', "؟");
    }

    fn remove_rtl_char(&mut self) {
        self.contents.remove_matches('\u{202b}');
    }

    fn fix_others(&mut self) {
        let fix_line = |line: &str| {
            match line {
                l if regex::SRT_TIMESTAMP_REGEX.is_match(l) => l.into(),
                l if l.trim().is_empty() => l.into(),
                l if regex::SRT_LINE_NUM_REGEX.is_match(l) => l.into(),

                // This is subtitle
                l => {
                    //TODO: remove [\.!?]* groups in line

                    // Replace digits with Persian digits
                    let mut l = l.to_string();
                    for (n, p) in regex::PERSIAN_NUMBERS.into_iter() {
                        l = l.replace(n, p);
                    }

                    // TODO: Make this optional
                    // Put RTL char in start of line (it forces some player to show that line RTL)
                    // format!("\u{202b}{l}")
                    l
                }
            }
        };

        let contents = self
            .contents
            .lines()
            .map(fix_line)
            .intersperse("\n".to_owned())
            .collect::<String>();
        self.contents = contents;
    }
}

#[cfg(test)]
mod tests {
    use crate::Subtitle;

    #[test]
    fn will_remove_italics() {
        let input = r#"1
00:08:26,025 --> 00:08:30,091
<i>!زود باشین برین</i>

!نمی‌ذارم هیچ کدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let expected = r#"1
00:08:26,025 --> 00:08:30,091
!زود باشین برین

!نمی‌ذارم هیچ کدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let output = Subtitle::new(input.to_string()).fix();

        assert_eq!(expected, output)
    }

    #[test]
    fn will_replace_arabic_chars() {
        let input = r#"1
00:08:26,025 --> 00:08:30,091
!زود باشین برين

!نمی‌ذارم هیچ كدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let expected = r#"1
00:08:26,025 --> 00:08:30,091
!زود باشین برین

!نمی‌ذارم هیچ کدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let output = Subtitle::new(input.to_string()).fix();

        assert_eq!(expected, output)
    }

    #[test]
    fn will_replace_question_mark() {
        let input = r#"1
00:08:26,025 --> 00:08:30,091
؟زود باشین برين

?نمی‌ذارم هیچ كدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let expected = r#"1
00:08:26,025 --> 00:08:30,091
؟زود باشین برین

؟نمی‌ذارم هیچ کدومتون اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه عجله کنین"#;

        let output = Subtitle::new(input.to_string()).fix();

        assert_eq!(expected, output)
    }

    #[test]
    fn will_replace_numbers_with_persian_numbers() {
        let input = r#"1
00:08:26,025 --> 00:08:30,091
؟زود باشین برين 12

؟نمی‌ذارم 2 هیچ كدومتون 0912 اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه 34عجله کنین"#;

        let expected = r#"1
00:08:26,025 --> 00:08:30,091
؟زود باشین برین ۱۲

؟نمی‌ذارم ۲ هیچ کدومتون ۰۹۱۲ اینجا بمیرین

2
00:08:22,037 --> 00:08:24,008
!همه ۳۴عجله کنین"#;

        let output = Subtitle::new(input.to_string()).fix();

        assert_eq!(expected, output)
    }
}
