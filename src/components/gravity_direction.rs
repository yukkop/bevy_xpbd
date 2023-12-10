use bevy::prelude::{Deref, DerefMut, Component};

use crate::math::Vector;

/// A component representing the direction of gravity.
/// 
/// This struct holds a [`Vector`] which represents the gravity direction in a 3D space.
#[derive(Clone, Component, Debug, Default, Deref, DerefMut)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct GravityDirection(pub Vector);

impl GravityDirection {
    /// Constructs a new `GravityDirection`.
    pub fn new(vec: impl Into<Vector>) -> Self {
        Self(vec.into())
    }

    /// Constructs a new `GravityDirection`.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 representing the x-component of the gravity direction.
    /// * `y` - A f32 representing the y-component of the gravity direction.
    ///
    /// # Examples
    ///
    /// ```
    /// let gravity = GravityDirection::from_xy(0.0, -1.0, 0.0);
    /// ```
    #[cfg(feature = "2d")]
    pub fn from_xy(x: f32, y: f32) -> Self {
        Self(Vector::new(x, y))
    }

    /// Constructs a new `GravityDirection`.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 representing the x-component of the gravity direction.
    /// * `y` - A f32 representing the y-component of the gravity direction.
    /// * `z` - A f32 representing the z-component of the gravity direction.
    ///
    /// # Examples
    ///
    /// ```
    /// let gravity = GravityDirection::new(0.0, -1.0, 0.0);
    /// ```
    #[cfg(feature = "3d")]
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self(Vector::new(x, y, z))
    }

    /// Sets the gravity direction using a [`Vector`].
    ///
    /// # Arguments
    ///
    /// * `vec` - A [`Vector`] representing the new gravity direction.
    pub fn set(&mut self, vec: impl Into<Vector>) {
        self.0 = vec.into();
    }
    
    /// Sets the individual components of the gravity direction.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 representing the new x-component of the gravity direction.
    /// * `y` - A f32 representing the new y-component of the gravity direction.
    #[cfg(feature = "2d")]
    pub fn set_xy(&mut self, x: f32, y: f32) {
        self.0.x = x;
        self.0.y = y;
    }

    /// Sets the individual components of the gravity direction.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 representing the new x-component of the gravity direction.
    /// * `y` - A f32 representing the new y-component of the gravity direction.
    /// * `z` - A f32 representing the new z-component of the gravity direction.
    #[cfg(feature = "3d")]
    pub fn set_xyz(&mut self, x: f32, y: f32, z: f32) {
        self.0.x = x;
        self.0.y = y;
        self.0.z = z;
    }

    /// Sets the x-component of the gravity direction.
    ///
    /// # Arguments
    ///
    /// * `x` - A f32 representing the new x-component of the gravity direc
    pub fn set_x(&mut self, x: f32) {
        self.0.x = x;
    }

    /// Sets the x-component of the gravity direction.
    ///
    /// # Arguments
    ///
    /// * `y` - A f32 representing the new y-component of the gravity direc
    pub fn set_y(&mut self, y: f32) {
        self.0.y = y;
    }

    /// Sets the x-component of the gravity direction.
    ///
    /// # Arguments
    ///
    /// * `z` - A f32 representing the new z-component of the gravity direc
    #[cfg(feature = "3d")]
    pub fn set_z(&mut self, z: f32) {
        self.0.z = z;
    }
}