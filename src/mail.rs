use lettre::transport::smtp::response::Response;
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Address,
    SmtpTransport, Transport,
};
use tera::{Context, Tera};

pub struct HtmlMailer {
    pub creadentials: Credentials,
    pub smtp_host: String,
    pub template_egine: Tera,
}

impl HtmlMailer {
    pub fn send_email(
        &self,
        receiver_email: &str,
        template: &str,
        context: &Context,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let html_body = self.template_egine.render(template, context)?;

        let message = lettre::Message::builder()
            .subject("Cr8s digest")
            .from(lettre::message::Mailbox::new(
                Some("info@cr8s.com".to_string()),
                Address::new("info", "cr8s.com")?,
            ))
            .to(receiver_email.parse()?)
            .header(ContentType::TEXT_HTML)
            .body(html_body)?;

        let mailer = SmtpTransport::relay(&self.smtp_host)?
            .credentials(self.creadentials.to_owned())
            .build();

        mailer.send(&message).map_err(|e| e.into())
    }
}
