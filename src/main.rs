use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use itertools::Itertools as _;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // sparse path. shows up as expected
    commands.spawn_bundle(build_path(
        [-10., 0., 10.], // x coordinates of the line points
        Color::WHITE,
        Vec2::Y * 10.,
    ));

    // dense path. is invisible
    let start = -10.;
    let end = 10.;

    // lowering this value causes it to render correctly.
    // The magic number seems to be 283: it renders fine at 282, but disappears at 283.
    // this corresponds to a step size of `0.07067138`
    let count = 283;
    let step = (end - start) / count as f32;
    dbg!(step);
    commands.spawn_bundle(build_path(
        (0..count).map(|i| start + step * i as f32), // iterator of floats from `start`..`end`, incrementing by `step`
        Color::BLACK,
        Vec2::Y * -10.,
    ));
}

fn build_path(x_coords: impl IntoIterator<Item = f32>, color: Color, pos: Vec2) -> ShapeBundle {
    // build a path using the specified x coordinates.
    // y is always zero.
    let path = x_coords
        .into_iter()
        .map(|x| Vec2::new(x, 0.0))
        .tuple_windows()
        .fold(PathBuilder::new(), |mut path, (a, b)| {
            dbg!(a);
            path.move_to(a);
            path.line_to(b);
            path
        })
        .build();

    GeometryBuilder::build_as(
        &path,
        DrawMode::Stroke(StrokeMode::new(color, 2.0)),
        Transform {
            translation: pos.extend(0.0),
            ..default()
        },
    )
}
