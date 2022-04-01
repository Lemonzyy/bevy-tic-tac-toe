use std::fmt;

use bevy::prelude::*;
use rand::random;

pub const SYMBOL_SIZE: f32 = 64.0;
pub const SPACE_SIZE: f32 = SYMBOL_SIZE / 3.0;

pub struct SymbolPlugin;

impl Plugin for SymbolPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SymbolsMaterials>()
            .init_resource::<Symbols>()
            .init_resource::<CurrentSymbol>();
    }
}

pub struct SymbolsMaterials {
    x: Handle<ColorMaterial>,
    o: Handle<ColorMaterial>,
    empty: Handle<ColorMaterial>,
}

impl FromWorld for SymbolsMaterials {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Couldn't get AssetServer");

        asset_server.watch_for_changes().unwrap();

        let (x, o, empty) = (
            asset_server.load("textures/symbols/x.png"),
            asset_server.load("textures/symbols/o.png"),
            asset_server.load("textures/symbols/empty.png"),
        );

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("Couldn't get ColorMaterial assets");

        Self {
            x: materials.add(x.into()),
            o: materials.add(o.into()),
            empty: materials.add(empty.into()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Symbol {
    X,
    O,
    Empty,
}

pub struct SymbolIndex(pub usize);

impl Default for Symbol {
    fn default() -> Self {
        Self::Empty
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::X => "X",
                Self::O => "O",
                Self::Empty => "Empty",
            }
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CurrentSymbol(pub Symbol);

impl Default for CurrentSymbol {
    fn default() -> Self {
        Self(if random() { Symbol::X } else { Symbol::O })
    }
}

impl fmt::Display for CurrentSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Symbols(pub [Symbol; 9]);

impl Default for Symbols {
    fn default() -> Self {
        Self([Symbol::default(); 9])
    }
}

pub fn update_symbols(
    commands: &mut Commands,
    symbols_query: &Query<Entity, With<Symbol>>,
    symbols_materials: &Res<SymbolsMaterials>,
    symbols: &ResMut<Symbols>,
) {
    for entity in symbols_query.iter() {
        commands.entity(entity).despawn_recursive()
    }

    let mut i = 0;
    for row in -1..=1 {
        for column in -1..=1 {
            let symbol = symbols.0[i];
            let current_material = match symbol {
                Symbol::X => symbols_materials.x.clone(),
                Symbol::O => symbols_materials.o.clone(),
                Symbol::Empty => symbols_materials.empty.clone(),
            };

            (*commands)
                .spawn_bundle(SpriteBundle {
                    material: current_material,
                    transform: Transform::from_translation(Vec3::new(
                        row as f32 * (SYMBOL_SIZE + SPACE_SIZE),
                        column as f32 * (SYMBOL_SIZE + SPACE_SIZE),
                        0.0,
                    )),
                    ..Default::default()
                })
                .insert(symbol)
                .insert(SymbolIndex(i));

            i += 1;
        }
    }
}
