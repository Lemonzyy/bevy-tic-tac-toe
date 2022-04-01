use core::fmt;

use crate::{
    button::{ButtonAssets, ButtonElement, SpawnButton},
    symbol::{CurrentSymbol, Symbol, Symbols, SymbolsMaterials},
    update_symbols, AppState, WinningEvent,
};
use bevy::{app::AppExit, prelude::*};

#[derive(Debug)]

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonAssets>()
            .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu))
            .add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_game_menu))
            .add_system(button_color_system)
            .add_system(button_click_system)
            .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(despawn_menu))
            .add_system_set(SystemSet::on_update(AppState::Game).with_system(text_system))
            .add_system_set(SystemSet::on_exit(AppState::Game).with_system(despawn_menu));
    }
}
pub enum TextElement {
    CurrentSymbol(CurrentSymbol),
    Winner(Option<WinningEvent>),
}

impl fmt::Display for TextElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CurrentSymbol(symbol) => write!(f, "Current symbol is {}", symbol),
            Self::Winner(symbol) => write!(
                f,
                "{}",
                match symbol {
                    Some(symbol) => match symbol {
                        WinningEvent::X => "The winner is X!",
                        WinningEvent::O => "The winner is O!",
                        WinningEvent::Draw => "It's a draw!",
                    },
                    None => "",
                }
            ),
        }
    }
}

pub enum NodeElement {
    Root,
    Text,
}

fn setup_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_assets: Res<ButtonAssets>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_wrap: FlexWrap::WrapReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(NodeElement::Root)
        .with_children(|root| {
            root.spawn_button(&button_assets, ButtonElement::StartGame)
                .spawn_button(&button_assets, ButtonElement::QuitGame);
        });
}

fn setup_game_menu(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    current_symbol: Option<Res<CurrentSymbol>>,
    button_assets: Res<ButtonAssets>,
) {
    let none = materials.add(Color::NONE.into());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: none.clone(),
            ..Default::default()
        })
        .insert(NodeElement::Root)
        .with_children(|root| {
            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Default::default()),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                material: none.clone(),
                ..Default::default()
            })
            .insert(NodeElement::Text)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: button_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(16.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TextElement::CurrentSymbol(*current_symbol.unwrap()));
                parent
                    .spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: button_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(16.0)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(TextElement::Winner(None));
            });

            root.spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Default::default()),
                    justify_content: JustifyContent::SpaceBetween,
                    align_items: AlignItems::FlexStart,
                    ..Default::default()
                },
                material: none,
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_button(&button_assets, ButtonElement::RestartGame)
                    .spawn_button(&button_assets, ButtonElement::QuitGame);
            });
        });
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<NodeElement>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive()
    }
}

fn text_system(mut query: Query<(&mut Text, &TextElement)>) {
    for (mut text, text_element) in query.iter_mut() {
        text.sections[0].value = format!("{}", text_element);
    }
}

pub fn update_texts(
    text_query: &mut Query<(Entity, &mut TextElement)>,
    current_symbol: &ResMut<CurrentSymbol>,
    reset: bool,
) {
    for (_, mut text_element) in text_query.iter_mut() {
        match *text_element {
            TextElement::CurrentSymbol(ref mut symbol) => *symbol = **current_symbol,
            TextElement::Winner(ref mut winner) => {
                if reset {
                    *winner = None
                }
            }
        };
    }
}

fn button_color_system(
    button_materials: Res<ButtonAssets>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        *material = match *interaction {
            Interaction::Clicked => button_materials.pressed.clone(),
            Interaction::Hovered => button_materials.hovered.clone(),
            Interaction::None => button_materials.normal.clone(),
        }
    }
}

fn button_click_system(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    interaction_query: Query<(&Interaction, &ButtonElement), (Changed<Interaction>, With<Button>)>,
    texts_query: Query<Entity, With<Symbol>>,
    mut text_query: Query<(Entity, &mut TextElement)>,
    symbols_materials: Res<SymbolsMaterials>,
    mut current_symbol: Option<ResMut<CurrentSymbol>>,
    mut symbols: ResMut<Symbols>,
) {
    for (interaction, button) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                ButtonElement::StartGame => state
                    .set(AppState::Game)
                    .expect("Couldn't enter the Game state"),
                ButtonElement::RestartGame => {
                    *current_symbol
                        .as_deref_mut()
                        .expect("can't get the current symbol to reset it") =
                        CurrentSymbol::default();

                    *symbols = Symbols::default();

                    update_texts(
                        &mut text_query,
                        current_symbol
                            .as_ref()
                            .expect("can't get the current symbol to reset its text"),
                        true,
                    );
                    update_symbols(&mut commands, &texts_query, &symbols_materials, &symbols);
                }
                ButtonElement::QuitGame => app_exit_events.send(AppExit),
            }
        }
    }
}
