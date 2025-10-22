use rust_page_system::system::page_system::PageData;
use crate::ui::pages::{page_1, page_2, subpage_page2, ButtonId, PageId};



/// put here pages that is static and don't need to be updated every frame
pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.push_page_link
    (
        Some
        (vec!
        [
            (PageId::Page2SubPage, subpage_page2)
        ]),
        Some
        (vec!
        [
            (PageId::Page1, page_1),
            (PageId::Page2, page_2)
        ])
    );

    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    page_data.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1, ButtonId::ButtonRedInputStartPage1),
        (PageId::Page2, ButtonId::ButtonPurpleInputStartPage2)
    ]);
}
