use std::str::FromStr;
use syntect::highlighting::{
    Color, FontStyle, ScopeSelectors, StyleModifier, Theme, ThemeItem, ThemeSettings,
};

const BG: Color = hex("#1e1e1e");
const FG: Color = hex("#e2e2e5");

const SALLY_YELLOW: Color = hex("#e0c020");

const GRAY_100: Color = hex("#eeeeee");
const GRAY_400: Color = hex("#888888");
const GRAY_500: Color = hex("#666666");

pub fn sally() -> Theme {
    Theme {
        name: Some("Sally".into()),
        author: None,
        settings: ThemeSettings {
            background: Some(BG),
            foreground: Some(FG),
            ..Default::default()
        },
        scopes: vec![
            item(
                "comment, punctuation.definition.comment",
                GRAY_500,
                FontStyle::ITALIC,
            ),
            item("string", SALLY_YELLOW, FontStyle::ITALIC),
            item("string.regexp", SALLY_YELLOW, FontStyle::empty()),
            item(
                "constant.character.escape",
                SALLY_YELLOW,
                FontStyle::empty(),
            ),
            item("constant.numeric", SALLY_YELLOW, FontStyle::empty()),
            item(
                "constant.language, constant.language.boolean, constant.other",
                SALLY_YELLOW,
                FontStyle::empty(),
            ),
            item(
                "variable.other.enummember, constant.other.enum",
                SALLY_YELLOW,
                FontStyle::empty(),
            ),
            item(
                "keyword, storage, storage.type",
                GRAY_400,
                FontStyle::empty(),
            ),
            item("keyword.operator", GRAY_400, FontStyle::empty()),
            item("variable", FG, FontStyle::empty()),
            item(
                "variable.language, variable.parameter.function",
                GRAY_400,
                FontStyle::empty(),
            ),
            item(
                "entity.name.function, support.function",
                GRAY_100,
                FontStyle::BOLD,
            ),
            item(
                "meta.function-call entity.name.function",
                GRAY_100,
                FontStyle::BOLD,
            ),
            item(
                "entity.name.type, entity.name.class, support.class",
                GRAY_100,
                FontStyle::BOLD,
            ),
            item(
                "support.type, support.type.builtin",
                GRAY_400,
                FontStyle::empty(),
            ),
            item("entity.name.tag, meta.tag", GRAY_400, FontStyle::empty()),
            item(
                "entity.other.attribute-name",
                SALLY_YELLOW,
                FontStyle::empty(),
            ),
            item(
                "support.type.property-name, variable.other.property, variable.other.object.property",
                SALLY_YELLOW,
                FontStyle::empty(),
            ),
            item("punctuation, meta.brace", GRAY_400, FontStyle::empty()),
            item(
                "punctuation.separator, punctuation.terminator",
                GRAY_400,
                FontStyle::empty(),
            ),
            item(
                "markup.heading, entity.name.section",
                GRAY_100,
                FontStyle::BOLD,
            ),
            item("markup.bold", FG, FontStyle::BOLD),
            item("markup.italic", FG, FontStyle::ITALIC),
            item("markup.inline.raw, markup.raw", FG, FontStyle::empty()),
            item("markup.underline.link", SALLY_YELLOW, FontStyle::ITALIC),
        ],
    }
}

fn item(scopes: &str, fg: Color, style: FontStyle) -> ThemeItem {
    ThemeItem {
        scope: ScopeSelectors::from_str(scopes).expect("valid scope selector"),
        style: StyleModifier {
            foreground: Some(fg),
            background: None,
            font_style: Some(style),
        },
    }
}

const fn hex(s: &str) -> Color {
    let bytes = s.as_bytes();
    let r = hex_byte(bytes[1], bytes[2]);
    let g = hex_byte(bytes[3], bytes[4]);
    let b = hex_byte(bytes[5], bytes[6]);
    Color { r, g, b, a: 255 }
}

const fn hex_byte(hi: u8, lo: u8) -> u8 {
    hex_digit(hi) * 16 + hex_digit(lo)
}

const fn hex_digit(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'a'..=b'f' => c - b'a' + 10,
        b'A'..=b'F' => c - b'A' + 10,
        _ => 0,
    }
}
