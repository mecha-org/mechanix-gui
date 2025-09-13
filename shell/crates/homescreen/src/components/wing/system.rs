use bevy::{
    math::Affine2,
    prelude::*,
    render::{mesh::MeshAabb, render_resource::Extent3d},
};

use crate::HomescreenSettings;

use super::{Wing, WingMesh, WingMeshBuilder};

#[derive(Component)]
pub struct WingConfigured;
pub fn on_wing_added(
    mut commands: Commands,
    wings: Query<(Entity, &Wing), Without<WingConfigured>>,
    settings: Res<HomescreenSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, wing) in &wings {
        let transform = Transform::from_xyz(wing.position.x, wing.position.y, wing.z_index);
        let mut wing_mesh = WingMeshBuilder::create_empty();

        wing_mesh.border_radius(settings.size.x * 0.025);

        wing_mesh.height(wing.size.y);
        wing_mesh.width(wing.size.x);

        wing_mesh.upper_wing_height(if wing.upper_wing > 0.0 {
            settings.size.y * 0.0625
        } else {
            0.0
        });
        wing_mesh.lower_wing_height(if wing.lower_wing > 0.0 {
            settings.size.y * 0.0625
        } else {
            0.0
        });

        wing_mesh.upper_wing_width(wing.size.x.min(wing.upper_wing));
        wing_mesh.lower_wing_width(wing.size.x.min(wing.lower_wing));
        wing_mesh.texture_handle(wing.texture_handle.clone());

        wing_mesh.mesh_handle(None);

        let mut wing_mesh = wing_mesh.build().unwrap();

        let mesh_handle = meshes.add(wing_mesh.get_mesh());
        wing_mesh.mesh_handle = Some(mesh_handle.clone());

        let material = materials.add(ColorMaterial {
            color: wing.background_color,
            texture: wing.texture_handle.clone(),
            ..Default::default()
        });

        commands.entity(entity).insert((
            transform,
            wing_mesh,
            Mesh2d(mesh_handle),
            MeshMaterial2d(material),
            WingConfigured,
        ));
    }
}

pub fn on_wing_changed(
    mut commands: Commands,
    mut wings: Query<
        (Entity, &Wing, &mut WingMesh, &MeshMaterial2d<ColorMaterial>),
        (With<WingConfigured>),
    >,
    settings: Res<HomescreenSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (entity, wing, mut wing_mesh, material) in &mut wings {
        let prev_wing = wing_mesh.clone();

        wing_mesh.border_radius = settings.size.x * 0.025;

        wing_mesh.height = wing.size.y;
        wing_mesh.width = wing.size.x;

        wing_mesh.upper_wing_height = if wing.upper_wing > 0.0 {
            settings.size.y * 0.0625
        } else {
            0.0
        };
        wing_mesh.lower_wing_height = if wing.lower_wing > 0.0 {
            settings.size.y * 0.0625
        } else {
            0.0
        };

        wing_mesh.upper_wing_width = wing.size.x.min(wing.upper_wing);
        wing_mesh.lower_wing_width = wing.size.x.min(wing.lower_wing);

        let transform = Transform::from_xyz(wing.position.x, wing.position.y, wing.z_index);
        commands.entity(entity).insert((transform,));
        if prev_wing == *wing_mesh {
            continue;
        }

        let mesh = meshes
            .get_mut(
                wing_mesh
                    .mesh_handle
                    .as_ref()
                    .expect("Mesh Handle Not found"),
            )
            .expect("Mesh Not found");

        *mesh = wing_mesh.get_mesh();
        // if let Some(material) = materials.get(material.id()) {
        //     if let Some(image) = material
        //         .texture
        //         .as_ref()
        //         .map(|handle| images.get_mut(handle).unwrap())
        //     {
        //         let size = mesh.compute_aabb().unwrap().max() - mesh.compute_aabb().unwrap().min();
        //         image.resize(Extent3d {
        //             width: size.x as u32,
        //             height: size.y as u32,
        //             depth_or_array_layers: 1,
        //         });
        //     }
        // }
    }
}
