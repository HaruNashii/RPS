use crate::pages::ButtonId;

pub fn button_action(button_clicked: Option<ButtonId>, get_user_input: &mut (bool, usize), page_to_render: &mut u8)
{
    match button_clicked
    {
        Some(ButtonId::Page1) =>
        {
            *page_to_render = 1;
        }

        Some(ButtonId::Page2) =>
        {
            *page_to_render = 2;
        }

        Some(ButtonId::InputStart1) => 
        {
            *get_user_input = (true, 0);
        }

        Some(ButtonId::SubPage) => 
        {
            *page_to_render = 3;
        }
            
        Some(ButtonId::Back) => 
        {
            *page_to_render = 2;
        }

        Some(ButtonId::InputStart2) =>
        {
            // PAGE 2 BUTTON (PAGE 2)
            *get_user_input = (true, 1);
        }
        
        _=> {},
    }
}
