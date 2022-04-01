#![windows_subsystem = "windows"]
use std::cmp;

use bevy::prelude::*;
use symbol::{
    update_symbols, CurrentSymbol, Symbol, SymbolIndex, SymbolPlugin, Symbols, SymbolsMaterials,
    SYMBOL_SIZE,
};
use ui::{update_texts, TextElement, UIPlugin};

mod button;
mod symbol;
mod ui;

struct MainCamera;

#[derive(Debug, Clone, Copy)]
pub enum WinningEvent {
    X,
    O,
    Draw,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Menu,
    Game,
}

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "Tic-Tac-Toe".into(),
            width: 600.0,
            height: 600.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb_u8(40, 44, 52)))
        .add_state(AppState::Menu)
        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(SymbolPlugin)
        .add_startup_system(setup)
        .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_game))
        .add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(solution_detection_system)
                .with_system(handle_winning_events_system)
                .with_system(mouse_input_system),
        )
        .add_event::<WinningEvent>()
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_game(
    mut commands: Commands,
    symbols_query: Query<Entity, With<Symbol>>,
    symbols_materials: Res<SymbolsMaterials>,
    symbols: ResMut<Symbols>,
) {
    update_symbols(&mut commands, &symbols_query, &symbols_materials, &symbols);
}

fn mouse_input_system(
    mut commands: Commands,
    windows: Res<Windows>,
    mouse_button_input: Res<Input<MouseButton>>,
    camera_query: Query<&Transform, With<MainCamera>>,
    query: Query<(&GlobalTransform, &SymbolIndex), With<Symbol>>,
    symbols_query: Query<Entity, With<Symbol>>,
    symbols_materials: Res<SymbolsMaterials>,
    mut text_query: Query<(Entity, &mut TextElement)>,
    mut current_symbol: ResMut<CurrentSymbol>,
    mut symbols: ResMut<Symbols>,
) {
    if current_symbol.0 != Symbol::Empty {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            for (symbol_global_transform, symbol_index) in query.iter() {
                let window = windows.get_primary().unwrap();

                if let Some(cursor_pos) = window.cursor_position() {
                    let window_size = Vec2::new(window.width(), window.height());
                    let p = cursor_pos - window_size / 2.0;

                    let camera_transform = camera_query.single().unwrap();
                    let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

                    let position = symbol_global_transform.translation;
                    let ui_position = Vec2::new(position.x, position.y);
                    let extents = Vec2::splat(SYMBOL_SIZE) / 2.0;
                    let min = ui_position - extents;
                    let max = ui_position + extents;

                    // if the current cursor position is within the bounds of the node, consider it for clicking
                    if (min.x..max.x).contains(&world_pos.x)
                        && (min.y..max.y).contains(&world_pos.y)
                    {
                        let symbol = &mut symbols.0[symbol_index.0];

                        if symbol != &Symbol::Empty {
                            return;
                        }

                        *symbol = current_symbol.0;

                        current_symbol.0 = match current_symbol.0 {
                            Symbol::X => Symbol::O,
                            Symbol::O => Symbol::X,
                            Symbol::Empty => unreachable!(),
                        };

                        update_texts(&mut text_query, &current_symbol, false);
                        update_symbols(&mut commands, &symbols_query, &symbols_materials, &symbols);
                    }
                }
            }
        }
    }
}

fn solution_detection_system(symbols: Res<Symbols>, mut winning_events: EventWriter<WinningEvent>) {
    if !symbols.is_changed() {
        return;
    }
    /*
    012
    345
    678
    */
    let solutions = [
        // Lines
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        // Rows
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        // Diagonals
        [0, 4, 8],
        [6, 4, 2],
    ];

    let (mut x, mut o) = (0, 0);

    solutions.iter().for_each(|solution| {
        let (solution_x, solution_o) =
            solution
                .iter()
                .map(|case| symbols.0[*case])
                .fold((0, 0), |acc, x| match x {
                    Symbol::X => (acc.0 + 1, acc.1),
                    Symbol::O => (acc.0, acc.1 + 1),
                    Symbol::Empty => acc,
                });

        x = cmp::max(solution_x, x);
        o = cmp::max(solution_o, o);
    });

    let empties = symbols
        .0
        .iter()
        .fold(0, |acc, s| acc + (if s == &Symbol::Empty { 1 } else { 0 }));

    winning_events.send(match (x, o, empties) {
        (3, 3, _) => WinningEvent::Draw,
        (3, _, _) => WinningEvent::X,
        (_, 3, _) => WinningEvent::O,
        (_, _, 0) => WinningEvent::Draw,
        _ => return,
    });
}

fn handle_winning_events_system(
    mut current_symbol: ResMut<CurrentSymbol>,
    mut winning_events: EventReader<WinningEvent>,
    mut text_query: Query<&mut TextElement, With<Text>>,
) {
    for event in winning_events.iter() {
        (*current_symbol).0 = Symbol::Empty;

        for mut text_element in text_query.iter_mut() {
            match *text_element {
                TextElement::Winner(ref mut winner) => *winner = Some(*event),
                _ => {}
            }
        }
    }
}
