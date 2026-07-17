use bevy::prelude::*;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButton {
    Resume,
    SaveGame,
    LoadGame,
    MainMenu,
    Quit,
}

pub fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        PauseMenu,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
    )).with_children(|parent| {
        parent.spawn((
            Text::new("PAUSED"),
            TextFont {
                font_size: FontSize::Px(42.0),
                ..default()
            },
            TextColor(Color::WHITE),
            Node {
                margin: UiRect::bottom(Val::Px(30.0)),
                ..default()
            },
        ));
    });

    info!("Pause menu spawned");
}

pub fn despawn_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    info!("Pause menu despawned");
}

pub fn handle_pause_hover(mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<PauseButton>)>) {
    for (interaction, mut bg) in interaction_query.iter_mut() {
        *bg = match interaction {
            Interaction::Hovered => BackgroundColor(Color::srgba(0.25, 0.25, 0.5, 0.9)),
            Interaction::Pressed => BackgroundColor(Color::srgba(0.35, 0.35, 0.6, 0.9)),
            Interaction::None => BackgroundColor(Color::srgba(0.15, 0.15, 0.3, 0.9)),
        };
    }
}