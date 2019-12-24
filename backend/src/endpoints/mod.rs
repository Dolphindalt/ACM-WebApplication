pub mod event;
pub mod login;
pub mod officers;

#[cfg(test)]
mod test {

    use rocket::http::Header;

    const DEFAULT_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6IntcInVzZXJfaWRcIjoyLFwicGFzc3dvcmRfaWRcIjoyLFwidXNlcl90eXBlXCI6MSxcImZpcnN0X25hbWVcIjpcIkZyYW5rXCIsXCJsYXN0X25hbWVcIjpcIkFja2VybWFuXCIsXCJlbWFpbFwiOlwiZmFja2VybWFuQG10ZWNoLmVkdVwiLFwicG9pbnRzXCI6MC4wfSIsImF1ZCI6bnVsbCwiZXhwIjpudWxsLCJuYmYiOm51bGwsImlhdCI6bnVsbCwianRpIjpudWxsfQ.zQUKkdnFZrgXrck3U4DuMfHzcOCJoaFAHiUCgPJhNKE";
    const ADMIN_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOm51bGwsInN1YiI6IntcInVzZXJfaWRcIjoxLFwicGFzc3dvcmRfaWRcIjoxLFwidXNlcl90eXBlXCI6NSxcImZpcnN0X25hbWVcIjpcIkplZmZcIixcImxhc3RfbmFtZVwiOlwiQnJhdW5cIixcImVtYWlsXCI6XCJqYnJhdW5AbXRlY2guZWR1XCIsXCJwb2ludHNcIjowLjB9IiwiYXVkIjpudWxsLCJleHAiOm51bGwsIm5iZiI6bnVsbCwiaWF0IjpudWxsLCJqdGkiOm51bGx9.QIjblAnzDftIhVuMnoMA4bGdJgW4H1B6U9pQ5OTG/R8";

    pub fn admin_authorization_header() -> Header<'static> {
        Header::new("Authentication", String::from(ADMIN_TOKEN))
    }

    pub fn default_authorization_header() -> Header<'static> {
        Header::new("Authentication", String::from(DEFAULT_TOKEN))
    }
}