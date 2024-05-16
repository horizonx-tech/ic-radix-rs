#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Method(Inner);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Inner {
    Get,
    Post,
}

impl Method {
    /// POST
    pub const POST: Method = Method(Inner::Post);
    pub const GET: Method = Method(Inner::Get);
}

#[cfg(test)]
impl Into<outer_reqwest::Method> for Method {
    fn into(self) -> outer_reqwest::Method {
        match self.0 {
            Inner::Get => outer_reqwest::Method::GET,
            Inner::Post => outer_reqwest::Method::POST,
        }
    }
}
