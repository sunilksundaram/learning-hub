use ollama_rs::{generation::{completion::request::GenerationRequest}, models::ModelOptions, Ollama};

#[tokio::main]
async fn run_it() {
    println!("Hello, world!");
    let ollama = Ollama::new("http://localhost", 11434);

    let model = "lfm2.5-thinking:latest".to_string(); //"granite4.1:8b".to_string();
    let options = ModelOptions::default()
        .temperature(0.2)
        .repeat_penalty(1.5)
        .top_k(25)
        .top_p(0.25)
        .num_ctx(1024);

    let prompt = "What is the meaning of life?".to_string();
    let result = ollama.generate(GenerationRequest::new(model, prompt).options(options)).await;

    if let Ok(result) = result {
        println!("Result: {}", result.response);
    } else {
        println!("Error: {:?}", result);
    }
}

fn main() {
    println!();

    run_it()
}