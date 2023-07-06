pub mod communities {
    use serde_derive::Serialize;
    #[derive(Serialize)]
    pub struct Community {
        name: String,
        description: String,
        population: u32,
    }

    impl Community {
        pub fn new(name: String, description: String, population: u32) -> Community {
            Community { 
                name: name, 
                description: description, 
                population: population 
            }
        }
    }
}

pub mod posts {
    use serde_derive::Serialize;

    #[derive(Serialize, Clone)]
    pub struct Post{
        author: String,
        message: String,
        created_at: u128,
        votes: i32,
        replies: Vec<Post>,
        community_id: String
    }

    impl Post{
        pub fn new(author: String, message: String, created_at: u128, community_id: String) -> Post{
            Post{
                author: author,
                message: message,
                created_at: created_at,
                votes: 0,
                replies: Vec::new(),
                community_id: community_id
            }
        }
    }
}