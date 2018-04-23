pub use collision::{Interpolate, TranslationInterpolate};
//use rhusics_transform::{Interpolate, Pose, TranslationInterpolate};
use rhusics_transform::{Pose};
use amethyst_core::{Transform as ATransform};
use amethyst_core::cgmath::{Basis2, EuclideanSpace, Euler, InnerSpace, Matrix3, Point2, Point3, Quaternion, Rotation2, Vector3};
use std::ops::{Deref,DerefMut};

pub struct Transform(ATransform);

impl Default for Transform {
    fn default() -> Self {
        ATransform::Default()
    }
}

impl Transform {
    fn new() -> Self {
        ATransform::new()
    }
}

impl Deref for Transform {
    type Target = ATransform;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Transform {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Interpolate<f32> for Transform {
    fn interpolate(&self, other: &Transform, amount: f32) -> Self {
        let translation = self.translation.lerp(other.translation, amount);
        Transform {
            translation,
            scale: self.scale.lerp(other.scale, amount),
            rotation: self.rotation.slerp(other.rotation, amount),
        }
    }
}

impl TranslationInterpolate<f32> for Transform {
    fn translation_interpolate(&self, other: &Transform, amount: f32) -> Self {
        Transform {
            translation: self.translation.lerp(other.translation, amount),
            ..*self
        }
    }
}

impl Pose<Point3<f32>, Quaternion<f32>> for Transform {
    fn new(position: Point3<f32>, rotation: Quaternion<f32>) -> Self {
        Transform {
            translation: position.to_vec(),
            rotation,
            ..Default::default()
        }
    }

    fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
    }

    fn set_position(&mut self, position: Point3<f32>) {
        self.translation = position.to_vec();
    }

    fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    fn position(&self) -> Point3<f32> {
        Point3::from_vec(self.translation)
    }
}

fn as_quat(basis: Basis2<f32>) -> Quaternion<f32> {
    let r = basis.as_ref();
    Matrix3::new(r[0][0], r[0][1], 0., r[1][0], r[1][1], 0., 0., 0., 1.).into()
}

impl Pose<Point2<f32>, Basis2<f32>> for Transform {
    fn new(position: Point2<f32>, rotation: Basis2<f32>) -> Self {
        Transform {
            translation: Vector3::new(position.x, position.y, 0.),
            rotation: as_quat(rotation),
            ..Default::default()
        }
    }

    fn set_rotation(&mut self, rotation: Basis2<f32>) {
        self.rotation = as_quat(rotation);
    }

    fn set_position(&mut self, position: Point2<f32>) {
        self.translation = Vector3::new(position.x, position.y, 0.);
    }

    fn rotation(&self) -> Basis2<f32> {
        Rotation2::from_angle(Euler::from(self.rotation).z)
    }

    fn position(&self) -> Point2<f32> {
        Point2::new(self.translation.x, self.translation.y)
    }
}
