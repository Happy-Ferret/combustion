//! Routines for converting Assimp structures to Combustion structures

use nalgebra::Vector3;

use assimp::{self, Named};

use protocols::math::data::Transform;
use protocols::mesh::protocol::MeshPrimitive;
use protocols::mesh::data::{Mesh, MeshVertices, Vertices, TexCoord};
use protocols::model::data::{Model, Node};

use ::error::{AssetResult, AssetError};

/// Converts an Assimp `Scene` into a Combustion `Model`
pub fn scene_to_model(scene: assimp::Scene) -> AssetResult<Model> {
    let raw_meshes = try_throw!(scene.meshes().ok_or(AssetError::UnsupportedFormat));

    let mut meshes = Vec::new();

    for raw_mesh in raw_meshes {
        meshes.push(assimp_mesh_to_mesh(raw_mesh)?);
    }

    let root = try_rethrow!(assimp_node_to_node(scene.root()));

    Ok(Model {
        meshes: meshes,
        root: root,
        materials: Vec::new(),
    })
}

fn assimp_mesh_to_mesh(mesh: assimp::Mesh) -> AssetResult<Mesh> {
    let vertices = MeshVertices::Discrete({
        let raw_positions = try_throw!(mesh.vertices().ok_or(AssetError::UnsupportedFormat));

        Vertices {
            positions: raw_positions.iter().map(|pos| Vector3::from(*pos).to_point()).collect(),
            normals: mesh.normals().map(|normals| {
                normals.iter().map(|normal| Vector3::from(*normal)).collect()
            }),
            uvs: mesh.uv_channel(0).map(|(_, uvs)| {
                uvs.iter().map(|uv| TexCoord::new(uv.x, uv.y)).collect()
            })
        }
    });

    let indices = mesh.indices().map(|indices| {
        indices.into_iter().map(|index| index.into()).collect()
    });

    Ok(Mesh {
        vertices: vertices,
        indices: indices,
        materials: Vec::new(),
        primitive: MeshPrimitive::Triangles,
    })
}

fn assimp_node_to_node(node: assimp::Node) -> AssetResult<Node> {
    let mut children = Vec::new();

    // Convert and collect children if there are any
    if let Some(raw_children) = node.children() {
        for child_node in raw_children {
            children.push(assimp_node_to_node(child_node)?);
        }
    }

    Ok(Node {
        // Take the node name and convert it into a String
        name: node.name().to_string(),
        // Take all the mesh indices (as c_uints), clone them, and convert them into u32
        meshes: node.meshes().map(|mesh_indices| {
            mesh_indices.iter().cloned().map(From::from).collect()
        }).unwrap_or_else(Vec::new),
        // Create a single-element Vec with the converted node transform
        transforms: vec![Transform::Matrix(node.transformation().clone().into())],
        children: children,
    })
}