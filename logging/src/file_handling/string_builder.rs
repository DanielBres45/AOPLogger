pub struct StringBuilder
{
    my_str: String
}


impl Into<String> for StringBuilder{
    fn into(self) -> String {
        self.my_str
    }
}

impl StringBuilder
{
    pub fn new() -> Self
    {
        StringBuilder
        {
            my_str: "".to_owned()
        }
    }

    pub fn add(mut self, append: &str) -> Self
    {
        self.my_str += append;
        self
    }

    pub fn add_string(self, append: String) -> Self
    {
        self.add(&append)
    }

    pub fn add_line(mut self, append: &str) -> Self
    {
        self.my_str = self.my_str + append + "\n";
        self
    }

    pub fn add_line_string(self, append: String) -> Self
    {
        self.add_line(&append)
    }
}