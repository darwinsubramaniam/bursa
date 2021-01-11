mod equity;
mod company_profile;

use std::time::Instant;

use chrono::Duration;
use company_profile::{chart_reading::ChartUrlContent, listed_company};
use equity::page_info;
use listed_company::CompanyProfile;
use page_info::{PageInfo, PageShareInfo};
use select::document::Document;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let first_page_info = PageInfo::new(1);

    let first_page_source = first_page_info.load().await.unwrap();

    let first_page_doc = Document::from(first_page_source.as_str());

    let total_page = first_page_info.total_page(&first_page_doc).unwrap();

    let mut session_share_info: Vec<PageShareInfo> = Vec::new();

    for page_number in 1..total_page {
        let page_info = PageInfo::new(page_number);

        let page_source = page_info.load().await.unwrap();

        let page_doc = Document::from(page_source.as_str());

        let nodes = page_info.table_data(&page_doc);

        for node in nodes {
            let share_info = page_info
            .extract_data(&node);

            session_share_info
            .push(share_info);
        }
    }

    let total_page_found_duration = start.elapsed();

    println!("Duration - Total Equity Table Extraction :: {:?} ", total_page_found_duration);

    let start = Instant::now();

    for share_info in &session_share_info{
        let company_profile = CompanyProfile::new(&share_info.stock_code);

        let company_profile_page = company_profile
        .load().await
        .unwrap();

        let company_profile_doc = Document::from(company_profile_page.as_str());

        let chart_node = company_profile.div_of_chart(&company_profile_doc);

        let mut chart = ChartUrlContent::new(&chart_node);

        let from_duration = Duration::weeks(53);

        let chart = chart.get_chart_data_url(&from_duration);

        let company_fullname = company_profile.company_fullname(&company_profile_doc);
        let market = company_profile.market(&company_profile_doc);
        let sector = company_profile.sector(&company_profile_doc);

        println!("
        Company: {} 
        Market: {}
        Sector: {}
        Chart_URL: {}",
        &company_fullname, &market,&sector,&chart.as_str());
    }

    let total_company_profile_extraction = start.elapsed();

    println!("Total Execution Duration {:?}",total_company_profile_extraction)

}
