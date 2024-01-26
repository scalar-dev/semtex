
pub trait EmbeddingModel {
    type TokenizedOutput: TokenizedOutput;
    fn tokenize(self: &Self, text: &[&str]) -> Vec<Self::TokenizedOutput>;
    fn embed(self: &Self, tokenized_output: Vec<Self::TokenizedOutput>) -> Vec<Vec<f32>>;
}

pub trait  TokenizedOutput { }
