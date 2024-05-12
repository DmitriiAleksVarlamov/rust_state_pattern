pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_content(&mut self, content: &str) {
        if let Some(s) = self.state.take() {
            s.add_content(self, content)
                .unwrap_or_else(|err| println!("Uoops, {:?}", err));
        }
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn get_content(&self) -> &str {
        self.state.as_ref().unwrap().get_content(self)
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_content(&self, post: &mut Post, content: &str) -> Result<(), &str>;
    fn get_content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PenndingReview { approvals: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_content(&self, post: &mut Post, content: &str) -> Result<(), &str> {
        post.content.push_str(content);
        Ok(())
    }
}

struct PenndingReview {
    approvals: u8,
}

impl State for PenndingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        let approvals = self.approvals;

        if approvals >= 2 {
            Box::new(Published {})
        } else {
            Box::new(PenndingReview { approvals })
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_content(&self, post: &mut Post, content: &str) -> Result<(), &str> {
        Err("Caannot add conntent, coz of State is not Draft")
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn get_content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_content(&self, post: &mut Post, content: &str) -> Result<(), &str> {
        Err("Caannot add conntent, coz of State is not Draft")
    }
}
