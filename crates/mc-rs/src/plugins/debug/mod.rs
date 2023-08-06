use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

/// A plugin with a debug display
///
/// This plugin adds a debug display to the game, which shows the current FPS and entity count.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, DebugPlugin::create_display);

        app.add_systems(
            Update,
            (DebugPlugin::update_fps, DebugPlugin::update_entities),
        );
    }
}

/// A marker component for the debug display
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct DebugDisplay;

/// A marker component for the debug fps display
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct DebugFpsDisplay;

/// A marker component for the debug entity count display
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Component)]
pub struct DebugEntityCounter;

impl DebugPlugin {
    fn create_display(mut commands: Commands) {
        let style = Style {
            align_self: AlignSelf::End,
            top: Val::Px(2.0),
            right: Val::Px(2.0),
            ..Default::default()
        };

        let text_style = TextStyle {
            font_size: 16.0,
            color: Color::WHITE,
            ..Default::default()
        };

        commands
            .spawn((
                DebugDisplay,
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        position_type: PositionType::Absolute,
                        top: Val::Px(0.0),
                        right: Val::Px(0.0),
                        ..Default::default()
                    },
                    visibility: Visibility::Visible,
                    z_index: ZIndex::Global(i32::MAX),
                    ..Default::default()
                },
            ))
            .with_children(|parent| {
                parent.spawn((
                    DebugFpsDisplay,
                    TextBundle {
                        style: style.clone(),
                        text: Text::from_section("0.0 FPS", text_style.clone()),
                        ..Default::default()
                    },
                ));
                parent.spawn((
                    DebugEntityCounter,
                    TextBundle {
                        style,
                        text: Text::from_section("0 ENT", text_style),
                        ..Default::default()
                    },
                ));
            });
    }

    /// Update the debug fps display
    fn update_fps(
        mut query: Query<&mut Text, (With<DebugFpsDisplay>, Without<DebugEntityCounter>)>,
        diag: Res<DiagnosticsStore>,
    ) {
        if let Some(diag) = diag.get(FrameTimeDiagnosticsPlugin::FPS).unwrap().average() {
            query.single_mut().sections[0].value = format!("{:.1} FPS", diag);
        }
    }

    /// Update the debug entity counter
    fn update_entities(
        mut query: Query<&mut Text, (With<DebugEntityCounter>, Without<DebugFpsDisplay>)>,
        count: Query<()>,
    ) {
        query.single_mut().sections[0].value = format!("{} ENT", count.iter().count());
    }
}
