use std::time::Instant;
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let start = Instant::now();
        let title = page_title(&args[1]).await.1;

        match title {
            Some(title) => println!("Title: '{title}'"),
            None => println!("Had no title!"),
        }
        let duration = start.elapsed();
        println!("Took: {}ms", duration.as_millis());
    });

    // trpl::block_on(async {
    //     let title_future1 = page_title(&args[1]);
    //     let title_future2 = page_title(&args[2]);
    //
    //     let (url, maybe_title) = match trpl::select(title_future1, title_future2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };
    //
    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("Its page title was: '{title}'"),
    //         None => println!("It had no title."),
    //     }
    // })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response_text = trpl::get(url).await.text().await;
    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}
