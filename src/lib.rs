//! TODO: Docs

extern crate cgmath;
#[macro_use]
extern crate bitflags;

use std::{u32, f32};
use cgmath::Vector3;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod sys;
pub mod device;
pub mod scene;
pub mod triangle_mesh;
pub mod quad_mesh;
pub mod buffer;

pub use device::Device;
pub use scene::Scene;
pub use triangle_mesh::TriangleMesh;
pub use quad_mesh::QuadMesh;
pub use buffer::{Buffer, MappedBuffer, BufferType};

pub type Ray = sys::RTCRay;

impl Ray {
    /// Create a new ray starting at `origin` and heading in direction `dir`
    pub fn new(origin: &Vector3<f32>, dir: &Vector3<f32>) -> Ray {
        sys::RTCRay {
            org: [origin.x, origin.y, origin.z],
            align0: 0.0,
            dir: [dir.x, dir.y, dir.z],
            align1: 0.0,
            tnear: 0.0,
            tfar: f32::INFINITY,
            time: 0.0,
            mask: u32::MAX,
            Ng: [0.0; 3],
            align2: 0.0,
            u: 0.0,
            v: 0.0,
            geomID: u32::MAX,
            primID: u32::MAX,
            instID: u32::MAX,
            __bindgen_padding_0: [0; 3],
        }
    }
    pub fn segment(origin: &Vector3<f32>, dir: &Vector3<f32>,
                   tnear: f32, tfar: f32) -> Ray {
        sys::RTCRay {
            org: [origin.x, origin.y, origin.z],
            align0: 0.0,
            dir: [dir.x, dir.y, dir.z],
            align1: 0.0,
            tnear: tnear,
            tfar: tfar,
            time: 0.0,
            mask: u32::MAX,
            Ng: [0.0; 3],
            align2: 0.0,
            u: 0.0,
            v: 0.0,
            geomID: u32::MAX,
            primID: u32::MAX,
            instID: u32::MAX,
            __bindgen_padding_0: [0; 3],
        }
    }
}

bitflags! {
    pub struct GeometryFlags: u32 {
        const STATIC = 0;
        const DEFORMABLE = 1;
        const DYNAMIC = 2;
    }
}
bitflags! {
    pub struct SceneFlags: u32 {
        const SCENE_STATIC = 0;
        const SCENE_DYNAMIC = 1;
        const SCENE_COMPACT = 256;
        const SCENE_COHERENT = 512;
        const SCENE_INCOHERENT = 1024;
        const SCENE_HIGH_QUALITY = 2048;
        const SCENE_ROBUST = 65536;
    }
}
bitflags! {
    // TODO: Should prefix these
    pub struct AlgorithmFlags: u32 {
        const INTERSECT1 = 1;
        const INTERSECT4 = 2;
        const INTERSECT8 = 4;
        const INTERSECT16 = 8;
        const INTERPOLATE = 16;
        const INTERSECT_STREAM = 32;
    }
}

