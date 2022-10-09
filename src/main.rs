use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct TextParameter {
    inputText: String,
    outputText: String,
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                <title>Space Remover</title>
                <form action="/output" method="post">
                <input type="text" name="inputText" />
                <button type="submit">Remove whitespaces</button>
                </form>
            "#,
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App.new()
            .route("/", web::get().to(get_index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn space_remover(mut& s: &str) -> String {

}

#[test]
fn test_space_remover() {
    let samples: Vec<(str, str)> = [
        ("a b c d e f gh i j k l m n",
            "abcdefghijklmn"),
        ("あ いうえ お、かき くけ こ　さし す せ そ ",
            "あいうえお、かきくけこさしすせそ"),
        ("",
            ""),
        ("012 345 6 7 89   10",
            "012345678910"),
    ]
    for sample in samples {
        assert_eq!(space_remover(sample.0), sample.1);
    }
}