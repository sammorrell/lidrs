#[derive(Debug, Clone, PartialEq)]
pub enum IesLuminousOpening {
    Point,
    Rectangular {
        width: f64,
        length: f64,
    },
    RectanguarLuminousSides {
        width: f64,
        length: f64,
        height: f64,
    },
    Circular {
        diameter: f64,
    },
    Ellipse {
        width: f64,
        length: f64,
    },
    VerticalCylinder {
        diameter: f64,
        height: f64,
    },
    VerticalEllipsoidalCylinder {
        width: f64,
        length: f64,
        height: f64,
    },
    Sphere {
        diameter: f64,
    },
    EllipsoidalSpheroid {
        width: f64,
        length: f64,
        height: f64,
    },
    HorizontalCylinderAlong {
        diameter: f64,
        length: f64,
    },
    HorizontalEllipsoidalCylinderAlong {
        width: f64,
        length: f64,
        height: f64,
    },
    HorizontalCylinderPerpendicular {
        width: f64,
        diameter: f64,
    },
    HorizontalEllipsoidalCylinderPerpendicular {
        width: f64,
        length: f64,
        height: f64,
    },
    VerticalCircle {
        diameter: f64,
    },
    VerticalEllipse {
        width: f64,
        height: f64,
    },
}

impl IesLuminousOpening {
    // Get the type and properties of the luminous opening from the supplied width, lenght and height.
    pub fn from_dimensions(width: f64, length: f64, height: f64) -> Self {
        // Check for the easy case first - point source.
        if width == 0. && length == 0. && height == 0. {
            return Self::Point;
        };

        // Now make a decision tree selecting down properties of the different types.
        if width < 0.0 {
            if length == 0. {
                if width == height {
                    Self::VerticalCircle { diameter: -width }
                } else {
                    Self::VerticalEllipse {
                        width: -width,
                        height: -height,
                    }
                }
            } else {
                if length < 0.0 {
                    if height == 0. {
                        if width == length {
                            Self::Circular { diameter: -width }
                        } else {
                            Self::Ellipse {
                                width: -width,
                                length: -length,
                            }
                        }
                    } else {
                        if height < 0. {
                            if width == length && length == height {
                                Self::Sphere { diameter: -width }
                            } else {
                                Self::EllipsoidalSpheroid {
                                    width: -width,
                                    length: -length,
                                    height: -height,
                                }
                            }
                        } else {
                            if width == length {
                                Self::VerticalCylinder {
                                    diameter: -width,
                                    height,
                                }
                            } else {
                                Self::VerticalEllipsoidalCylinder {
                                    width: -width,
                                    length: -length,
                                    height,
                                }
                            }
                        }
                    }
                } else {
                    if width == height {
                        Self::HorizontalCylinderAlong {
                            diameter: -width,
                            length,
                        }
                    } else {
                        Self::HorizontalEllipsoidalCylinderAlong {
                            width: -width,
                            length,
                            height: -height,
                        }
                    }
                }
            }
        } else {
            if length < 0. {
                if length == height {
                    Self::HorizontalCylinderPerpendicular {
                        width,
                        diameter: -length,
                    }
                } else {
                    Self::HorizontalEllipsoidalCylinderPerpendicular {
                        width,
                        length: -length,
                        height: -height,
                    }
                }
            } else {
                if height == 0. {
                    Self::Rectangular { width, length }
                } else {
                    Self::RectanguarLuminousSides {
                        width,
                        length,
                        height,
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IesLuminousOpening;

    #[test]
    /// In this test we will run through each case in turn and check that we get the correct result.
    fn test_from_dimensions() {
        // Point
        assert_eq!(
            IesLuminousOpening::from_dimensions(0.0, 0.0, 0.0),
            IesLuminousOpening::Point
        );

        // Rectangular
        assert_eq!(
            IesLuminousOpening::from_dimensions(1.0, 1.0, 0.0),
            IesLuminousOpening::Rectangular {
                width: 1.0,
                length: 1.0
            }
        );

        // Rectangular with luminous sides.
        assert_eq!(
            IesLuminousOpening::from_dimensions(1.0, 1.0, 1.0),
            IesLuminousOpening::RectanguarLuminousSides {
                width: 1.0,
                length: 1.0,
                height: 1.0
            }
        );

        // Circular
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -1.0, 0.0),
            IesLuminousOpening::Circular { diameter: 1.0 }
        );

        // Ellipse
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -2.0, 0.0),
            IesLuminousOpening::Ellipse {
                width: 1.0,
                length: 2.0
            }
        );

        // Vertical Cylinder
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -1.0, 1.0),
            IesLuminousOpening::VerticalCylinder {
                diameter: 1.0,
                height: 1.0
            }
        );

        // Vertical Ellipsoidal Cylinder
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -2.0, 1.0),
            IesLuminousOpening::VerticalEllipsoidalCylinder {
                width: 1.0,
                length: 2.0,
                height: 1.0
            }
        );

        // Sphere
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -1.0, -1.0),
            IesLuminousOpening::Sphere { diameter: 1.0 }
        );

        // Ellipsoidal Spheroid
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, -1.0, -2.0),
            IesLuminousOpening::EllipsoidalSpheroid {
                width: 1.0,
                length: 1.0,
                height: 2.0
            }
        );
        assert_eq!(
            IesLuminousOpening::from_dimensions(-3.0, -2.0, -1.0),
            IesLuminousOpening::EllipsoidalSpheroid {
                width: 3.0,
                length: 2.0,
                height: 1.0
            }
        );

        // Horizontal Cylinder along Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, 1.0, -1.0),
            IesLuminousOpening::HorizontalCylinderAlong {
                diameter: 1.0,
                length: 1.0
            }
        );

        // Horizontal Ellipsoidal Cylinder Along Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, 1.0, -2.0),
            IesLuminousOpening::HorizontalEllipsoidalCylinderAlong {
                width: 1.0,
                length: 1.0,
                height: 2.0
            }
        );

        // Horizontal Cylinder Perpendicular to Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(1.0, -1.0, -1.0),
            IesLuminousOpening::HorizontalCylinderPerpendicular {
                width: 1.0,
                diameter: 1.0
            }
        );

        // Horizontal Ellipsoidal Cylinder Perpendicular to Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(1.0, -1.0, -2.0),
            IesLuminousOpening::HorizontalEllipsoidalCylinderPerpendicular {
                width: 1.0,
                length: 1.0,
                height: 2.0
            }
        );

        // Vertical Circle Facing Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, 0.0, -1.0),
            IesLuminousOpening::VerticalCircle { diameter: 1.0 }
        );

        // Vertical Ellipse Facing Photometric Horizontal
        assert_eq!(
            IesLuminousOpening::from_dimensions(-1.0, 0.0, -2.0),
            IesLuminousOpening::VerticalEllipse {
                width: 1.0,
                height: 2.0
            }
        );
    }
}
