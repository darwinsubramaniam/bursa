///! Using the div with the attribute id "stockChartContainer"
///! chart url json can be downloaded
///! https://ws20.bursamalaysia.com/api/v2/stock_price_data?
///! stock_code=0072.MY&
///! mode=historical&
///! from_date=20160106&
///! ws_a=be89df97117532618b410aee79a5813a3b8d6b920d0c92612e7b9d7a5f8b2a83&
///! ws_m=1610275366.0

///! <div id="stockChartContainer" 
///! data-stock-code="0072.MY" 
///! data-api-source="//ws20.bursamalaysia.com/api/v2/stock_price_data" 
///! data-ws-a="a72f171aea702cbe81246a4356d4bdcd71fd03de085d788121b5aa9158956a78"
///! data-ws-m="1610273971.0"></div>

use chrono::{Duration, Utc};
use select::node::Node;
use url::Url;

#[warn(dead_code)]
pub struct ChartUrlContent{
    base_url:String,
    pub data_stock_code:String,
    pub data_api_source:String,
    pub data_ws_a:String,
    pub data_ws_m:String,
}

impl ChartUrlContent {

    #[allow(dead_code)]
    pub fn new(stock_chart_container:&Node) -> Self{

        let stock_code =stock_chart_container
        .attr("data-stock-code")
        .unwrap();

        let api_source = stock_chart_container
        .attr("data-api-source")
        .unwrap();

        let ws_a = stock_chart_container
        .attr("data-ws-a")
        .unwrap();

        let ws_m = stock_chart_container
        .attr("data-ws-m")
        .unwrap();

        return ChartUrlContent{
            base_url:String::from("https://ws20.bursamalaysia.com/api/v2/stock_price_data"),
            data_stock_code:stock_code.to_owned(),
            data_api_source:api_source.to_owned(),
            data_ws_a :ws_a.to_owned(),
            data_ws_m: ws_m.to_owned(),
        }
    }

    #[warn(dead_code)]
    #[allow(dead_code)]
    pub fn get_chart_data_url<'a>(&'a mut self, duration:&Duration) -> Url{

        let mut url = Url::parse(&self.base_url.as_str()).unwrap();

        let now = Utc::now();

        let from_date = now.checked_sub_signed(*duration).unwrap();

        let fmt_date = from_date.format("%Y%m%d").to_string();

        let query = format!("stock_code={}&mode=historical&from_date={}&ws_a={}&ws_m={}",
         self.data_stock_code,
         &fmt_date,
         self.data_ws_a,
         self.data_ws_m);

        url.set_query(Some(query.as_str()));
        
        url
    } 
}

#[cfg(test)]
mod test{
    use chrono::{Duration, Utc};
    use reqwest::Url;
    use select::{document::Document, predicate::Attr};

    use super::ChartUrlContent;

    static TEST_HTML_CONTENT: &str = r#"<div id="stockChartContainer" 
        data-stock-code="0072.MY" 
        data-api-source="//ws20.bursamalaysia.com/api/v2/stock_price_data" 
        data-ws-a="a72f171aea702cbe81246a4356d4bdcd71fd03de085d788121b5aa9158956a78"  
        data-ws-m="1610273971.0">
        </div>"#;

    fn default_chart_url_content() -> ChartUrlContent{
        let doc = Document::from(TEST_HTML_CONTENT);

        let node = doc
        .find(Attr("id","stockChartContainer"))
        .last()
        .unwrap();

        let default_chart_url = ChartUrlContent::new(&node);

        default_chart_url
    }

    #[test]
    fn default_chart_url(){
        
        let default_chart_url = default_chart_url_content();

        assert_eq!(default_chart_url.data_stock_code,"0072.MY");
        assert_eq!(default_chart_url.data_ws_a,"a72f171aea702cbe81246a4356d4bdcd71fd03de085d788121b5aa9158956a78");
        assert_eq!(default_chart_url.data_ws_m,"1610273971.0");
        assert_eq!(default_chart_url.data_api_source,"//ws20.bursamalaysia.com/api/v2/stock_price_data");
    }

    #[test]
    fn generate_correct_chart_url(){
        let mut chart_url = default_chart_url_content();

        let duration = Duration::weeks(53);

        let url = chart_url.get_chart_data_url(&duration);

        let from_date = Utc::now()
        .checked_sub_signed(duration)
        .unwrap();

        let fmt_date = from_date
        .format("%Y%m%d")
        .to_string();

        let mut expected_url = Url
        ::parse("https://ws20.bursamalaysia.com/api/v2/stock_price_data")
        .unwrap();

        let query = format!("stock_code={}&mode=historical&from_date={}&ws_a={}&ws_m={}",
         "0072.MY",
         fmt_date,
         "a72f171aea702cbe81246a4356d4bdcd71fd03de085d788121b5aa9158956a78",
         "1610273971.0");

        expected_url
        .set_query(Some(query.as_str()));

        assert_eq!(url.as_str(),expected_url.as_str());
    }

    

}
