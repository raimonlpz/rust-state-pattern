fn main() {
    let mut post = crate::Post::new();

    post.add_text("I ate a salad for lunch today");

    post.request_review();
    if let Some(s) = &post.state {
        let act = &s.action();
        println!("{}", act);
    }

    post.reject();
    if let Some(s) = &post.state {
        let act = &s.action();
        println!("{}", act);
    }

    post.request_review();
    if let Some(s) = &post.state {
        let act = &s.action();
        println!("{}", act);
    }

    post.approve();
    post.approve();

    if let Some(s) = &post.state {
        let act = &s.action();
        println!("{}", act);
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    approval_counter: u8,
    is_edtbl: bool,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {
                action: String::from("Drafting..."),
            })),
            content: String::new(),
            approval_counter: 0,
            is_edtbl: true,
        }
    }
    pub fn add_text(&mut self, text: &str) {
        if self.is_edtbl {
            self.content.push_str(text);
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }
    pub fn request_review(&mut self) {
        if self.is_edtbl {
            self.toggle_draft();
        }
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }
    pub fn approve(&mut self) {
        let appr_c = &mut self.approval_counter;
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(appr_c))
        }
    }
    pub fn reject(&mut self) {
        self.toggle_draft();
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
    pub fn toggle_draft(&mut self) {
        self.is_edtbl = !self.is_edtbl;
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, c: &mut u8) -> Box<dyn State>;
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn action(&self) -> &str;
}

struct Draft {
    pub action: String,
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            action: String::from("Reviewing..."),
        })
    }
    fn approve(self: Box<Self>, _: &mut u8) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn action(&self) -> &str {
        &self.action
    }
}

struct PendingReview {
    pub action: String,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>, c: &mut u8) -> Box<dyn State> {
        *c += 1;
        match c {
            2 => Box::new(Published {
                action: String::from("Publishing..."),
            }),
            _ => Box::new(PendingReview {
                action: String::from("Reviewing..."),
            }),
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {
            action: String::from("Drafting..."),
        })
    }
    fn action(&self) -> &str {
        &self.action
    }
}

struct Published {
    pub action: String,
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>, _: &mut u8) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn action(&self) -> &str {
        &self.action
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut post = crate::Post::new();

        post.add_text("I ate a salad for lunch today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.add_text("...amazing!!");
        post.approve();
        post.approve();
        post.add_text("I ate more!!");
        assert_eq!("I ate a salad for lunch today", post.content());
    }
}
