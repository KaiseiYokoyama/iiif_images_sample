#[derive(Debug)]
pub struct URL(String);

#[derive(Deserialize, Debug)]
pub struct Manifest {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    license: String,
    attribution: String,
    description: String,
    label: String,
    sequences: Vec<Sequence>,
}

impl Manifest {
    pub fn get_images(&self) -> Vec<URL> {
        let mut images = Vec::new();

        for sequence in &self.sequences {
            for canvas in &sequence.canvases {
                for image in &canvas.images {
                    images.push(URL(image.resource.id.clone()));
                }
            }
        }

        return images;
    }
}

#[derive(Deserialize, Debug)]
struct Sequence {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    canvases: Vec<Canvas>,
}

#[derive(Deserialize, Debug)]
struct Canvas {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    width: u32,
    height: u32,
    label: String,
    images: Vec<Image>,
}

#[derive(Deserialize, Debug)]
struct Image {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    resource: Resource,
}

#[derive(Deserialize, Debug)]
struct Resource {
    #[serde(rename = "@id")]
    id: String,
    #[serde(rename = "@type")]
    type_: String,
    format: String,
    width: u32,
    height: u32,
    service: Service,
}

#[derive(Deserialize, Debug)]
struct Service {
    #[serde(rename = "@context")]
    context: String,
    #[serde(rename = "@id")]
    id: String,
    profile: String,
}