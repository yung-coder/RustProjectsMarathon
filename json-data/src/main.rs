use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let json = r#"
    {
       "article": "how to work with json in rust",
       "author": "Saksham",
       "paragraph": [
           {
             "name": "Starting"
           },
           {
             "name": "Chad"
           },
           {
             "name": "End"
           }       
       ]
    }"#;

    let parsed: Article = read_json_typed(json);
    println!(
        "\n\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}

fn read_json_typed(raw_json: &str) -> Article {
    let parsed: Result<Article, serde_json::Error> = serde_json::from_str(raw_json);

    match parsed {
        Ok(article) => article,
        Err(err) => {
            eprintln!("Failed to parse JSON: {}", err);
            std::process::exit(1);
        }
    }
}
