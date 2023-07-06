use crate::domain;

pub trait CommunityService {
    fn get_all_communities_from_system(&self) -> Vec<domain::communities::Community>;
}

pub trait PostService {
    fn get_all_posts_from_system(&self, community_name: String) -> Vec<domain::posts::Post>;
    fn create_post(&mut self, author: String, message: String);
}