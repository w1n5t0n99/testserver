use actix_web::{get, Responder, HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use actix_web_grants::proc_macro::has_permissions;
use sailfish::TemplateOnce;

use crate::utils::e500;
use crate::components::navbar::{NavBar, NavBarBuilder, Link};

#[derive(TemplateOnce)]
#[template(path = "list.stpl")]
struct ListPage<'a> {
    pub messages: Vec<&'a str>,
    pub navbar: NavBar,
}

#[tracing::instrument( name = "List", skip_all)]
#[has_permissions("admin")]
#[get("/list")]
pub async fn list(flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let navbar = NavBarBuilder::default()
        .username("admin".to_string())
        .email("admin.richmond-county.k12.va.us".to_string())
        .is_admin(true)
        .add_link(Link::Active { name: "Asset-Items".into(), url: "/web/home".into() })
        .add_link(Link::Active { name: "User-Items".into(), url: "/web/home".into() })
        .build()
        .map_err(e500)?;

    let body = ListPage {
        messages,
        navbar,
    }
    .render_once()
    .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    
}