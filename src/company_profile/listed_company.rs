use document::Document;
use reqwest::{Result, Url, get};
use select::{document, node::Node, predicate::Attr};


#[derive(Debug)]
pub struct CompanyProfile{
    stock_code:String,
    url:Url
}

impl CompanyProfile{

    #[allow(dead_code)]
    pub fn new(stock_code:&str)->Self{
        let url_str = format!("https://www.bursamalaysia.com/trade/trading_resources/listing_directory/company-profile?stock_code={}",stock_code);
        return CompanyProfile{
            url: Url::parse(&url_str).unwrap(),
            stock_code:stock_code.to_owned()
        }
    }

    #[allow(dead_code)]
    pub async fn load(&self) -> Result<String> {
        let url = self.url.as_str();

        let page_html = get(url).await?.text().await?;

        Ok(page_html)
    }

    // Get section of the document with the section div of id stockChartContainer.
    #[allow(dead_code)]
    pub fn div_of_chart<'a>(&self, doc:&'a Document) -> Node<'a> {
        doc.find(Attr("id","stockChartContainer"))
        .last()
        .unwrap()
    }
}


