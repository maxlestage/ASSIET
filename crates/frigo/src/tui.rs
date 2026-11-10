//! L'interface plein écran : le frigo en haut, les recettes à gauche, la recette
//! choisie à droite (empilées sur un terminal étroit).

use std::time::Duration;

use color_eyre::eyre::Result;
use frigo_core::{Criteria, Match, MealDb, parse_list};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, BorderType, Gauge, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc;

use crate::print::steps;

const ACCENT: Color = Color::Rgb(255, 122, 89);
const FRESH: Color = Color::Rgb(94, 234, 212);
const MUTED: Color = Color::Rgb(140, 140, 160);
const SPINNER: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Focus {
    Fridge,
    List,
    Recipe,
}

enum Status {
    Idle,
    Loading,
    Done,
    Failed(String),
}

struct App {
    db: MealDb,
    criteria: Criteria,
    input: String,
    cursor: usize,
    focus: Focus,
    matches: Vec<Match>,
    list: ListState,
    scroll: u16,
    status: Status,
    tick: usize,
    quit: bool,
}

pub async fn run(db: MealDb, fridge: Vec<String>, criteria: Criteria) -> Result<()> {
    let input = fridge.join(", ");
    let mut app = App {
        db,
        criteria,
        cursor: input.chars().count(),
        focus: Focus::Fridge,
        input,
        matches: Vec::new(),
        list: ListState::default(),
        scroll: 0,
        status: Status::Idle,
        tick: 0,
        quit: false,
    };
    let mut terminal = ratatui::init();
    let result = app.event_loop(&mut terminal).await;
    ratatui::restore();
    result
}

type Answer = Result<Vec<Match>, String>;

