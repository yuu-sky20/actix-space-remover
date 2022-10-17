use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct TextParameter {
    input_text: String,
}


#[post("/result")]
async fn post_text(form: web::Form<TextParameter>) -> impl Responder {
    let response : String = space_remover(&form.input_text);
    let style = r#"
                    <style type="text/css">
                        .textbox{height: 500; width: 600}
                    </style>
                "#;
    macro_rules! base_text {() => (
        r#"
            <title>Space Remover</title>
            <label>
                <textarea class="textbox" wrap="soft" autocomplete="off" readonly>{text}</textarea>
            </label>
        "#
    )}
    let text = format!(base_text!(), text = response) + style;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(text)
}

async fn get_index() -> impl Responder {
    let style = r#"
                    <style type="text/css">
                        .textbox{height: 500; width: 600}
                    </style>.
                "#;
    let text = 
            r#"
                <title>Space Remover</title>
                <form action="/result" method="post">
                <input type="textarea" name="input_text" class="textbox" wrap="soft" autocomplete="off"/>
                <button type="submit">Remove whitespaces</button>
                </form>
            "#.to_string() + style;
    HttpResponse::Ok()
        .content_type("text/html")
        .body(text)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(post_text)
            .route("/", web::get().to(get_index))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

fn space_remover(s: &str) -> String {
    let all_text = s.to_string();
    let stage1: Vec<&str> = all_text.split_whitespace().collect();
    let stage1_str = stage1.concat().to_string();
    stage1_str
}

#[test]
fn test_space_remover() {
    let samples : Vec<(&str, &str)> = vec![
        ("a b c d e f gh ij k l m n",
            "abcdefghijklmn"),
        ("あ　いうえ　お、　かき　くけ　こ　　さし　すせそ　",
            "あいうえお、かきくけこさしすせそ"),
        ("",
            ""),
        ("012 345 6 7 89   10",
            "012345678910"),
        ("first line\n
        second line",
            "firstlinesecondline"),
    ];
    for sample in samples {
        assert_eq!(space_remover(sample.0), sample.1);
    }
}