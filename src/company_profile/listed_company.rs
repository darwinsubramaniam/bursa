use document::Document;
use reqwest::{Result, Url, get};
use select::{document, node::Node, predicate::{Attr, Class, Name}};


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

    pub fn company_fullname<'a>(&self, doc:&'a Document) -> String{

        let section_company_detail = doc
        .find(Name("section"))
        .nth(1)
        .unwrap();

        let company_fullname =section_company_detail
        .find(Class("text-center"))
        .nth(0).unwrap()
        .find(Name("h5"))
        .nth(0).unwrap()
        .text();

        company_fullname
    }

    pub fn market(&self,doc:&Document) ->String{
        let section_company_detail = doc
        .find(Name("section"))
        .nth(1)
        .unwrap();

        let market =section_company_detail
        .find(Class("text-center"))
        .nth(1).unwrap()
        .text()
        .trim()
        .split(':')
        .last()
        .unwrap().trim().to_string();

        market
    }

    pub fn sector(&self,doc:&Document) ->String{
        let section_company_detail = doc
        .find(Name("section"))
        .nth(1)
        .unwrap();

        let sector =section_company_detail
        .find(Class("text-center"))
        .nth(2).unwrap()
        .text()
        .trim()
        .split(':')
        .last()
        .unwrap().trim().to_string();

        sector
    }
}

#[cfg(test)]
mod test{
    use reqwest::blocking;
    use select::document::Document;

    use super::CompanyProfile;


    fn load_test_page() -> String{
        blocking::get("https://gist.githubusercontent.com/darwinsubramaniam/52f4af8cf363e8940adbac7dc57f76b2/raw/05f700b3a552dbddd30c53c2d3599d364d3f395c/company_profile.html")
        .unwrap().text().unwrap()
    }

    #[test]
    fn test_company_fullname(){
        let company:CompanyProfile = CompanyProfile::new("TestCode");
        let company_name = company.company_fullname(&Document::from(load_test_page().as_str()));

        assert_eq!("AT SYSTEMATIZATION BERHAD",company_name);
    }

    #[test]
    fn test_market_info(){
        let company:CompanyProfile = CompanyProfile::new("TestCode");
        let market = company.market(&Document::from(load_test_page().as_str()));

        assert_eq!("ACE Market",market);
    }

    #[test]
    fn test_sector_info(){
        let company:CompanyProfile = CompanyProfile::new("TestCode");
        let market = company.sector(&Document::from(load_test_page().as_str()));

        assert_eq!("INDUSTRIAL PRODUCTS & SERVICES",market);
    }



}




