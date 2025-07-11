mod search;

use {
    crate::{s, tui::color},
    matetui::{ratatui::style::Color, widgets::gridselector::GridItem},
    rust_i18n::t,
    search::fetch_config_paths,
    serde::{Deserialize, Deserializer},
    serde_yml::from_reader,
    std::{collections::HashMap, fs::File, path::PathBuf},
};

/// Custom deserializer for Color.
fn deserialize_color<'de, D>(deserializer: D) -> Result<HashMap<String, Color>, D::Error>
where
    D: Deserializer<'de>,
{
    let hex_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;
    Ok(hex_map.into_iter().map(|(k, v)| (k, color(&v))).collect())
}

#[derive(Debug, Deserialize, Clone)]
pub struct Theme(#[serde(deserialize_with = "deserialize_color")] HashMap<String, Color>);

impl Default for Theme {
    fn default() -> Self {
        Theme(HashMap::from([
            // logo
            (s!("logo:fg:1"), Color::Blue),
            (s!("logo:fg:2"), Color::LightMagenta),
            // header
            (s!("header:type"), Color::Yellow),
            (s!("header:scope"), Color::Blue),
            // grid-selector
            (s!("grid:selected"), Color::Green),
            (s!("grid:hovered"), Color::Blue),
            // switch
            (s!("switch:on"), color("#00c980")),
            (s!("switch:off"), Color::Black),
            (s!("switch:switch"), Color::White),
            // Yes/No
            (s!("yes"), color("#00c980")),
            (s!("no"), Color::Red),
            // textarea main
            (s!("textarea:bg"), color("#050f21")),
            (s!("textarea:fg"), color("#ffffff")),
            (s!("textarea:sel"), color("#232a38")),
            // scope-textarea
            (s!("scope:bg"), color("#125acc")),
            (s!("scope:fg"), color("#ffffff")),
            (s!("scope:sec"), color("#000000")),
            // summary-textarea
            (s!("summary:bg"), color("#6a2eab")),
            (s!("summary:fg"), color("#ffffff")),
            (s!("summary:sec"), color("#000000")),
            // body-textarea
            (s!("body:bg"), color("#f24e50")),
            (s!("body:fg"), color("#ffffff")),
            (s!("body:sec"), color("#000000")),
            // footer-textarea
            (s!("footer:bg"), color("#db279f")),
            (s!("footer:fg"), color("#ffffff")),
            (s!("footer:sec"), color("#000000")),
        ]))
    }
}

impl Theme {
    pub fn get(&self, key: &str) -> Color {
        self.0.get(key).copied().unwrap_or(Color::Reset)
    }

    pub fn merge(self, other: Self) -> Self {
        let mut merged = self.0.clone();
        for (key, value) in other.0 {
            merged.insert(key, value);
        }
        Theme(merged)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommitKind {
    pub name: String,
    pub emoji: String,
    pub description: String,
}

impl From<CommitKind> for GridItem {
    fn from(val: CommitKind) -> Self {
        GridItem::new(val.name)
    }
}

fn ctype(name: &str, emoji: &str, description: &str) -> CommitKind {
    CommitKind {
        name: s!(name),
        emoji: s!(emoji),
        description: s!(description),
    }
}

#[derive(Debug, Clone)]
pub struct CocoConfig {
    pub theme: Theme,
    pub max_summary_length: usize,
    pub use_emoji: bool,
    pub ask_scope: bool,
    pub ask_body: bool,
    pub ask_footer: bool,
    pub ask_breaking_change: bool,
    pub scopes: Vec<String>,
    pub types: Vec<CommitKind>,
}

#[derive(Debug, Deserialize, Default)]
struct PartialConfig {
    pub theme: Option<Theme>,
    #[serde(alias = "maxSummaryLength")]
    pub max_summary_length: Option<usize>,
    #[serde(alias = "useEmoji")]
    pub use_emoji: Option<bool>,
    #[serde(alias = "askScope")]
    pub ask_scope: Option<bool>,
    #[serde(alias = "askBody")]
    pub ask_body: Option<bool>,
    #[serde(alias = "askFooter")]
    pub ask_footer: Option<bool>,
    #[serde(alias = "askBreakingChange")]
    pub ask_breaking_change: Option<bool>,
    #[serde(alias = "scopes")]
    pub scopes: Option<Vec<String>>,
    #[serde(alias = "types")]
    pub types: Option<Vec<CommitKind>>,
}

impl Default for CocoConfig {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            max_summary_length: 72,
            use_emoji: true,
            ask_scope: true,
            ask_body: true,
            ask_footer: true,
            ask_breaking_change: true,
            scopes: vec![],
            types: vec![
                ctype("feat", "✨", &t!("Introduces a new feature")),
                ctype("fix", "🚑", &t!("Fixes a bug")),
                ctype("chore", "🧹", &t!("Other changes that don't modify src or test files")),
                ctype("docs", "📝", &t!("Documentation only changes")),
                ctype("style", "💄", &t!("Code cosmetic changes (formatting, indentation, etc.)")),
                ctype(
                    "refactor",
                    "🔨",
                    &t!("A change that refactors code without adding or removing features"),
                ),
                ctype("perf", "🐎", &t!("A code change that improves performance")),
                ctype("test", "🧪", &t!("A change that only adds or updates tests")),
                ctype("ci", "🔄", &t!("Changes to our CI configuration files and scripts")),
                ctype("revert", "🔙", &t!("Reverts a previous commit")),
                ctype("release", "🔖", &t!("Releases a new version")),
                ctype("wip", "🚧", &t!("Work in progress")),
                ctype(
                    "i18n",
                    "🌐",
                    &t!("A change that updates or adds translations (internationalization)"),
                ),
            ],
        }
    }
}

impl CocoConfig {
    pub fn from_files() -> Self {
        // Load the default configuration
        let default_config = Self::default();

        // Fetch configuration file paths
        let (home_config_path, current_config_path) = match fetch_config_paths() {
            Ok((home, current)) => (home, current),
            Err(_) => (None, None),
        };

        // Load partial configurations
        let maybe_home_cfg = home_config_path.map(Self::load_partial_from_file);
        let maybe_cwd_cfg = current_config_path.map(Self::load_partial_from_file);

        // Merge configurations
        Self::merge_configs(default_config, maybe_home_cfg, maybe_cwd_cfg)
    }

    fn load_partial_from_file(path: PathBuf) -> PartialConfig {
        let file = File::open(path).unwrap();
        from_reader(file).unwrap()
    }

    fn merge_configs(
        default: Self,
        home: Option<PartialConfig>,
        current: Option<PartialConfig>,
    ) -> Self {
        let mut config = default;

        if let Some(home) = home {
            config.theme = home.theme.map_or(config.theme.clone(), |t| config.theme.merge(t));
            config.max_summary_length =
                home.max_summary_length.unwrap_or(config.max_summary_length);
            config.use_emoji = home.use_emoji.unwrap_or(config.use_emoji);
            config.ask_scope = home.ask_scope.unwrap_or(config.ask_scope);
            config.ask_body = home.ask_body.unwrap_or(config.ask_body);
            config.ask_footer = home.ask_footer.unwrap_or(config.ask_footer);
            config.ask_breaking_change =
                home.ask_breaking_change.unwrap_or(config.ask_breaking_change);
            config.scopes = home.scopes.unwrap_or(config.scopes);
            config.types = home.types.unwrap_or(config.types);
        }

        if let Some(current) = current {
            config.theme = current.theme.map_or(config.theme.clone(), |t| config.theme.merge(t));
            config.max_summary_length =
                current.max_summary_length.unwrap_or(config.max_summary_length);
            config.use_emoji = current.use_emoji.unwrap_or(config.use_emoji);
            config.ask_scope = current.ask_scope.unwrap_or(config.ask_scope);
            config.ask_body = current.ask_body.unwrap_or(config.ask_body);
            config.ask_footer = current.ask_footer.unwrap_or(config.ask_footer);
            config.ask_breaking_change =
                current.ask_breaking_change.unwrap_or(config.ask_breaking_change);
            config.scopes = current.scopes.unwrap_or(config.scopes);
            config.types = current.types.unwrap_or(config.types);
        }

        config
    }
}
