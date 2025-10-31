pub trait GetOrCreate
{
    fn get_or_create(&mut self, index: usize) -> String;
}

impl GetOrCreate for Vec<String>
{
    ///Helper Function That Prevent Crash If The PageData (vec_user_input) is still not populated
    fn get_or_create(&mut self, index: usize) -> String
    {
        // If index doesn't exist, return an empty string
        if index >= self.len()
        {
            return String::new();
        }
        self[index].to_string()
    }
}
