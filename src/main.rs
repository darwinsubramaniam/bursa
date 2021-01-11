mod company_profile;
mod equity;

use std::{convert::TryInto, io, time::Instant};

use chrono::Duration;
use company_profile::{chart_reading::ChartUrlContent, listed_company};
use equity::page_info;
use listed_company::CompanyProfile;
use page_info::{PageInfo, PageShareInfo};
use pbr::ProgressBar;
use select::document::Document;
use tokio::task::JoinHandle;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let first_page_info = PageInfo::new(1);

    let first_page_source = first_page_info.load().await.unwrap();

    let first_page_doc = Document::from(first_page_source.as_str());

    let total_page = first_page_info.total_page(&first_page_doc).unwrap();

    let mut session_share_info: Vec<PageShareInfo> = Vec::new();

    let mut progress_equity_page = ProgressBar::new(total_page.try_into().unwrap());

    progress_equity_page.format("╢▌▌░╟");
    println!("Start Equity Page Extraction");

    let mut spawns: Vec<JoinHandle<Result<Vec<PageShareInfo>, io::Error>>> = Vec::new();
    for page_number in 1..total_page {
        progress_equity_page.inc();

        let thread: JoinHandle<Result<Vec<PageShareInfo>, io::Error>> = tokio::spawn(async move {
            let page_info = PageInfo::new(page_number);

            let page_source = page_info.load().await.unwrap();

            let page_doc = Document::from(page_source.as_str());

            let nodes = page_info.table_data(&page_doc);

            let mut current_equity_page_info: Vec<PageShareInfo> = Vec::new();

            for node in nodes {
                let share_info = page_info.extract_data(&node);
                current_equity_page_info.push(share_info);
            }

            Ok(current_equity_page_info)
        });
        spawns.push(thread);
    }

    for thread in spawns {
        let results = thread.await.unwrap().unwrap();

        for item in results {
            session_share_info.push(item);
        }
    }

    progress_equity_page.finish_println("Complete Equity Page Extraction");

    let total_page_found_duration = start.elapsed();

    println!(
        "Duration - Total Equity Table Extraction :: {:?} ",
        total_page_found_duration
    );

    let start = Instant::now();

    let mut progress_company_info = ProgressBar::new(session_share_info.len().try_into().unwrap());

    progress_company_info.format("╢▌▌░╟");

    let mut spawns: Vec<JoinHandle<Result<String, io::Error>>> = Vec::new();
    for share_info in session_share_info {
       

        let thread: JoinHandle<Result<String, io::Error>> = 
        tokio::spawn(async move {
            let company_profile = CompanyProfile::new(&share_info.stock_code);

            let company_profile_page = company_profile.load().await.unwrap();

            let company_profile_doc = Document::from(company_profile_page.as_str());

            let chart_node = company_profile.div_of_chart(&company_profile_doc);

            let mut chart = ChartUrlContent::new(&chart_node);

            let from_duration = Duration::weeks(53);

            let _chart = chart.get_chart_data_url(&from_duration);

            let _company_fullname = company_profile.company_fullname(&company_profile_doc);
            let _market = company_profile.market(&company_profile_doc);
            let _sector = company_profile.sector(&company_profile_doc);
            Ok(_company_fullname)
        });
        spawns.push(thread);
    }

    for thread in spawns {

        let results = thread.await.unwrap().unwrap();
        progress_company_info.inc();
    }

    progress_company_info.finish_println("Complete company profile extraction.");
    let total_company_profile_extraction = start.elapsed();

    println!(
        "Total Execution Duration {:?}",
        total_company_profile_extraction
    )
}
