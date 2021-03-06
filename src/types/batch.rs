use crate::utility::unordered_tracker::*;
use cgmath::*;

/// Token to reference a batch with.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatchToken {
    key: Key<BatchToken>,
}

impl BatchToken {
    pub(crate) fn new(key: Key<BatchToken>) -> BatchToken {
        BatchToken {
            key,
        }
    }

    pub(crate) fn key(&self) -> Key<BatchToken> {
        self.key
    }
}

/// Configuration settings for a batch.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatchSettings {
    /// The translation of the batch.
    pub translation: Vector2<f32>,
    /// The zoom level of the batch. This is 1.0 by default, meaning 1 pixel takes up 1x1 pixels on
    /// screen.
    pub scale: f32,
    /// Rotation is measured in turns from [0, 1). Values outside of the range are wrapped into the
    /// range. For example, 1.75 is wrapped into 0.75, -0.4 is wrapped into 0.6.
    pub rotation: f32,
    /// If the renderer should render this batch or not.
    pub visible: bool,
}

impl Default for BatchSettings {
    fn default() -> BatchSettings {
        BatchSettings {
            translation: Vector2::new(0.0, 0.0),
            scale: 1.0,
            rotation: 0.0,
            visible: true,
        }
    }
}

impl BatchSettings {
    /// Creates a new transform matix based on the parameters of the BatchSettings. The transform
    /// matrix is built in this order: Scale * Translation * Rotation.
    pub fn transform_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_scale(self.scale)
            * Matrix4::from_translation(self.translation.extend(0.0))
            * Matrix4::from_angle_z(Rad(std::f32::consts::PI * 2.0 * self.rotation))
    }
}
