use bevy::prelude::*;

// Spline resource 
#[derive(Resource, Default)]
pub struct SplineDraft {
    pub points: Vec<Vec3>,
}

const SPLINE_SAMPLES: usize = 100;
const SPLINE_COLOR: Color = Color::srgb(1., 0., 0.);

pub struct SplinePlugin;

impl Plugin for SplinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SplineDraft>()
            .add_systems(Update, draw_spline);
    }
}

fn build_curve(points: &[Vec3]) -> Option<CubicCurve<Vec3>> {
    CubicCardinalSpline::new_catmull_rom(points.iter().copied()).to_curve().ok()
}

fn draw_spline(spline: Res<SplineDraft>, mut gizmos: Gizmos) {
    // Draw points
    for p in spline.points.iter() {
        gizmos.sphere(Isometry3d::new(
            p + Dir3::Y * 0.1, 
            Quat::default()), 
            0.2, 
            Color::WHITE
        );
    }

    // Draw curve
    let Some(curve) = build_curve(&spline.points) else {
        return;
    };
    gizmos.linestrip(curve.iter_positions(SPLINE_SAMPLES), SPLINE_COLOR);
}