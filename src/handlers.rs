pub mod communities{
    use rocket::response::{status};
    use rocket_contrib::json::Json;

    use crate::{traits::CommunityService, services::communities::CommunityServiceImpl, domain::communities::Community};

    #[get("/communities")]
    pub fn get_communities() -> status::Accepted<Json<Vec<Community>>>{
        let community_service = CommunityServiceImpl;
        let all_communities: Vec<Community> = community_service.get_all_communities_from_system();
        status::Accepted(Some(Json(all_communities)))
    }
}

pub mod posts {
    use rocket::response::{status};
    use rocket_contrib::json::Json;

    use crate::{domain::posts::Post, services::posts::PostServiceImpl, traits::PostService};

    #[get("/posts/<community_name>")]
    pub fn get_posts(community_name: String) -> status::Accepted<Json<Vec<Post>>>{
        let post_service: PostServiceImpl = PostServiceImpl;
        let all_posts: Vec<Post> = post_service.get_all_posts_from_system(community_name);
        status::Accepted(Some(Json(all_posts)))
    }
}
