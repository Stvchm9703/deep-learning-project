pub mod face_description;
pub mod face_hair_style;

// pub use face_description::*;
// pub use face_hair_style::*; // import FaceHairStyle struct and constants // import FaceShape struct and constants

pub mod face_result_set {
    use super::face_hair_style::*;
    use crate::data::face_description::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct FaceResultSet {
        pub title: &'static str,
        pub cover_img: &'static str,
        pub description: &'static str,
    }

    pub const FACE_RESULT_SQUARE: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_SQUARE,
        description: FACE_SHAPE_SQUARE_DESCRIPTION,
        cover_img: "./public/sample-woman-square.jpg",
    };

    pub fn get_hair_styles_square() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_TEXTURED_BOB,
            HAIR_STYLE_PIXIE_CUT,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_LAYERED_LOB,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_WAVY_SHAG,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_A_LINE_BOB,
            HAIR_STYLE_SIDE_PART,
            HAIR_STYLE_LOOSE_UPDO,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_SOFT_PONYTAIL,
            HAIR_STYLE_FACE_CURTAIN_BANGS,
            HAIR_STYLE_FACE_FEATHERED_BANGS,
        ]
    }

    pub const FACE_RESULT_OBLONG: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_OBLONG,
        description: FACE_SHAPE_OBLONG_DESCRIPTION,
        cover_img: "./public/sample-woman-oblong.jpg",
    };

    pub fn get_hair_styles_oblong() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_CHIN_LENGTH_BOB,
            HAIR_STYLE_TEXTURED_BOB,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_LAYERED_LOB,
            HAIR_STYLE_SHAGGY_LOB,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_LOOSE_WAVES,
            HAIR_STYLE_VOLUMINOUS_CURLS,
            HAIR_STYLE_LOOSE_UPDO,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_LOOSE_PONYTAIL,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_BLUNT_BANGS,
            HAIR_STYLE_WISPY_BANGS,
        ]
    }

    pub const FACE_RESULT_ROUND: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_ROUND,
        description: FACE_SHAPE_ROUND_DESCRIPTION,
        cover_img: "",
    };

    pub fn get_hair_styles_round() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_PIXIE_CUT_WITH_VOLUME,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_TEXTURED_CROP,
            HAIR_STYLE_LONG_BOB,
            HAIR_STYLE_SHAGGY_LOB,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_LAYERED_CUT,
            HAIR_STYLE_STRAIGHT_AND_SLEEK,
            HAIR_STYLE_BEACH_WAVES,
            HAIR_STYLE_HIGH_PONYTAIL,
            HAIR_STYLE_TOP_KNOT,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_SIDE_BANGS,
            HAIR_STYLE_FEATHERED_BANGS,
        ]
    }

    pub const FACE_RESULT_HEART: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_HEART,
        description: FACE_SHAPE_HEART_DESCRIPTION,
        cover_img: "./public/sample-woman-heart.jpg",
    };

    pub fn get_hair_styles_heart() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_TEXTURED_PIXIE_CUT,
            HAIR_STYLE_SIDE_SWEPT_PIXIE,
            HAIR_STYLE_CHIN_LENGTH_BOB,
            HAIR_STYLE_LAYERED_LOB,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_WAVY_SHAG,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_LOOSE_WAVES,
            HAIR_STYLE_SIDE_PART,
            HAIR_STYLE_LOW_BUN,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_LOOSE_PONYTAIL,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_SIDE_BANGS,
            HAIR_STYLE_WISPY_BANGS,
        ]
    }

    pub const FACE_RESULT_RECTANGULAR: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_RECTANGULAR,
        description: FACE_SHAPE_RECTANGULAR_DESCRIPTION,
        cover_img: "./public/sample-man-rectangular.jpg",
    };
    pub fn get_hair_styles_rectangular() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_TEXTURED_BOB,
            HAIR_STYLE_PIXIE_CUT_WITH_BANGS,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_LAYERED_LOB,
            HAIR_STYLE_WAVY_SHAG,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_LOOSE_WAVES,
            HAIR_STYLE_VOLUMINOUS_CURLS,
            HAIR_STYLE_SOFT_CHIGNON,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_LOOSE_PONYTAIL,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_BLUNT_BANGS,
            HAIR_STYLE_WISPY_BANGS,
        ]
    }

    pub const FACE_RESULT_OVAL: FaceResultSet = FaceResultSet {
        title: FACE_SHAPE_OVAL,
        description: FACE_SHAPE_OVAL_DESCRIPTION,
        cover_img: "./public/sample-woman-oval.jpg",
    };

    pub fn get_hair_styles_oval() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_CLASSIC_BOB,
            HAIR_STYLE_PIXIE_CUT,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_LONG_BOB,
            HAIR_STYLE_LAYERED_SHAG,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_BEACH_WAVES,
            HAIR_STYLE_STRAIGHT_AND_SLEEK,
            HAIR_STYLE_HIGH_PONYTAIL,
            HAIR_STYLE_TOP_KNOT,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_BLUNT_BANGS,
            HAIR_STYLE_WISPY_BANGS,
        ]
    }

    pub fn get_hair_styles(face_shape: &str) -> Vec<&'static FaceHairStyle> {
        match face_shape {
            FACE_SHAPE_OBLONG => get_hair_styles_oblong(),
            FACE_SHAPE_ROUND => get_hair_styles_round(),
            FACE_SHAPE_HEART => get_hair_styles_heart(),
            FACE_SHAPE_RECTANGULAR => get_hair_styles_rectangular(),
            FACE_SHAPE_OVAL => get_hair_styles_oval(),
            _ => vec![],
        }
    }

    pub fn get_face_result(face_shape: &str) -> FaceResultSet {
        match face_shape {
            FACE_SHAPE_OBLONG => FACE_RESULT_OBLONG,
            FACE_SHAPE_ROUND => FACE_RESULT_ROUND,
            FACE_SHAPE_HEART => FACE_RESULT_HEART,
            FACE_SHAPE_RECTANGULAR => FACE_RESULT_RECTANGULAR,
            FACE_SHAPE_OVAL => FACE_RESULT_OVAL,
            _ => FACE_RESULT_OBLONG,
        }
    }

    pub fn get_all_hair_style() -> Vec<&'static FaceHairStyle> {
        vec![
            HAIR_STYLE_TEXTURED_BOB,
            HAIR_STYLE_PIXIE_CUT,
            HAIR_STYLE_ASYMMETRICAL_BOB,
            HAIR_STYLE_LAYERED_LOB,
            HAIR_STYLE_SIDE_SWEPT_BANGS,
            HAIR_STYLE_WAVY_SHAG,
            HAIR_STYLE_LONG_LAYERS,
            HAIR_STYLE_A_LINE_BOB,
            HAIR_STYLE_SIDE_PART,
            HAIR_STYLE_LOOSE_UPDO,
            HAIR_STYLE_HALF_UP_HALF_DOWN,
            HAIR_STYLE_SOFT_PONYTAIL,
            HAIR_STYLE_FACE_CURTAIN_BANGS,
            HAIR_STYLE_FACE_FEATHERED_BANGS,
            HAIR_STYLE_CHIN_LENGTH_BOB,
            HAIR_STYLE_SHAGGY_LOB,
            HAIR_STYLE_VOLUMINOUS_CURLS,
            HAIR_STYLE_PIXIE_CUT_WITH_VOLUME,
            HAIR_STYLE_TEXTURED_CROP,
            HAIR_STYLE_LAYERED_CUT,
            HAIR_STYLE_FEATHERED_BANGS,
            HAIR_STYLE_TEXTURED_PIXIE_CUT,
            HAIR_STYLE_SIDE_SWEPT_PIXIE,
            HAIR_STYLE_LOOSE_WAVES,
            HAIR_STYLE_LOW_BUN,
            HAIR_STYLE_LOOSE_PONYTAIL,
            HAIR_STYLE_SIDE_BANGS,
            HAIR_STYLE_CLASSIC_BOB,
            HAIR_STYLE_LONG_BOB,
            HAIR_STYLE_LAYERED_SHAG,
            HAIR_STYLE_BEACH_WAVES,
            HAIR_STYLE_STRAIGHT_AND_SLEEK,
            HAIR_STYLE_HIGH_PONYTAIL,
            HAIR_STYLE_TOP_KNOT,
            HAIR_STYLE_CURTAIN_BANGS,
            HAIR_STYLE_BLUNT_BANGS,
            HAIR_STYLE_WISPY_BANGS,
        ]
    }
}
