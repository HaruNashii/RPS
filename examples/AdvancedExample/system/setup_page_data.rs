use rust_page_system::system::page_system::PageData;
use crate::ui::pages::{page_1, page_2, persistent_elements1, persistent_elements2, subpage_page2, ButtonId, PageId};



pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    //Populate Vec_Of_User_input With Page And Buttons That Receives User_Input
    page_data.push_vec_user_input(vec!
    [
        (PageId::Page1, ButtonId::ButtonPurpleInputStartPage1),
        (PageId::Page1, ButtonId::ButtonRedInputStartPage1),
        (PageId::Page2, ButtonId::ButtonPurpleInputStartPage2),
    ]);

    //Populate Persistent Elements with your defined persistent elements, (If your Persistent
    //Elements have runtime changing elements, like: Userinput, you need to place this definition inside an loop)
    page_data.define_persistent_elements(vec!
    [
        persistent_elements1(),
        persistent_elements2()
    ]);
}
pub fn update_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    //Populate PageData allpages vector
    page_data.populate_and_update_all_pages(vec!
    [
        page_1(&page_data.vec_user_input_string),
        page_2(&page_data.vec_user_input_string),
        subpage_page2(),
    ]);
}

