use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle},
};

use map_model::{Area, AreaID, AreaType};

use crate::{colors::ColorScheme, mesh_builder::build_mesh_from_polygon};

#[derive(Component)]
struct AreaIdComponent(pub AreaID);

#[derive(Bundle)]
pub struct AreaBundle {
    area_id: AreaIdComponent,

    #[bundle]
    mesh: MaterialMesh2dBundle<ColorMaterial>,
}

impl AreaBundle {
    pub fn new(
        area: &Area,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        color_scheme: &ColorScheme,
    ) -> Self {
        let mesh = build_mesh_from_polygon(area.polygon.clone());

        AreaBundle {
            area_id: AreaIdComponent(area.id),
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                material: materials.add(ColorMaterial::from(match area.area_type {
                    AreaType::Park => color_scheme.grass,
                    AreaType::Water => color_scheme.water,
                    AreaType::Island => color_scheme.map_background,
                    AreaType::StudyArea => color_scheme.study_area,
                })),
                transform: Transform::from_xyz(0., 0., 5.0),
                ..default()
            },
        }
    }
}
