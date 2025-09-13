use bevy::prelude::*;
use bevy::{
    math::{Mat2, Vec2},
    render::{
        mesh::{Indices, Mesh},
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};
use derive_builder::Builder;
mod system;

#[derive(Component, Clone, Debug)]
pub struct Wing {
    pub position: Vec2,
    pub size: Vec2,
    pub z_index: f32,

    pub background_color: Color,
    pub border_color: Color,

    pub upper_wing: f32,
    pub lower_wing: f32,

    pub texture_handle: Option<Handle<Image>>,
}

pub struct WingWidgetPlugin;
impl Plugin for WingWidgetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, (system::on_wing_added, system::on_wing_changed));
    }
}

#[derive(Builder, Component, Clone, Debug, PartialEq)]
#[builder(setter(into))]
pub(super) struct WingMesh {
    pub height: f32,
    pub width: f32,
    pub upper_wing_height: f32,
    pub upper_wing_width: f32,
    pub lower_wing_height: f32,
    pub lower_wing_width: f32,
    pub border_radius: f32,
    pub mesh_handle: Option<Handle<Mesh>>,
    pub texture_handle: Option<Handle<Image>>,
}

impl WingMesh {
    pub fn get_mesh(&self) -> Mesh {
        let upper_corners = if self.upper_wing_height > 0.1 && self.upper_wing_width > 0.1 {
            vec![
                Vec2::new(0.0, 0.0),
                Vec2::new(self.upper_wing_width, 0.0),
                Vec2::new(
                    self.upper_wing_width + self.upper_wing_height,
                    -self.upper_wing_height,
                ),
                Vec2::new(self.width, -self.upper_wing_height),
            ]
        } else {
            vec![Vec2::new(0.0, 0.0), Vec2::new(self.width, 0.0)]
        };

        let lower_corners = if self.lower_wing_height > 0.1 && self.lower_wing_width > 0.1 {
            vec![
                Vec2::new(self.width, -self.height),
                Vec2::new(self.width - self.lower_wing_width, -self.height),
                Vec2::new(
                    self.width - self.lower_wing_width - self.lower_wing_height,
                    -self.height + self.lower_wing_height,
                ),
                Vec2::new(0.0, -self.height + self.lower_wing_height),
            ]
        } else {
            vec![
                Vec2::new(self.width, -self.height),
                Vec2::new(0.0, -self.height),
            ]
        };

        let mut corners = vec![];
        corners.extend(upper_corners);
        corners.extend(lower_corners);
        corners
            .iter_mut()
            .for_each(|corner| corner.x = corner.x.clamp(0.0, self.width));
        corners.dedup();

        let filleted_corners = fillet(&corners, self.border_radius, 8);

        let num_vertices = filleted_corners.len() + 1;
        let num_indices = filleted_corners.len() * 3;

        let mut positions = Vec::with_capacity(num_vertices);
        let mut normals = Vec::with_capacity(num_vertices);
        let mut uvs = Vec::with_capacity(num_vertices);
        let mut indices = Vec::with_capacity(num_indices);

        positions.push([self.width / 2.0, -self.height / 2.0, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([self.width / 1000.0, self.height / 1000.0]);
        for (i, v) in filleted_corners.iter().enumerate() {
            let x = v.x;
            let y = v.y;
            positions.push([x, y, 0.0]);
            normals.push([0.0, 0.0, 1.0]);
            uvs.push([2.0 * (x) / 1000.0, 2.0 * (y) / -1000.0]);

            let i = i as u32;
            let num_corners = filleted_corners.len() as u32;
            indices.extend([i + 1, 0, ((i + 1) % num_corners) + 1]);
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_indices(Indices::U32(indices));

        mesh
    }
}

#[derive(Clone, Copy, Debug)]
struct CornerInfo {
    t1: Vec2,
    t2: Vec2,
    center: Vec2,
    effective_radius: f32,
    fillet_clockwise: bool,
}

/// Creates a new shape by rounding the corners of a polygon.
fn fillet(points: &[Vec2], radius: f32, num_segments: u32) -> Vec<Vec2> {
    if points.len() < 3 || radius <= 0.0 || num_segments == 0 {
        return points.to_vec();
    }

    let mut corner_info_list = Vec::with_capacity(points.len());
    const EPSILON: f32 = 1e-6;

    for i in 0..points.len() {
        let p_prev = points[(i + points.len() - 1) % points.len()];
        let p_curr = points[i];
        let p_next = points[(i + 1) % points.len()];

        let v1 = p_prev - p_curr;
        let v2 = p_next - p_curr;

        let v1_len_sq = v1.length_squared();
        let v2_len_sq = v2.length_squared();

        if v1_len_sq < EPSILON || v2_len_sq < EPSILON {
            corner_info_list.push(CornerInfo {
                t1: p_curr,
                t2: p_curr,
                center: p_curr,
                effective_radius: 0.0,
                fillet_clockwise: false,
            });
            continue;
        }

        let u1 = v1 / v1_len_sq.sqrt();
        let u2 = v2 / v2_len_sq.sqrt();
        let dot = u1.dot(u2);

        if (1.0 - dot.abs()) < EPSILON {
            corner_info_list.push(CornerInfo {
                t1: p_curr,
                t2: p_curr,
                center: p_curr,
                effective_radius: 0.0,
                fillet_clockwise: false,
            });
            continue;
        }
        let tan_half_angle = ((1.0 - dot) / (1.0 + dot + EPSILON)).sqrt();
        let dist_to_tangent = (radius / tan_half_angle).abs();

        let max_dist_to_tangent = (v1_len_sq.sqrt().min(v2_len_sq.sqrt())) / 2.0;
        let clamped_dist = dist_to_tangent.min(max_dist_to_tangent);
        let effective_radius = clamped_dist * tan_half_angle;

        let t1 = p_curr + u1 * clamped_dist;
        let t2 = p_curr + u2 * clamped_dist;

        let cross_z = u1.perp_dot(u2);
        let fillet_clockwise = cross_z < 0.0;

        let normal = if fillet_clockwise {
            -u1.perp()
        } else {
            u1.perp()
        };
        let center = t1 + normal * effective_radius;

        corner_info_list.push(CornerInfo {
            t1,
            t2,
            center,
            effective_radius,
            fillet_clockwise,
        });
    }

    let capacity = corner_info_list.iter().fold(0, |acc, c| {
        acc + if c.effective_radius > EPSILON {
            num_segments as usize
        } else {
            1
        }
    });
    let mut output = Vec::with_capacity(capacity);

    for corner in &corner_info_list {
        if corner.effective_radius > EPSILON {
            let start_vec = corner.t1 - corner.center;
            let end_vec = corner.t2 - corner.center;

            let start_angle = start_vec.y.atan2(start_vec.x);
            let mut end_angle = end_vec.y.atan2(end_vec.x);

            if !corner.fillet_clockwise {
                if end_angle > start_angle {
                    end_angle -= std::f32::consts::PI * 2.0;
                }
            } else if end_angle < start_angle {
                end_angle += std::f32::consts::PI * 2.0;
            }

            let total_arc_angle = end_angle - start_angle;
            let delta_angle = total_arc_angle / num_segments as f32;

            let rot_mat = Mat2::from_angle(delta_angle);
            let mut current_vec = start_vec;

            for _ in 0..num_segments {
                output.push(corner.center + current_vec);
                current_vec = rot_mat * current_vec;
            }
        } else {
            output.push(corner.t1);
        }
    }
    output
}
