use reqwest::blocking;
use reqwest::Result;
use select::node::Node;
use select::predicate::Name;
use select::{document::Document, predicate::Attr};
use url::Url;

#[derive(Debug)]
pub struct PageInfo {
    base_url: Url,
    page_number: i32,
    url: Option<Url>,
}

#[derive(Debug)]
pub struct PageShareInfo {
    share_name: String,
    share_code: String,
}

impl PartialEq for PageShareInfo {
    fn eq(&self, other: &Self) -> bool {
        let same_name = self.share_name == other.share_name;
        let same_code = self.share_code == other.share_code;

        same_name && same_code
    }
}

impl PageInfo {
    #[allow(dead_code)]
    pub fn new(page_number: i32) -> PageInfo {
        let mut new_info = PageInfo {
            base_url: Url::parse(
                "https://www.bursamalaysia.com/market_information/equities_prices",
            )
            .unwrap(),
            page_number,
            url: None,
        };

        let page_url = new_info.page_url();
        new_info.url = Some(page_url);

        new_info
    }

    #[allow(dead_code)]
    pub fn page_url(&self) -> Url {
        let query = format!("per_page=50&page={}", self.page_number);
        let mut page_url: Url = self.base_url.clone();

        page_url.set_query(Some(&query));

        println!("url run in {}", page_url.as_str());

        page_url
    }

    #[allow(dead_code)]
    pub fn total_page<'a>(&self, doc: &'a Document) -> Result<i32> {

        let total = doc.find(Attr("id", "total_page")).last().unwrap();

        println!("{:?}",total);

        let total = total.attr("data-val").unwrap().to_string();

        println!("Total Page {}", total);

        Ok(total.parse().unwrap())
    }

    #[allow(dead_code)]
    pub async fn page_to_go(&self) -> Result<i32> {
        Ok(1)
    }

    #[allow(dead_code)]
    pub async fn load(&self) -> Result<String> {
        let url = &self.page_url().to_string();

        let page_html = blocking::get(url)?.text()?;

        Ok(page_html)
    }

    #[allow(dead_code)]
    pub fn table_data<'a>(&self, doc: &'a Document) -> Vec<Node<'a>> {
        let table = doc.find(Attr("id", "data-1")).collect::<Vec<Node>>();

        table
    }

    pub fn extract_data<'a>(&self, table_data: &'a Node) -> PageShareInfo {
        // first is the index
        // second it the share name
        // third is the share code

        let share_name = table_data.find(Name("td")).nth(1).unwrap();
        let share_code = table_data.find(Name("td")).nth(2).unwrap();

        let result = PageShareInfo {
            share_name: share_name.text().trim().to_string(),
            share_code: share_code.text().trim().to_string(),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::equity::page_info::Document;
    use crate::equity::page_info::PageInfo;
    use crate::equity::page_info::PageShareInfo;
    use crate::equity::page_info::Url;
    use std::fs;

    #[test]
    fn correct_new_page_url() {
        let expected_page_url = Url::parse(
            "https://www.bursamalaysia.com/market_information/equities_prices?per_page=50&page=1",
        )
        .unwrap();

        assert_eq!(PageInfo::new(1).url.unwrap(), expected_page_url)
    }

    #[test]
    fn total_page_found() {
        let test_html = fs::read_to_string("src/equity/test_asset/equity_page.html")
        .unwrap();

        let document = Document::from(test_html.as_str());

        let page_info = PageInfo::new(1);

        let total_page = page_info.total_page(&document);

        assert_eq!(total_page.unwrap(), 43);
    }

    #[test]
    fn table_can_be_found() {
        let test_html = fs::read_to_string("src/equity/test_asset/equity_page.html")
        .unwrap();

        let document = Document::from(test_html.as_str());

        let page_info = PageInfo::new(1);

        let table = page_info.table_data(&document);

        assert_eq!(table.len(), 50)
    }

    #[test]
    fn extract_share_basic_info() {
        let test_html = fs::read_to_string("src/equity/test_asset/equity_page.html")
        .unwrap();


        let document = Document::from(test_html.as_str());

        let page_info = PageInfo::new(1);

        let table = page_info.table_data(&document);

        let first_item = table.first().unwrap();

        let expected_item = PageShareInfo {
            share_name: String::from("AT [S]"),
            share_code: String::from("0072"),
        };

        let result = page_info.extract_data(first_item);

        assert_eq!(result, expected_item)
    }
}
