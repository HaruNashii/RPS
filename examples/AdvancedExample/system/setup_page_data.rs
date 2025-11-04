use crate::ui::pages::{ButtonId, PageId, page_1, page_2, subpage_page2};
use rust_page_system::system::page_system::PageData;
use std::rc::Rc;

/// put here pages that is static and don't need to be updated every frame
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    // When linking pages, wrap the constructors in boxed closures. This allows
    // additional state or configuration to be captured by the closure if needed.
    page_data.push_page_link(Some(vec![(PageId::Page2SubPage, Rc::new(|| subpage_page2()))]), Some(vec![(PageId::Page1, Rc::new(|input: &mut Vec<String>| page_1(input))), (PageId::Page2, Rc::new(|input: &mut Vec<String>| page_2(input)))]));
}
