use actix::prelude::*;
use scraper_lib::{ElementRef, Selector};

// Define the ScrapeRequest trait which will be used as an associated type
trait ScrapeRequest<'a>{
    type Element;
    type Selector; // Associated type representing the element
    type Sender : Actor; // Associated type representing the sender
    // methods and types for ScrapeRequest
    fn get_selector(&self) -> Self::Selector;
    fn get_url(&self) -> &str;
    fn get_sender(&self) -> &Addr<Self::Sender>;
}

struct HtmlScrapeRequest<Sender : Actor> {
    // fields and methods for HtmlScrapeRequest
    url: String,
    selector: Selector,
    sender: Addr<Sender>,
    // The element type is String

}

impl <'a, Sender : Actor>ScrapeRequest<'a> for HtmlScrapeRequest<Sender> {
    type Element = ElementRef<'a>;  
    type Selector = Selector;
    type Sender = Sender;



    fn get_selector(&self) -> Self::Selector {
        self.selector.clone()
    }

    fn get_url(&self) -> &str {
        &self.url
    }

    fn get_sender(&self) -> &Addr<Self::Sender> {
        &self.sender
    }
}

// The Scraper trait extends the Actor trait
trait Scraper: Actor {
    type Request<'a>: ScrapeRequest<'a>;

    // Method to scrape based on the ScrapeRequest
    fn scrape<'a>(&self, request: Self::Request<'a>) -> dyn Iterator<Item = <Self::Request<'a> as ScrapeRequest<'a>>::Element>;

    // Method to process the scraped data
    fn process<'a>(&self, data: <Self::Request<'a> as ScrapeRequest<'a>>::Element) -> ProcessingResult;
}

trait ScraperScheduler: Actor {
    type Request<'a>: ScrapeRequest<'a>;
    // Method to schedule the scraping
    fn schedule<'a>(&self, request: &'a Self::Request<'a>);

    // Method to process the scraped data
    fn process<'a>(&self, processing_result: ProcessingResult);
}

// Assuming the definition of ProcessingResult exists
struct ProcessingResult {
    // fields and methods for ProcessingResult
}

struct ScrapeScheduler {
    // fields and methods for ScrapeScheduler
}

impl Actor for ScrapeScheduler {
    type Context = Context<Self>;
}

