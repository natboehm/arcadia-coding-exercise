#[derive(Clone)]
struct Blog {
    title: String,
    posts: Vec<Post>,
}

#[derive(Clone)]
struct Post {
    title: String,
    author: String,
    body: String,
    labels: Vec<String>,
    comments: Vec<Comment>,
}

#[derive(Clone)]
struct Comment {
    author: String,
    body: String,
}

impl Blog {
    fn init_blog(title: String) -> Blog {
        let post_list = Vec::new();

        let new_blog = Blog {
            title: title,
            posts: post_list,
        };

        return new_blog
    }

    fn create_post(mut self, title: String, author: String, body: String, labels: Vec<String>, comments: Vec<Comment>) {
        let new_post = Post {
            title: title,
            author: author,
            body: body,
            labels: labels,
            comments: comments,
        };

        self.posts.push(new_post);
    }

    fn read_posts(self) -> Vec<Post> {
        return self.posts;
    }

    fn read_post(self, title: String) -> Option<Post> {
        // if let statement - if pattern returned by search_by_title
        // matches Some(post), if evaluates to true
        // Some(post) is evaluating if there is a value in post,
        // or is post equal to None
        if let Some(post) = search_by_title(self, title) {
            return Some(post);
        } else {
            return None;
        }
    }

    fn update_title(self, old_title: String, new_title: String) {
        if let Some(post) = search_by_title(self.clone(), old_title) {
            // do update something like find old post, delete post,
            // reinsert post with new title
            // return some form of success statement
            self.delete_post();
        } else {
            // post to be updated was not found, return
            // some sort of error statement
        }
    }

    fn update_author() {

    }

    fn update_body() {

    }

    fn update_labels() {

    }

    fn delete_post(self) {

    }
}

fn search_by_title(blog: Blog, title: String) -> Option<Post> {
    for post in blog.posts {
        if title == post.title {
            return Some(post);
        }
    }
    return None;
}

fn search_by_author(author: String) {

}

fn search_by_label(label: String) {

}

fn search_by_body(body: String) {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
