#[derive(Debug, PartialEq, Eq)]
pub struct FaceActor {
    pub uid: u32,
    pub name: &'static str,
    pub image_url: &'static str,
}

pub fn generate_heart_shape_actor() -> Vec<FaceActor> {
    vec![FaceActor {
        uid: 1,
        name: "Heart Shape",
        image_url: "https://www.google.com",
    }]
}
