pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

impl Post {
    // A "Constructor" method to create a new Post easily
    pub fn new(id: u32, title: String, body: String) -> Self {
        Self { id, title, body }
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

// Implement that speficic trait for our Post struct
impl Summary for Post {
    fn summarize(&self) -> String {
        // Calculate the safe end point for the title and body
        let body_end = self.body.len().min(20);

        format!(
            "(Post #{}) Title: {}, Body: {}...",
            self.id,
            self.title,
            &self.body[..body_end]
        )
    }
}
