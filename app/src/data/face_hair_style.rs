#[derive(Debug, PartialEq, Eq)]
pub struct FaceHairStyle {
    pub uid: u32,
    pub style: &'static str,
    pub description: &'static str,
    pub image_url: &'static str,
}

// square face shape
pub const HAIR_STYLE_TEXTURED_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 0,
    style: "Textured Bob",
    description: "A bob with layers adds texture and movement, softening the jawline.",
    image_url: "https://picsum.photos/150",
};
// square face shape
pub const HAIR_STYLE_PIXIE_CUT: &FaceHairStyle = &FaceHairStyle {
    uid: 1,
    style: "Pixie Cut",
    description:
        "A layered pixie cut can create a more oval appearance, especially with some volume on top.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_ASYMMETRICAL_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 2,
    style: "Asymmetrical Bob",
    description: "An uneven bob with one side longer than the other can add interest and balance the strong lines of a square face.",
    image_url: "https://picsum.photos/150",

};

pub const HAIR_STYLE_LAYERED_LOB: &FaceHairStyle = &FaceHairStyle {
    uid: 3,
    style: "Layered Lob",
    description: "A long bob that hits around the collarbone with layers can elongate the face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SIDE_SWEPT_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 4,
    style: "Side-Swept Bangs",
    description: "Side-swept bangs can soften the angular features of a square face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_WAVY_SHAG: &FaceHairStyle = &FaceHairStyle {
    uid: 5,
    style: "Wavy Shag",
    description: "Medium-length hair with shaggy layers and waves can add softness and movement.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LONG_LAYERS: &FaceHairStyle = &FaceHairStyle {
    uid: 6,
    style: "Long Layers",
    description:
        "Long hair with layers starting at the jawline can soften the face and add dimension.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_A_LINE_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 7,
    style: "Beach Waves",
    description:
        "Loose, tousled waves can create a soft, romantic look that balances a square face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SIDE_PART: &FaceHairStyle = &FaceHairStyle {
    uid: 8,
    style: "Side Part",
    description: "A deep side part can offset the symmetry of a square face, creating a more oval appearance.",
    image_url: "https://picsum.photos/150",

};

pub const HAIR_STYLE_LOOSE_UPDO: &FaceHairStyle = &FaceHairStyle {
    uid: 9,
    style: "Loose Updo",
    description: "An updo with loose tendrils framing the face can soften strong features.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_HALF_UP_HALF_DOWN: &FaceHairStyle = &FaceHairStyle {
    uid: 10,
    style: "Half-Up, Half-Down",
    description: "This style adds volume on top while allowing some hair to frame the face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SOFT_PONYTAIL: &FaceHairStyle = &FaceHairStyle {
    uid: 11,
    style: "Soft Ponytail",
    description: "A low ponytail with some volume at the crown and loose strands around the face can be flattering.",
    image_url: "https://picsum.photos/150",

};

