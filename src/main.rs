use colour::{white,green};
use newsapi::get_article;
//use newsapi::Article;
use newsapi::Articles;
use std::error::Error;
use dotenv::dotenv;


fn render_articles(articles: &Articles){
    for i in &articles.articles{
        white!(">{}\n",i.title);
        green!(">{}\n",i.url);
    }
}


fn main()->Result<(),Box<dyn Error>> {
    dotenv()?;
    let api_key=std::env::var("API_KEY")?;

   let url ="https://newsapi.org/v2/everything?q=bitcoin&apiKey=";

   let url=format!("{}{}",url,api_key);

   let articles=get_article(&url)?;
    render_articles(&articles);
    Ok(())
}
