use crate::ui::pages::{ButtonId, PageId, page_1, page_2, subpage_page2};
use rust_page_system::system::page_system::PageData;
use std::rc::Rc;

pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.populate_rps_data(Some(vec![Rc::new(subpage_page2)]), Some(vec![Rc::new(|input: &mut Vec<String>| page_1(input)), Rc::new(|input: &mut Vec<String>| page_2(input))]));
}