pub const HAIR_STYLE_FACE_CURTAIN_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 12,
    style: "Curtain Bangs",
    description: "These bangs frame the face on both sides, creating a soft, flattering look.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_FACE_FEATHERED_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 13,
    style: "Feathered Bangs",
    description: "Light, feathered bangs can soften a square face without being too harsh.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_PIXIE_CUT_WITH_VOLUME: &FaceHairStyle = &FaceHairStyle {
    uid: 14,
    style: "Pixie Cut with Volume",
    description: "A pixie cut with added volume on top elongates the face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_TEXTURED_CROP: &FaceHairStyle = &FaceHairStyle {
    uid: 15,
    style: "Textured Crop",
    description: "Short and choppy layers add dimension and divert attention from the roundness.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LONG_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 16,
    style: "Long Bob",
    description: " A lob that hits below the chin elongates the face. Adding layers can further enhance the effect.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SHAGGY_LOB: &FaceHairStyle = &FaceHairStyle {
    uid: 17,
    style: "Shaggy Lob",
    description: "A medium-length shag with layers adds texture and movement.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LAYERED_CUT: &FaceHairStyle = &FaceHairStyle {
    uid: 19,
    style: "Layered Cut",
    description: "Long layers starting below the chin create a slimming effect.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_STRAIGHT_AND_SLEEK: &FaceHairStyle = &FaceHairStyle {
    uid: 20,
    style: "Straight and Sleek",
    description: " Long, straight hair with a center part can make the face appear longer.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_BEACH_WAVES: &FaceHairStyle = &FaceHairStyle {
    uid: 21,
    style: "Beach Waves",
    description: "Loose waves add volume and length without adding width.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_HIGH_PONYTAIL: &FaceHairStyle = &FaceHairStyle {
    uid: 22,
    style: "High Ponytail",
    description: "A ponytail positioned high on the head adds height and elongates the face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_TOP_KNOT: &FaceHairStyle = &FaceHairStyle {
    uid: 23,
    style: "Top Knot",
    description: "A top knot or high bun creates vertical length.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_CURTAIN_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 25,
    style: "Curtain Bangs",
    description:
        "These bangs part in the middle and frame the face, creating a lengthening effect.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SIDE_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 26,
    style: "Side Bangs",
    description: "Side-swept bangs add angles and can make the face look longer.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_FEATHERED_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 27,
    style: "Feathered Bangs",
    description: "Light, airy bangs can soften the face without adding too much width.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_CLASSIC_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 28,
    style: "Classic Bob",
    description: "A chin-length bob can accentuate the natural symmetry of an oval face.",
    image_url: "https://picsum.photos/150",
};
pub const HAIR_STYLE_LAYERED_SHAG: &FaceHairStyle = &FaceHairStyle {
    uid: 29,
    style: "Layered Shag",
    description: "Medium-length hair with shaggy layers adds movement and volume.",
    image_url: "https://picsum.photos/150",
};
pub const HAIR_STYLE_BLUNT_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 30,
    style: "Blunt Bangs",
    description: "Straight-across bangs can add a bold, edgy look while balancing the forehead.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_WISPY_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 31,
    style: "Wispy Bangs",
    description: " Light, wispy bangs soften the face and add a delicate touch.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SOFT_CHIGNON: &FaceHairStyle = &FaceHairStyle {
    uid: 32,
    style: "Soft Chignon",
    description: "A low, soft chignon with tendrils framing the face adds width and softness.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LOOSE_PONYTAIL: &FaceHairStyle = &FaceHairStyle {
    uid: 33,
    style: "Loose Ponytail",
    description: "A low, loose ponytail with some volume at the sides and crown can add width and soften the face.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_PIXIE_CUT_WITH_BANGS: &FaceHairStyle = &FaceHairStyle {
    uid: 34,
    style: "Pixie Cut with Bangs",
    description:
        "A pixie cut that includes bangs can shorten the face’s appearance and add softness.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LOOSE_WAVES: &FaceHairStyle = &FaceHairStyle {
    uid: 35,
    style: "Loose Waves",
    description: "Long, loose waves add width and create a more rounded appearance.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_VOLUMINOUS_CURLS: &FaceHairStyle = &FaceHairStyle {
    uid: 36,
    style: "Voluminous Curls",
    description: "Big, bouncy curls add width and dimension, making the face look more balanced.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LOW_SIDE_SWEPT_PONYTAIL: &FaceHairStyle = &FaceHairStyle {
    uid: 37,
    style: "Low, Side-Swept Ponytail",
    description: "A low ponytail swept to the side can add width and draw attention away from the length of the face.",
    image_url: "https://picsum.photos/150",
};
pub const HAIR_STYLE_CHIN_LENGTH_BOB: &FaceHairStyle = &FaceHairStyle {
    uid: 38,
    style: "Chin-Length Bob",
    description: "A bob that hits at the chin adds width and balances the face’s length.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_TEXTURED_PIXIE_CUT: &FaceHairStyle = &FaceHairStyle {
    uid: 39,
    style: "Textured Pixie Cut",
    description: "A pixie cut with textured layers can add volume around the jawline and soften the forehead.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_SIDE_SWEPT_PIXIE: &FaceHairStyle = &FaceHairStyle {
    uid: 40,
    style: "Side-Swept Pixie",
    description: "This style draws attention away from the forehead and highlights the cheekbones.",
    image_url: "https://picsum.photos/150",
};

pub const HAIR_STYLE_LOW_BUN: &FaceHairStyle = &FaceHairStyle {
    uid: 41,
    style: "Low Bun",
    description: "A low, loose bun adds volume at the nape of the neck, balancing the face shape.",
    image_url: "https://picsum.photos/150",
};
