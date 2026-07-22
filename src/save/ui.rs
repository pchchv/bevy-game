use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SaveLoadMode {
    Save,
    Load,
}

#[derive(Resource, Default)]
pub struct PendingSaveLoadAction(pub Option<(SaveLoadMode, usize)>);

#[derive(Component)]
pub struct SaveLoadUI;

#[derive(Resource)]
pub struct SaveLoadUIState {
    pub active: bool,
    pub mode: SaveLoadMode,
}

impl Default for SaveLoadUIState {
    fn default() -> Self {
        Self {
            active: false,
            mode: SaveLoadMode::Save,
        }
    }
}

pub fn handle_save_load_ui(mut commands: Commands, ui_state: Res<SaveLoadUIState>, existing_ui: Query<Entity, With<SaveLoadUI>>) {
    if !ui_state.is_changed() {
        return;
    }

    for entity in existing_ui.iter() {
        commands.entity(entity).despawn();
    }

    if !ui_state.active {
        return;
    }

    let title = match ui_state.mode {
        SaveLoadMode::Save => "SAVE GAME",
        SaveLoadMode::Load => "LOAD GAME",
    };
    let mut slot_infos: Vec<Option<SaveMetadata>> = Vec::new();
    for slot in 0..MAX_SLOTS {
        slot_infos.push(systems::load_slot_metadata(slot));
    }

    commands
        .spawn((
            SaveLoadUI,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgba(0.05, 0.05, 0.1, 1.0)),
            GlobalZIndex(100),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(title),
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

            for slot in 0..MAX_SLOTS {
                let info = &slot_infos[slot];
                let label = match info {
                    Some(meta) => format!("Slot {} — {}", slot + 1, meta.timestamp,),
                    None => format!("Slot {} — Empty", slot + 1),
                };
                let is_empty = info.is_none();
                let is_load_mode = ui_state.mode == SaveLoadMode::Load;
                let disabled = is_load_mode && is_empty;
                let bg_color = if disabled {
                    Color::srgba(0.2, 0.2, 0.2, 0.5)
                } else {
                    Color::srgba(0.15, 0.15, 0.3, 0.9)
                };

                let text_color = if disabled {
                    Color::srgba(0.5, 0.5, 0.5, 1.0)
                } else {
                    Color::WHITE
                };

                let mut btn = parent.spawn((
                    SlotButton(slot),
                    Button,
                    Node {
                        width: Val::Px(500.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(5.0)),
                        ..default()
                    },
                    BackgroundColor(bg_color),
                ));

                if disabled {
                    btn.remove::<Button>();
                }

                btn.with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new(label),
                        TextFont {
                            font_size: FontSize::Px(20.0),
                            ..default()
                        },
                        TextColor(text_color),
                    ));
                });
            }

            parent
                .spawn((
                    BackButton,
                    Button,
                    Node {
                        width: Val::Px(200.0),
                        height: Val::Px(45.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.4, 0.1, 0.1, 0.9)),
                ))
                .with_children(|btn_parent| {
                    btn_parent.spawn((
                        Text::new("Back"),
                        TextFont {
                            font_size: FontSize::Px(24.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn execute_save(
    mut pending: ResMut<PendingSaveLoadAction>,
    tile_query: Query<(&Transform, &Sprite, &TileMarker, Option<&Pickable>)>,
    player_query: Query<
        (&Transform, &Health, &PlayerCombat, &CharacterEntry, &Facing),
        With<Player>,
    >,
    enemy_query: Query<(&Transform, &Health, &CharacterEntry, &Facing), With<Enemy>>,
    inventory: Res<Inventory>,
    character_index: Res<CurrentCharacterIndex>,
) {
    let Some((SaveLoadMode::Save, slot)) = pending.0 else {
        return;
    };
    pending.0 = None;
    let Ok((player_tf, player_health, player_combat, player_entry, player_facing)) =
        player_query.single()
    else {
        error!("No player found for save");
        return;
    };
    let player_save = PlayerSave {
        position: [
            player_tf.translation.x,
            player_tf.translation.y,
            player_tf.translation.z,
        ],
        health_current: player_health.current,
        health_max: player_health.max,
        power_type: player_combat.power_type,
        character_name: player_entry.name.clone(),
        character_index: character_index.index,
        facing: *player_facing,
    };
    let mut enemies = Vec::new();
    for (tf, health, entry, facing) in enemy_query.iter() {
        enemies.push(EnemySave {
            position: [tf.translation.x, tf.translation.y, tf.translation.z],
            health_current: health.current,
            health_max: health.max,
            character_name: entry.name.clone(),
            power_type: crate::combat::PowerType::Fire,
            facing: *facing,
        });
    }

    let mut tiles = Vec::new();
    for (tf, sprite, tile_marker, pickable) in tile_query.iter() {
        let atlas_index = sprite.texture_atlas.as_ref().map(|a| a.index).unwrap_or(0);
        let rot = tf.rotation;
        tiles.push(TileSave {
            position: [tf.translation.x, tf.translation.y, tf.translation.z],
            rotation: [rot.x, rot.y, rot.z, rot.w],
            scale: [tf.scale.x, tf.scale.y, tf.scale.z],
            atlas_index,
            tile_type: tile_marker.tile_type,
            pickable: pickable.map(|p| p.kind),
        });
    }

    let timestamp = chrono::Local::now().format("%d %b %Y, %I:%M %p").to_string();
    let save_data = SaveData {
        version: SAVE_VERSION,
        timestamp: timestamp.clone(),
        slot_name: format!("Slot {}", slot + 1),
        player: player_save,
        enemies,
        inventory: inventory.items().clone(),
        tiles,
    };

    match do_write_save(slot, &save_data, &timestamp) {
        Ok(()) => info!("Saved to slot {}", slot + 1),
        Err(e) => error!("Failed to save: {}", e),
    }
}