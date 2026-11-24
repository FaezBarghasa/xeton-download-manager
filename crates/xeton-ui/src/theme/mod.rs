pub mod colors;

pub use colors::*;

pub struct Spacing {
    pub xxs: &'static str,
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

pub const SPACING: Spacing = Spacing {
    xxs: "2px",
    xs: "4px",
    sm: "8px",
    md: "12px",
    lg: "16px",
    xl: "24px",
};

pub struct BorderRadius {
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub full: &'static str,
}

pub const RADIUS: BorderRadius = BorderRadius {
    sm: "4px",
    md: "6px",
    lg: "8px",
    full: "9999px",
};
