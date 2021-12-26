use std::ffi::OsStr;

#[derive(Copy, Clone, Debug)]
pub enum Colour {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Purple = 35,
    Cyan = 36,
    White = 37,
}
pub enum Style {
    Plain,
    Foreground(Colour),
    CustomStyle(StyleStruct),
}
struct StyleStruct {
    foreground: Colour,
    background: Option<Colour>,
    bold: bool,
    underline: bool,
}
/// SGR 参数
/// https://zh.wikipedia.org/wiki/ANSI%E8%BD%AC%E4%B9%89%E5%BA%8F%E5%88%97#%E9%80%89%E6%8B%A9%E5%9B%BE%E5%BD%A2%E5%86%8D%E7%8E%B0%EF%BC%88SGR%EF%BC%89%E5%8F%82%E6%95%B0
///
impl Colour {
    pub fn paint(&self, input: &[u8]) -> String {
        format!(
            "\x1B[{}m{}\x1B[0m",
            *self as usize,
            std::str::from_utf8(input).unwrap()
        )
    }

    pub fn underline(&self) -> Style {
        Style::CustomStyle(StyleStruct {
            foreground: *self,
            background: None,
            bold: false,
            underline: true,
        })
    }
    pub fn bold(&self) -> Style {
        Style::CustomStyle(StyleStruct {
            foreground: *self,
            background: None,
            bold: true,
            underline: false,
        })
    }
    pub fn normal(&self) -> Style {
        Style::CustomStyle(StyleStruct {
            foreground: *self,
            background: None,
            bold: false,
            underline: false,
        })
    }
    pub fn on(&self, background: Colour) -> Style {
        Style::CustomStyle(StyleStruct {
            foreground: *self,
            background: Some(background),
            bold: false,
            underline: false,
        })
    }
}

impl Style {
    pub fn paint(&self, input: &[u8]) -> String {
        match self {
            Style::Plain => String::from_utf8(input.to_vec()).unwrap(),
            Style::Foreground(colour) => colour.paint(input),
            Style::CustomStyle(s) => match s {
                StyleStruct {
                    foreground,
                    background,
                    bold,
                    underline,
                } => {
                    let bg = match background {
                        None => String::from(""),
                        Some(bg) => format!("{};", *bg as usize + 10),
                    };
                    let bo = if *bold { "1;" } else { "" };
                    let un = if *underline { "4;" } else { "" };
                    format!(
                        "\x1B[{}{}{}{}m{}\x1B[0m",
                        bo,
                        un,
                        bg,
                        *foreground as usize,
                        std::str::from_utf8(input).unwrap()
                    )
                }
            },
        }
    }
    pub fn bold(&self) -> Style {
        match self {
            Style::Plain => Style::CustomStyle(StyleStruct {
                foreground: Colour::White,
                background: None,
                bold: true,
                underline: false,
            }),
            Style::Foreground(c) => Style::CustomStyle(StyleStruct {
                foreground: *c,
                background: None,
                bold: true,
                underline: false,
            }),
            Style::CustomStyle(st) => Style::CustomStyle(StyleStruct {
                foreground: st.foreground,
                background: st.background,
                bold: true,
                underline: false,
            }),
        }
    }

    pub fn underline(&self) -> Style {
        match self {
            Style::Plain => Style::CustomStyle(StyleStruct {
                foreground: Colour::White,
                background: None,
                bold: false,
                underline: true,
            }),
            Style::Foreground(c) => Style::CustomStyle(StyleStruct {
                foreground: *c,
                background: None,
                bold: false,
                underline: true,
            }),
            Style::CustomStyle(st) => Style::CustomStyle(StyleStruct {
                foreground: st.foreground,
                background: st.background,
                bold: false,
                underline: true,
            }),
        }
    }

    pub fn on(&self, background: Colour) -> Style {
        match self {
            Style::Plain => Style::CustomStyle(StyleStruct {
                foreground: Colour::White,
                background: Some(background),
                bold: false,
                underline: false,
            }),
            Style::Foreground(c) => Style::CustomStyle(StyleStruct {
                foreground: *c,
                background: Some(background),
                bold: false,
                underline: false,
            }),
            Style::CustomStyle(st) => Style::CustomStyle(StyleStruct {
                foreground: st.foreground,
                background: Some(background),
                bold: false,
                underline: false,
            }),
        }
    }
}
