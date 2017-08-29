struct Blog {
    title: String,
    posts: Vec<Post>,
}

struct Post {
    title: String,
    author: String,
    body: String,
    labels: Option<Vec<String>>,
    comments: Option<Vec<Comment>>,
}

struct Comment {
    author: String,
    body: String,
}

impl Post {
    fn create() {

    }

    fn read() {

    }

    fn update() {

    }

    fn delete() {

    }
}

impl Comment {
    fn create() {

    }

    fn read() {

    }

    fn update() {

    }

    fn delete() {

    }
}

fn search_by_title() {

}

fn search_by_author() {

}

fn search_by_label() {

}

fn search_by_body() {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
