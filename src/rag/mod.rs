use futures::StreamExt;

pub trait Retriever {
	type Prompt;
	type Ctx;
	fn retrieve_context(&self, prompt: Self::Prompt) -> Self::Ctx;
}

pub trait Augmenter {
	type Prompt;
	type Ctx;
	fn augment(&self, prompt: Self::Prompt, context: Self::Ctx) -> Self::Prompt;
}

pub trait Generator {
	type Prompt;
	type Chunk;
	fn generate(&self, prompt: Self::Prompt) -> impl StreamExt;
}

#[allow(dead_code)]
pub struct RAGClient<R, A, G, P, Ctx>
where
	R: Retriever<Prompt=P, Ctx=Ctx>,
	A: Augmenter<Prompt=P, Ctx=Ctx>,
	G: Generator<Prompt=P>,
{
	retriever: R,
	augmenter: A,
	generator: G
}

#[cfg(test)]
pub mod tests {
	struct MockRetriever {}
	struct MockAugmenter {}
	struct MockGenerator {}

	#[test]
	fn test_simple_usecase() {

	}
}