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

    let article: Article = Article {
        article: String::from("how to add"),
        author: String::from("Saksham"),
        paragraph: vec![
            Paragraph {
                name: String::from("first"),
            },
            Paragraph {
                name: String::from("nahhh"),
            },
            Paragraph {
                name: String::from("Chad shit"),
            },
        ],
    };

    // write to json

    let json2 = serde_json::to_string(&article).unwrap();
    println!("the json is: {}", json2);

    // read json

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
