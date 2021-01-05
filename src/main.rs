mod equity;

use equity::page_info;
use page_info::{PageInfo, PageShareInfo};
use select::document::Document;

#[tokio::main]
async fn main() {
    let first_page_info = PageInfo::new(1);

    let first_page_source = first_page_info.load().await.unwrap();

    let first_page_doc = Document::from(first_page_source.as_str());

    let total_page = first_page_info.total_page(&first_page_doc).unwrap();

    let mut session_share_info: Vec<PageShareInfo> = Vec::new();

    for index in 1..total_page {
        let page_info = PageInfo::new(index);

        let page_source = page_info.load().await.unwrap();

        let page_doc = Document::from(page_source.as_str());

        let nodes = page_info.table_data(&page_doc);

        for node in nodes {
            session_share_info.push(page_info.extract_data(&node));
        }

        println!(
            "Total information found in the session are {}",
            session_share_info.len()
        );
    }
}
