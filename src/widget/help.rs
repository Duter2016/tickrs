use super::block;
use crate::draw::{add_padding, PaddingDirection};

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{Paragraph, Text, Widget};

const TEXT: &str = r#"
Quit: q or <Ctrl+c>
Add Stock:
  - /: open prompt
  - (while adding):
    - <Enter>: accept
    - <Escape>: quit
Remove Stock:
  - k: remove stock
Change Tab:
  - <Tab>: next stock
  - <Shift+Tab>: previous stock
Change Time Frame:
  - <Right>: next time frame
  - <Left>: previous time frame
Toggle Options Pane:
  - o: toggle pane
  - <Escape>: close pane
  - <Tab>: toggle calls / puts
  - Navigate with arrow keys
Toggle Summary Pane:
  - s: toggle pane
Graphing Display:
  - p: toggle pre / post market
  - x: toggle labels
"#;

pub const HELP_WIDTH: u16 = 35;
pub const HELP_HEIGHT: u16 = 27;

#[derive(Copy, Clone)]
pub struct HelpWidget {}

impl HelpWidget {
    pub fn get_rect(self, area: Rect) -> Rect {
        Rect {
            x: (area.width - HELP_WIDTH) / 2,
            y: (area.height - HELP_HEIGHT) / 2,
            width: HELP_WIDTH,
            height: HELP_HEIGHT,
        }
    }
}

impl Widget for HelpWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        block::new(" Help - <ESC> to go back ", None).render(area, buf);

        let text: Vec<_> = TEXT
            .lines()
            .map(|line| Text::styled(format!("{}\n", line), Style::default()))
            .collect();

        let mut help_area = area;
        help_area = add_padding(help_area, 2, PaddingDirection::Left);
        help_area = add_padding(help_area, 1, PaddingDirection::Top);

        Paragraph::new(text.iter()).render(help_area, buf);
    }
}
