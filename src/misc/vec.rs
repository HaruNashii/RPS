pub trait GetOrCreate
{
    fn get_or_create(&mut self, index: usize) -> String;
}

impl GetOrCreate for Vec<String>
{
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
