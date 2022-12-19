#![allow(dead_code)]

use glam::Vec2;

use crate::nodes::node::NodeUuid;
use crate::nodes::node_tree::NodeTree;

/// Who is allowed to use the puppet?
#[derive(Clone, Copy, Debug, Default)]
pub enum PuppetAllowedUsers {
    /// Only the author(s) are allowed to use the puppet.
    #[default]
    OnlyAuthor,
    /// Only licensee(s) are allowed to use the puppet.
    OnlyLicensee,
    /// Everyone may use the model.
    Everyone,
}

/// Can the puppet be redistributed?
#[derive(Clone, Copy, Debug, Default)]
pub enum PuppetAllowedRedistribution {
    /// Redistribution is prohibited
    #[default]
    Prohibited,
    /// Redistribution is allowed, but only under the same license
    /// as the original.
    ViralLicense,
    /// Redistribution is allowed, and the puppet may be
    /// redistributed under a different license than the original.
    ///
    /// This goes in conjunction with modification rights.
    CopyleftLicense,
}

/// Can the puppet be modified?
#[derive(Clone, Copy, Debug, Default)]
pub enum PuppetAllowedModification {
    /// Modification is prohibited
    #[default]
    Prohibited,
    /// Modification is only allowed for personal use
    AllowPersonal,
    /// Modification is allowed with redistribution, see
    /// `allowed_redistribution` for redistribution terms.
    AllowRedistribute,
}

/// Terms of usage of the puppet.
#[derive(Clone, Debug, Default)]
pub struct PuppetUsageRights {
    /// Who is allowed to use the puppet?
    pub allowed_users: PuppetAllowedUsers,
    /// Whether violence content is allowed.
    pub allow_violence: bool,
    /// Whether sexual content is allowed.
    pub allow_sexual: bool,
    /// Whether commercial use is allowed.
    pub allow_commercial: bool,
    /// Whether a model may be redistributed.
    pub allow_redistribution: PuppetAllowedRedistribution,
    /// Whether a model may be modified.
    pub allow_modification: PuppetAllowedModification,
    /// Whether the author(s) must be attributed for use.
    pub require_attribution: bool,
}

/// Puppet meta information.
#[derive(Clone, Debug)]
pub struct PuppetMeta {
    /// Name of the puppet.
    pub name: Option<String>,
    /// Version of the Inochi2D spec that was used when creating this model.
    pub version: String,
    /// Rigger(s) of the puppet.
    pub rigger: Option<String>,
    /// Artist(s) of the puppet.
    pub artist: Option<String>,
    /// Usage Rights of the puppet.
    pub rights: Option<PuppetUsageRights>,
    /// Copyright string.
    pub copyright: Option<String>,
    /// URL of the license.
    pub license_url: Option<String>,
    /// Contact information of the first author.
    pub contact: Option<String>,
    /// Link to the origin of this puppet.
    pub reference: Option<String>,
    /// Texture ID of this puppet's thumbnail.
    pub thumbnail_id: Option<u32>,
    /// Whether the puppet should preserve pixel borders.
    /// This feature is mainly useful for puppets that use pixel art.
    pub preserve_pixels: bool,
}

impl Default for PuppetMeta {
    fn default() -> Self {
        Self {
            name: Default::default(),
            version: crate::INOCHI2D_SPEC_VERSION.to_owned(),
            rigger: Default::default(),
            artist: Default::default(),
            rights: Default::default(),
            copyright: Default::default(),
            license_url: Default::default(),
            contact: Default::default(),
            reference: Default::default(),
            thumbnail_id: Default::default(),
            preserve_pixels: Default::default(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PuppetPhysics {
    pixels_per_meter: f32,
    gravity: f32,
}

#[derive(Debug)]
pub enum InterpolateMode {
    Linear,
}

#[derive(Debug)]
pub struct BindingBase {
    node: NodeUuid,
    is_set: Vec<Vec<bool>>,
    interpolate_mode: InterpolateMode,
}

#[derive(Debug)]
pub enum Binding {
    ZSort {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformTX {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformTY {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformSX {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformSY {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformRX {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformRY {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    TransformRZ {
        base: BindingBase,
        values: Vec<Vec<f32>>,
    },
    Deform {
        base: BindingBase,
        values: Vec<Vec<Vec<Vec2>>>,
    },
}

#[derive(Debug)]
pub struct Param {
    uuid: u32,
    name: String,
    is_vec2: bool,
    min: Vec2,
    max: Vec2,
    defaults: Vec2,
    axis_points: [Vec<f32>; 2],
    bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Puppet {
    pub meta: PuppetMeta,
    pub physics: PuppetPhysics,
    pub nodes: NodeTree,
    pub parameters: Vec<Param>,
}
