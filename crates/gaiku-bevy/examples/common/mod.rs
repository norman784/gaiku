use bevy::{app::AppExit, prelude::*};

pub fn exit_app_system(input: Res<Input<KeyCode>>, mut event: ResMut<Events<AppExit>>) {
  if input.pressed(KeyCode::Escape) {
    event.send(AppExit);
  }
}
