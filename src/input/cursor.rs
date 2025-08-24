use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub fn hide_cursor(mut q_window: Query<&mut Window>) {
    if let Ok(mut window) = q_window.get_single_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

pub fn exit_on_esc(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
