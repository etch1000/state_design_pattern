use state_design_pattern::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("Pingu was the coolest cartoon in the 90s");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("Pingu was the coolest cartoon in the 90s", post.content());

    println!("{}", post.content());
}