impl App {
    async fn event_loop(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Answer>();
        if !self.input.trim().is_empty() {
            self.search(&tx);
        }
        while !self.quit {
            terminal.draw(|f| self.draw(f))?;
            if let Ok(answer) = rx.try_recv() {
                self.receive(answer);
            }
            // Le terminal est lu sans bloquer le moteur asynchrone.
            let has_event = tokio::task::block_in_place(|| event::poll(Duration::from_millis(80)))?;
            if has_event
                && let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                self.key(key.code, key.modifiers, &tx);
            }
            self.tick = self.tick.wrapping_add(1);
        }
        Ok(())
    }

    fn search(&mut self, tx: &mpsc::UnboundedSender<Answer>) {
        let fridge = parse_list(&self.input);
        if fridge.is_empty() {
            self.status = Status::Failed("Le frigo est vide : tapez quelques ingrédients.".into());
            return;
        }
        self.status = Status::Loading;
        let db = self.db.clone();
        let criteria = self.criteria.clone();
        let tx = tx.clone();
        tokio::spawn(async move {
            let answer = db
                .suggest(&fridge, &criteria)
                .await
                .map_err(|e| e.to_string());
            let _ = tx.send(answer);
        });
    }

    fn receive(&mut self, answer: Answer) {
        match answer {
            Ok(matches) => {
                self.list.select((!matches.is_empty()).then_some(0));
                self.matches = matches;
                self.scroll = 0;
                self.status = Status::Done;
                if !self.matches.is_empty() {
                    self.focus = Focus::List;
                }
            }
            Err(e) => self.status = Status::Failed(e),
        }
    }

    fn key(&mut self, code: KeyCode, modifiers: KeyModifiers, tx: &mpsc::UnboundedSender<Answer>) {
        if modifiers.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
            self.quit = true;
            return;
        }
        if code == KeyCode::Tab {
            self.focus = match self.focus {
                Focus::Fridge => Focus::List,
                Focus::List => Focus::Recipe,
                Focus::Recipe => Focus::Fridge,
            };
            return;
        }
        if self.focus == Focus::Fridge {
            let len = self.input.chars().count();
            match code {
                KeyCode::Enter => self.search(tx),
                KeyCode::Esc => self.focus = Focus::List,
                KeyCode::Left => self.cursor = self.cursor.saturating_sub(1),
                KeyCode::Right => self.cursor = (self.cursor + 1).min(len),
                KeyCode::Home => self.cursor = 0,
                KeyCode::End => self.cursor = len,
                KeyCode::Backspace if self.cursor > 0 => {
                    self.cursor -= 1;
                    let at = self.byte(self.cursor);
                    self.input.remove(at);
                }
                KeyCode::Delete if self.cursor < len => {
                    let at = self.byte(self.cursor);
                    self.input.remove(at);
                }
                KeyCode::Char(c) => {
                    let at = self.byte(self.cursor);
                    self.input.insert(at, c);
                    self.cursor += 1;
                }
                _ => {}
            }
            return;
        }
        match code {
            KeyCode::Char('q') | KeyCode::Esc => self.quit = true,
            KeyCode::Char('i') | KeyCode::Char('/') | KeyCode::Char('e') => {
                self.focus = Focus::Fridge
            }
            KeyCode::Char('s') => {
                self.criteria.strict = !self.criteria.strict;
                self.search(tx);
            }
            KeyCode::Char('p') => {
                self.criteria.pantry = !self.criteria.pantry;
                self.search(tx);
            }
            KeyCode::Char('r') => self.search(tx),
            KeyCode::Enter => self.focus = Focus::Recipe,
            KeyCode::Down | KeyCode::Char('j') if self.focus == Focus::List => self.select(1),
            KeyCode::Up | KeyCode::Char('k') if self.focus == Focus::List => self.select(-1),
            KeyCode::Down | KeyCode::Char('j') => self.scroll = self.scroll.saturating_add(1),
            KeyCode::Up | KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
            KeyCode::PageDown => self.scroll = self.scroll.saturating_add(10),
            KeyCode::PageUp => self.scroll = self.scroll.saturating_sub(10),
            _ => {}
        }
    }

    fn byte(&self, chars: usize) -> usize {
        self.input
            .char_indices()
            .nth(chars)
            .map_or(self.input.len(), |(i, _)| i)
    }

    fn select(&mut self, delta: i32) {
        if self.matches.is_empty() {
            return;
        }
        let last = self.matches.len() as i32 - 1;
        let current = self.list.selected().unwrap_or(0) as i32;
        self.list
            .select(Some((current + delta).clamp(0, last) as usize));
        self.scroll = 0;
    }

    fn block(&self, title: &str, focus: Focus) -> Block<'static> {
        let active = self.focus == focus;
        Block::bordered()
            .border_type(if active {
                BorderType::Thick
            } else {
                BorderType::Rounded
            })
            .border_style(Style::new().fg(if active { ACCENT } else { MUTED }))
            .title(Span::styled(format!(" {title} "), Style::new().bold()))
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [top, body, help] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(1),
        ])
        .areas(area);
        self.draw_fridge(frame, top);

        // Étroit (téléphone, petite fenêtre) : la liste au-dessus de la recette.
        let narrow = body.width < 90;
        let [left, right] = if narrow {
            Layout::vertical([Constraint::Percentage(35), Constraint::Percentage(65)]).areas(body)
        } else {
            Layout::horizontal([Constraint::Percentage(38), Constraint::Percentage(62)]).areas(body)
        };
        self.draw_list(frame, left);
        self.draw_recipe(frame, right);

        let flags = format!(
            " strict:{} placard:{} ",
            if self.criteria.strict { "oui" } else { "non" },
            if self.criteria.pantry { "oui" } else { "non" }
        );
        let keys = if self.focus == Focus::Fridge {
            " Entrée chercher · Tab changer de zone · Échap liste "
        } else {
            " ↑↓ choisir · Tab zone · i frigo · s strict · p placard · r relancer · q quitter "
        };
        frame.render_widget(
            Line::from(vec![
                Span::styled(flags, Style::new().fg(Color::Black).bg(FRESH)),
                Span::styled(keys, Style::new().fg(MUTED)),
            ]),
            help,
        );
    }

    fn draw_fridge(&self, frame: &mut Frame, area: Rect) {
        let block = self.block("🧊 Mon frigo", Focus::Fridge);
        let inner = block.inner(area);
        let text = if self.input.is_empty() {
            Line::from(Span::styled(
                "poulet, riz, oignon…",
                Style::new().fg(MUTED).italic(),
            ))
        } else {
            Line::from(self.input.as_str())
        };
        // Le texte défile pour que le curseur reste visible.
        let offset = (self.cursor as u16).saturating_sub(inner.width.saturating_sub(1));
        frame.render_widget(Paragraph::new(text).block(block).scroll((0, offset)), area);
        if self.focus == Focus::Fridge {
            frame.set_cursor_position((inner.x + self.cursor as u16 - offset, inner.y));
        }
    }

    fn draw_list(&mut self, frame: &mut Frame, area: Rect) {
        let title = format!("🍽  Recettes ({})", self.matches.len());
        let block = self.block(&title, Focus::List);
        match &self.status {
            Status::Loading => {
                let spinner = SPINNER[self.tick % SPINNER.len()];
                let text = Text::from(vec![
                    Line::from(""),
                    Line::from(vec![
                        Span::styled(format!(" {spinner} "), Style::new().fg(ACCENT)),
                        Span::raw("TheMealDB fouille son garde-manger…"),
                    ]),
                ]);
                frame.render_widget(Paragraph::new(text).block(block), area);
                return;
            }
            Status::Failed(e) => {
                let text = Paragraph::new(Span::styled(e.clone(), Style::new().fg(Color::Red)))
                    .wrap(Wrap { trim: true })
                    .block(block);
                frame.render_widget(text, area);
                return;
            }
            Status::Idle => {
                let text = Paragraph::new("Tapez vos ingrédients puis Entrée.")
                    .fg(MUTED)
                    .wrap(Wrap { trim: true })
                    .block(block);
                frame.render_widget(text, area);
                return;
            }
            Status::Done if self.matches.is_empty() => {
                let text = Paragraph::new(
                    "Aucune recette. Essayez « s » (strict) ou d'autres ingrédients.",
                )
                .fg(MUTED)
                .wrap(Wrap { trim: true })
                .block(block);
                frame.render_widget(text, area);
                return;
            }
            Status::Done => {}
        }
        let items: Vec<ListItem> = self
            .matches
            .iter()
            .map(|m| {
                let badge = if m.complete() {
                    Span::styled(" ✓ ", Style::new().fg(Color::Black).bg(FRESH))
                } else {
                    Span::styled(
                        format!(" -{} ", m.missing.len()),
                        Style::new().fg(Color::Black).bg(ACCENT),
                    )
                };
                ListItem::new(Line::from(vec![
                    badge,
                    Span::raw(" "),
                    Span::raw(m.meal.name.clone()),
                    Span::styled(format!("  {} min", m.minutes), Style::new().fg(MUTED)),
                ]))
            })
            .collect();
        let list = List::new(items)
            .block(block)
            .highlight_symbol("▶ ")
            .highlight_style(Style::new().add_modifier(Modifier::BOLD).fg(ACCENT));
        frame.render_stateful_widget(list, area, &mut self.list);
    }

    fn draw_recipe(&self, frame: &mut Frame, area: Rect) {
        let selected = self.list.selected().and_then(|i| self.matches.get(i));
        let Some(m) = selected else {
            let block = self.block("Recette", Focus::Recipe);
            let art = Text::from(vec![
                Line::from(""),
                Line::from("    ┌──────────┐"),
                Line::from("    │  ◦    ◦  │   Rien dans l'assiette…"),
                Line::from("    │──────────│   pour l'instant."),
                Line::from("    │   ____   │"),
                Line::from("    └──────────┘"),
            ])
            .fg(MUTED);
            frame.render_widget(Paragraph::new(art).block(block), area);
            return;
        };
        let block = self.block(&m.meal.name, Focus::Recipe);
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let [gauge_area, text_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(1)]).areas(inner);
        let gauge = Gauge::default()
            .gauge_style(Style::new().fg(FRESH).bg(Color::Rgb(40, 40, 52)))
            .ratio(f64::from(m.coverage()).clamp(0.0, 1.0))
            .label(format!(
                "{:.0} % des ingrédients · ≈ {} min",
                m.coverage() * 100.0,
                m.minutes
            ));
        frame.render_widget(gauge, gauge_area);

        let mut lines = vec![Line::from("")];
        let mut meta = Vec::new();
        meta.extend(m.meal.category.clone());
        meta.extend(m.meal.area.clone());
        if !meta.is_empty() {
            lines.push(Line::from(Span::styled(
                meta.join(" · "),
                Style::new().fg(MUTED),
            )));
            lines.push(Line::from(""));
        }
        for ingredient in &m.meal.ingredients {
            let ok = m.have.contains(&ingredient.name);
            lines.push(Line::from(vec![
                if ok {
                    Span::styled(" ✓ ", Style::new().fg(FRESH).bold())
                } else {
                    Span::styled(" ✗ ", Style::new().fg(ACCENT).bold())
                },
                Span::raw(ingredient.name.clone()),
                Span::styled(
                    if ingredient.measure.is_empty() {
                        String::new()
                    } else {
                        format!("  {}", ingredient.measure)
                    },
                    Style::new().fg(MUTED),
                ),
            ]));
        }
        lines.push(Line::from(""));
        for (i, step) in steps(&m.meal.instructions).into_iter().enumerate() {
            lines.push(Line::from(vec![
                Span::styled(format!("{:>2}. ", i + 1), Style::new().fg(ACCENT).bold()),
                Span::raw(step),
            ]));
        }
        if let Some(video) = &m.meal.youtube {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                format!("▶ {video}"),
                Style::new().fg(MUTED),
            )));
        }
        frame.render_widget(
            Paragraph::new(lines)
                .wrap(Wrap { trim: false })
                .scroll((self.scroll, 0)),
            text_area,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn app() -> App {
        App {
            db: MealDb::default(),
            criteria: Criteria::default(),
            input: "poulet, riz".into(),
            cursor: 11,
            focus: Focus::Fridge,
            matches: Vec::new(),
            list: ListState::default(),
            scroll: 0,
            status: Status::Idle,
            tick: 0,
            quit: false,
        }
    }

    #[test]
    fn draws_on_wide_and_narrow_terminals() {
        for (w, h) in [(120, 40), (40, 30)] {
            let mut terminal = Terminal::new(TestBackend::new(w, h)).unwrap();
            let mut app = app();
            terminal.draw(|f| app.draw(f)).unwrap();
            let screen: String = terminal
                .backend()
                .buffer()
                .content()
                .iter()
                .map(|c| c.symbol())
                .collect();
            assert!(screen.contains("poulet, riz"));
        }
    }

    #[test]
    fn typing_edits_the_fridge() {
        let (tx, _rx) = mpsc::unbounded_channel();
        let mut app = app();
        for c in ", ail".chars() {
            app.key(KeyCode::Char(c), KeyModifiers::NONE, &tx);
        }
        app.key(KeyCode::Backspace, KeyModifiers::NONE, &tx);
        assert_eq!(app.input, "poulet, riz, ai");
        app.key(KeyCode::Tab, KeyModifiers::NONE, &tx);
        app.key(KeyCode::Char('q'), KeyModifiers::NONE, &tx);
        assert!(app.quit);
    }
}
