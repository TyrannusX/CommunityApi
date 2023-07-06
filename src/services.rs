pub mod communities {
    use crate::traits;
    use crate::domain;

    pub struct CommunityServiceImpl;

    impl traits::CommunityService for CommunityServiceImpl{
        fn get_all_communities_from_system(&self) -> Vec<domain::communities::Community> {
            let mut community_vector: Vec<domain::communities::Community> = Vec::new();
            community_vector.push(domain::communities::Community::new(String::from("Games"), String::from("We like video games."), 10));
            community_vector.push(domain::communities::Community::new(String::from("Books"), String::from("We like books."), 15));
            community_vector.push(domain::communities::Community::new(String::from("TV"), String::from("We like TV."), 100));

            community_vector
        }
    }
}

pub mod posts{
    use std::time::SystemTime;
    use std::time::UNIX_EPOCH;

    use crate::domain::posts::Post;
    use crate::traits;
    use crate::domain;

    pub struct PostServiceImpl;

    impl traits::PostService for PostServiceImpl{
        fn get_all_posts_from_system(&self, community_name: String) -> Vec<Post>{
            let mut post_vector: Vec<Post> = Vec::new();
            post_vector.push(Post::new(String::from("TyrannusX"), String::from("This is my first book post"), SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get epoch").as_millis(), String::from("Books")));
            post_vector.push(Post::new(String::from("TyrannusX"), String::from("I am enjoying ASOIAF!! So good!!"), SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get epoch").as_millis(), String::from("Books")));

            post_vector
        }

        fn create_post(&mut self, author: String, message: String){
            
        }
    }
}