use matetui::ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Buffer, Line, Rect, Span, Stylize, Widget},
    style::Style,
    widgets::Paragraph,
};
use std::rc::Rc;

pub struct HelpItem {
    pub description: String,
    pub keys: Vec<String>,
    pub observation: Option<String>,
}

pub struct HelpSection {
    pub title: String,
    pub items: Vec<HelpItem>,
}

pub struct CocoHelp {
    sections: Vec<HelpSection>,
}

impl CocoHelp {
    pub fn new(sections: Vec<HelpSection>) -> Self {
        Self { sections }
    }

    fn layout(&self, area: Rect) -> [Rect; 1] {
        let total_height = self.sections.iter().fold(0, |acc, section| {
            // each section has a title and a list of items
            // the title is one line, and each item is one line
            // so the height of the section is the number of items + 1 (for the title)
            acc + section.items.len() + 1
        }) + (self.sections.len() - 1); // +1 blank line after each section, -1 for the last section

        Layout::default()
            .constraints([Constraint::Length(total_height as u16)])
            .flex(Flex::Center)
            .direction(Direction::Vertical)
            .areas(area)
    }

    fn sections_layout(&self, area: Rect) -> Rc<[Rect]> {
        let section_count = self.sections.len();
        let constraints: Vec<Constraint> = self
            .sections
            .iter()
            .enumerate()
            .map(|(index, section)| {
                let items_count = section.items.len();
                let mut height = items_count + 1; // Items + Title
                                                  // Add an extra space if it's not the last section
                if index < section_count - 1 {
                    height += 1;
                }
                Constraint::Length(height as u16)
            })
            .collect();

        Layout::vertical(constraints).split(area)
    }

    fn section_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::vertical(vec![Constraint::Length(1), Constraint::Min(0)]).areas(area)
    }

    fn items_layout(&self, area: Rect, section: &HelpSection) -> Rc<[Rect]> {
        Layout::vertical(section.items.iter().map(|_| Constraint::Length(1)).collect::<Vec<_>>())
            .split(area)
    }

    fn item_layout(&self, area: Rect) -> [Rect; 2] {
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).areas(area)
    }
}

impl Widget for CocoHelp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [area] = self.layout(area);
        let sections_layout = self.sections_layout(area);

        for (i, section) in self.sections.iter().enumerate() {
            let [title_area, items_area] = self.section_layout(sections_layout[i]);

            // draw the title of the section
            let title = Paragraph::new(section.title.as_str()).blue().bold();
            title.render(title_area, buf);

            // draw each of the items in the section
            let items_layout = self.items_layout(items_area, section);
            for (j, item) in section.items.iter().enumerate() {
                let [desc_area, keys_area] = self.item_layout(items_layout[j]);

                // Render the description on the left
                let desc_width = item.description.len() as u16;
                let dots_width = desc_area.width.saturating_sub(desc_width).saturating_sub(5);
                let dots = ".".repeat(dots_width as usize);

                let description = Paragraph::new(Line::from(vec![
                    item.description.clone().into(),
                    " ".into(),
                    dots.dim(),
                    "  : ".dim(),
                ]));
                description.render(desc_area, buf);

                // Render the key and observation on the right
                let mut keys = item.keys.iter().fold(Vec::new(), |mut acc, key| {
                    if !acc.is_empty() {
                        acc.push(" or ".dim());
                    }
                    acc.push(key.into());
                    acc
                });

                // push observation between brackets and dimmed
                if let Some(observation) = &item.observation {
                    keys.push(Span::from(format!(" ({})", observation)).dim());
                }

                let keys = Line::from(keys);
                let keys = Paragraph::new(keys);
                keys.render(keys_area, buf);
            }

            // Add a space after each section except the last one
            if i < self.sections.len() - 1 {
                let empty_space = Paragraph::new(" ") // Single space for spacing
                    .style(Style::default());
                // Render it in the next available section area
                empty_space.render(sections_layout[i + 1], buf);
            }
        }
    }
}

/// Macro to create a vector of `HelpSection` from a list of sections and items.
///
/// Just a convenience macro to avoid writing the entire tree of `HelpSection`s and `HelpItem`s
/// repeatedly.
#[macro_export]
macro_rules! help {
    (
        $( $section:tt => {
            $(
                $desc:tt : [ $( $key:expr ),+ ] $( ; observation => $obs:expr )? ,
            )*
        } )*
    ) => {
        vec![
            $(
                $crate::widgets::coco_help::HelpSection {
                    title: $section.to_string(),
                    items: vec![
                        $(
                            $crate::widgets::coco_help::HelpItem {
                                description: $desc.to_string(),
                                keys: vec![ $( $key.to_string() ),+ ],
                                observation: None $( .or(Some($obs.to_string())) )?,
                            }
                        ),*
                    ],
                }
            ),*
        ]
    };
}
