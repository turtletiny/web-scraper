use std::{process, time::Instant};
use trpl::{Either, Html};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //USAGE: 
    //[FLAG] [LINK(s)]

    trpl::block_on(async {
        let start = Instant::now();
        let title = page_title(match &args.get(1) {
            Some(link) => link,
            None => process::exit(69),
        })
        .await;

        match title {
            Some(title) => println!("Title: '{title}'"),
            None => println!("Had no title!"),
        }
        let duration = start.elapsed();
        println!("Took: {}ms", duration.as_millis());
    });

    // RACING 2 PAGES: (ADD FLAG TO IMPL LATER)
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

async fn page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
// async fn page_title(url: &str) -> (&str, Option<String>) {
//     let response_text = trpl::get(url).await.text().await;
//     let title = Html::parse(&response_text)
//         .select_first("title")
//         .map(|title| title.inner_html());
//
//     (url, title)
// }
