use bevy::prelude::*;
use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode};
use rand::Rng;
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::time::Duration;

use fi1e_0::character::{Character, Experience, Name};

#[derive(Resource)]
struct GameState {
    running: bool,
    tick_count: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            running: true,
            tick_count: 0,
        }
    }
}

fn update_system(mut game_state: ResMut<GameState>, mut query: Query<(&Name, &mut Experience)>) {
    game_state.tick_count += 1;

    for (_name, mut experience) in query.iter_mut() {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.05) {
            let xp_gain = rng.gen_range(1..=30);
            experience.0 += xp_gain;
        }
    }
}

fn spawn_entities(mut commands: Commands) {
    commands.spawn(Character::new("f3lix".to_string(), 0));
    commands.spawn(Character::new("j3ssy".to_string(), 0));
    commands.spawn(Character::new("a13x".to_string(), 0));
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .init_resource::<GameState>()
        .add_systems(Startup, spawn_entities)
        .add_systems(Update, update_system);

    let terminal = ratatui::init();
    let result = run(terminal, app);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, mut app: App) -> Result<()> {
    loop {
        app.update();

        let game_state = app.world().resource::<GameState>();
        let should_exit = !game_state.running;
        let tick_count = game_state.tick_count;

        let mut entities_info = Vec::new();
        let mut query = app.world_mut().query::<(&Name, &Experience)>();

        for (name, health) in query.iter(app.world()) {
            entities_info.push(format!("{}: XP: {}", name.0, health.0));
        }

        terminal.draw(|frame| render(frame, tick_count, &entities_info))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        if should_exit {
            break;
        }

        std::thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    Ok(())
}

fn render(frame: &mut Frame, tick_count: u32, entities_info: &[String]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title = Paragraph::new("fi1e-0 PoC")
        .style(Style::default().fg(Color::Cyan))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    let mut display_items = vec![format!("Tick: {}", tick_count)];
    display_items.extend(entities_info.iter().cloned());

    let items: Vec<ListItem> = display_items
        .iter()
        .enumerate()
        .map(|(i, entity)| {
            let color = match i {
                0 => Color::Yellow,  // Tick counter
                1 => Color::Green,   // f3lix
                2 => Color::Red,     // j3ssy
                3 => Color::Magenta, // a13x
                _ => Color::White,
            };
            ListItem::new(Line::from(Span::styled(entity, Style::default().fg(color))))
        })
        .collect();

    let entities_list =
        List::new(items).block(Block::default().borders(Borders::ALL).title("Entities"));
    frame.render_widget(entities_list, chunks[1]);

    let instructions = Paragraph::new("Press 'q' or 'Esc' to quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(instructions, chunks[2]);
}
