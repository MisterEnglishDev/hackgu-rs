//! hackgu-ccs: CyberConnect2 CCS/CCSF container parser for .hack//G.U. Last Recode.

pub mod anim;
pub mod clump;
pub mod header;
pub mod hitmesh;
pub mod index;
pub mod material;
pub mod model;
pub mod objects;
pub mod parser;
pub mod texture;

pub use anim::{bind_local_pose, blend_local_poses, nlerp, world_from_local_pose, AnimBoneChannel, AnimError, CcsAnimation, LocalPose, RotationKey, TranslationKey};
pub use clump::{CcsBoneTransform, CcsClump, ClumpError, Mat4};
pub use hitmesh::{HitMesh, HitRecord};
pub use header::{CcsHeader, CcsVersion, HeaderError, CCS_MAGIC};
pub use index::{IndexError, IndexFileEntry, IndexObjectEntry, IndexSection};
pub use material::CcsMaterial;
pub use model::{triangulate_strip, DecodedModel, ModelError, ModelVertex, SubMesh};
pub use objects::{section_name, CcsObjectNode, CcsRawObject};
pub use parser::{CcsError, CcsFile};
pub use texture::{DecodedTexture, TextureError};

