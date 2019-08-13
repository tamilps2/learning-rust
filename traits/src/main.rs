mod news;
// mod tweets;

pub trait Summary {
    fn summary(&self) -> String;
}

impl Summary for news::NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn main() {
    let article = news::NewsArticle {
        headline: String::from("This is a test line"),
        location: String::from("TN"),
        author: String::from("Mineu"),
        content: String::from("This is a lorem possum"),
    };

    println!("{}", article.summary());
}
