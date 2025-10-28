pub struct RectCenterPos
{
    pub pos_y: i32,
    pub pos_x: i32,
    pub w: u32,
    pub h: u32
}

pub fn get_center(rect_size: (i32, i32), window_pos: (u32, u32)) -> RectCenterPos
{
    let new_pos = ((window_pos.0 as i32 / 2) - (rect_size.0 / 2), (window_pos.1 as i32 / 2) - (rect_size.1 / 2));
    RectCenterPos { pos_x: new_pos.0, pos_y: new_pos.1, w: rect_size.0 as u32, h: rect_size.1 as u32 }
}
