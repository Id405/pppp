use std::env::var;

#[derive(PartialEq)]
pub struct StyleConfig {
    pub foreground_color: u8, // TODO eventually this should be either a terminal color code, ansi color, or hex code
    pub background_color: u8,
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub strikethrough: bool,
    pub light: bool,
}

fn prefix_string(prefix: &str, suffix: &str) -> String {
    format!("{}_STYLE_{}", prefix, suffix)
}

impl StyleConfig {
    pub fn from_env(prefix: &str, default: Self) -> Self {
        let foreground_color = var(prefix_string(prefix, "FG"))
            .unwrap_or(format!("{}", default.foreground_color))
            .parse()
            .unwrap_or(default.foreground_color);

        let background_color = var(prefix_string(prefix, "BG"))
            .unwrap_or(format!("{}", default.background_color))
            .parse()
            .unwrap_or(default.background_color);

        let italic = var(prefix_string(prefix, "ITALIC"))
            .unwrap_or(format!("{}", default.italic))
            .parse()
            .unwrap_or(default.italic);

        let bold = var(prefix_string(prefix, "BOLD"))
            .unwrap_or(format!("{}", default.bold))
            .parse()
            .unwrap_or(default.bold);

        let underline = var(prefix_string(prefix, "UNDERLINE"))
            .unwrap_or(format!("{}", default.underline))
            .parse()
            .unwrap_or(default.underline);

        let light = var(prefix_string(prefix, "LIGHT"))
            .unwrap_or(format!("{}", default.light))
            .parse()
            .unwrap_or(default.light);

        let strikethrough = var(prefix_string(prefix, "STRIKETHROUGH"))
            .unwrap_or(format!("{}", default.strikethrough))
            .parse()
            .unwrap_or( default.strikethrough);

        Self {
            foreground_color,
            background_color,
            italic,
            bold,
            underline,
            light,
            strikethrough,
        }
    }

    pub fn default() -> Self {
        Self {
            foreground_color: 7,
            background_color: 0,
            italic: false,
            bold: false,
            underline: false,
            light: false,
            strikethrough: false,
        }
    }
}

impl ToString for StyleConfig {
    fn to_string(&self) -> String {
        let mut str = format!(
            "\x1b[38;5;{}m\x1b[48;5;{}m",
            self.foreground_color, self.background_color
        );

        if self.bold {
            str.push_str("\x1b[1m");
        }

        if self.light {
            str.push_str("\x1b[2m");
        }

        if self.italic {
            str.push_str("\x1b[3m");
        }

        if self.underline {
            str.push_str("\x1b[4m");
        }

        if self.strikethrough {
            str.push_str("\x1b[9m");
        }

        str
    }
}