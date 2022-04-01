use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum ButtonElement {
    StartGame,
    RestartGame,
    QuitGame,
}

impl Into<String> for ButtonElement {
    fn into(self) -> String {
        match self {
            Self::StartGame => "Start game",
            Self::RestartGame => "Restart game",
            Self::QuitGame => "Quit game",
        }
        .into()
    }
}

pub struct ButtonAssets {
    pub font: Handle<Font>,
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonAssets {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("Couldn't get ColorMaterial assets");

        let (normal, hovered, pressed) = (
            materials.add(Color::rgb_u8(190, 105, 177).into()),
            materials.add(Color::rgb_u8(220, 124, 217).into()),
            materials.add(Color::rgb_u8(120, 85, 136).into()),
        );

        let asset_server = world
            .get_resource::<AssetServer>()
            .expect("Couldn't get AssetServer");

        ButtonAssets {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            normal,
            hovered,
            pressed,
        }
    }
}

pub trait SpawnButton {
    fn spawn_button(&mut self, button_assets: &ButtonAssets, value: ButtonElement) -> &mut Self;
}

impl SpawnButton for ChildBuilder<'_, '_> {
    fn spawn_button(&mut self, button_assets: &ButtonAssets, value: ButtonElement) -> &mut Self {
        self.spawn_bundle(ButtonBundle {
            style: Style {
                margin: Rect::all(Val::Px(16.0)),
                padding: Rect::all(Val::Px(8.0)), // Add "inner" margin
                justify_content: JustifyContent::Center, // horizontally center child text
                align_items: AlignItems::Center,  // horizontally center child text
                ..Default::default()
            },
            material: button_assets.normal.clone(),
            ..Default::default()
        })
        .insert(value)
        .with_children(|b| {
            b.spawn_bundle(TextBundle {
                text: Text::with_section(
                    value,
                    TextStyle {
                        font: button_assets.font.clone(),
                        font_size: 40.0,
                        color: Color::BLACK,
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        });

        self
    }
}
