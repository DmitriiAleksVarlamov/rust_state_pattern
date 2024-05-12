use blog_app::Post;

fn main() {
    let mut post = Post::new();
    post.add_content("Some new content");

    post.request_review();
    assert_eq!("", post.get_content());

    post.approve();
    assert_eq!("Some new content", post.get_content());
}
