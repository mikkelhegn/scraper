use scraper::{Html, Selector};
use spin_sdk::http::{IntoResponse, Method, Request, Response};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
async fn handle_scraper(req: Request) -> anyhow::Result<impl IntoResponse> {

    let mut text: String = Default::default();

    let uris: Vec<String> = serde_json::from_str(&String::from_utf8(req.body().to_vec())?)?;
    
    for uri in uris {
        let blog_request = Request::builder()
            .method(Method::Get)
            .uri(uri.clone())
            .build();

        println!("Getting the blog content for: {:?}", uri);

        let blog: Response = spin_sdk::http::send(blog_request).await?;
        let document = Html::parse_document(&String::from_utf8(blog.body().to_vec())?);
        let selector = Selector::parse("article").unwrap();
        let article = document.select(&selector).next().unwrap();
        text.push_str(&article.text().collect::<Vec<_>>().concat());
    }

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(text)
        .build())
}
