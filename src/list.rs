use tui::style::Style;

pub struct StatefulList<T> {
    pub items: Vec<T>,
    selected: Option<usize>,
    highlight_symbol: String,
    highlight_style: Style,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> Self {
        Self {
            items,
            selected: None,
            highlight_symbol: String::new(),
            highlight_style: Style::default(),
        }
    }

    pub fn with_highlight_symbol(&mut self, symbol: impl AsRef<str>) -> &mut Self {
        self.highlight_symbol = String::from(symbol.as_ref());

        self
    }

    pub fn with_highlight_style(&mut self, style: Style) -> &mut Self {
        self.highlight_style = style;

        self
    }

    pub fn highlight_symbol(&self) -> &String {
        &self.highlight_symbol
    }

    pub fn highlight_style(&self) -> Style {
        self.highlight_style
    }

    pub fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.select(Some(i));
    }

    pub fn selected(&self) -> Option<usize> {
        self.selected
    }

    pub fn selected_item(&self) -> Option<&T> {
        let i = self.selected()?;
        self.items.get(i)
    }

    pub fn unselect(&mut self) {
        self.select(None);
    }

    pub fn select(&mut self, item: Option<usize>) {
        self.selected = item;
    }
}
