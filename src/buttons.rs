pub fn button_action(button_clicked: Option<usize>, get_user_input: &mut (bool, usize), page_to_render: &mut u8)
{
    match button_clicked
    {
        Some(1) =>
        {
            //PAGE 1 BUTTON (PERSISTENT PAGE)
            *page_to_render = 1;
        }

        Some(2) =>
        {
            //PAGE 2 BUTTON (PERSISTENT PAGE)
            *page_to_render = 2;
        }

        Some(3) => 
        {
            // PAGE 1 BUTTON (PAGE 1)
            *get_user_input = (true, 0);
        }

        Some(4) => 
        {
            //PAGE 2 TO SUBPAGE_PAGE2 BUTTON (PAGE 2)
            *page_to_render = 3;
        }
            
        Some(5) => 
        {
            //SUBPAGE_PAGE2 TO PAGE 2 BUTTON (PAGE 3)
            *page_to_render = 2;
        }

        Some(6) =>
        {
            // PAGE 2 BUTTON (PAGE 2)
            *get_user_input = (true, 1);
        }
        
        _=> {},
    }
}
