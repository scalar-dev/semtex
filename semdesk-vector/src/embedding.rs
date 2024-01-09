
trait EmbeddingModel {
    pub fn embed(text: &[&str]) -> [f32];
}