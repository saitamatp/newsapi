use std::error::Error;
use serde::Deserialize;
use serde_json;
//use thiserror::Error;

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}

#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed to read Article")]
    RequestFailed(ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article Parsing failed")]
    ArticleParseFailed(serde_json::Error)

} 


#[derive(Deserialize, Debug)]
pub struct Article {
    pub title:String,
    pub url: String 
}



pub fn get_article(url: &str)->Result<Articles,Box<dyn Error>> {
    let response=ureq::get(url).call().map_err(|e| NewsApiError::RequestFailed(e))
    ?.into_string().map_err(|e| NewsApiError::FailedResponseToString(e))?;

    let articles: Articles=serde_json::from_str(&response).map_err(|e| NewsApiError::ArticleParseFailed(e))?;

    Ok(articles)
}