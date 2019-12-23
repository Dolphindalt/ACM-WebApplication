extern crate lettre;
extern crate lettre_email;

use lettre::{SmtpClient, Transport};
use lettre_email::Email;
use crate::models::user::User;
use std::env;

pub fn send_verification_email(user: &User, verification_code: String) -> bool{
    let email_enabled: bool = env::var("EMAIL_ENABLED")
        .expect("Email Enabled must be set to true or false")
        .parse()
        .unwrap();
    if email_enabled {
        let from_email = env::var("EMAIL").expect("Email must be set");
        let site_url = env::var("SITE_URL").expect("Site url must be set");

        let link = format!("{}/register/{}/{}", site_url, verification_code, user.user_id.unwrap());
        let content = format!("Navigate to this page to complete the registration process: <a href={}>{}</a>",
            &link, &link);

        let email = Email::builder()
            .to(user.email.clone())
            .from(from_email)
            .subject("Montana Tech ACM Site Registration")
            .html(content)
            .build()
            .unwrap();
        
        let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
        let result = mailer.send(email.into());

        result.is_ok()
    } else {
        true
    }
}