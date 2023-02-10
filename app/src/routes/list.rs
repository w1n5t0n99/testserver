use actix_web::{get, Responder, HttpResponse, http::header::ContentType};
use actix_web_flash_messages::IncomingFlashMessages;
use actix_web_grants::proc_macro::has_permissions;
use sailfish::TemplateOnce;

use crate::utils::e500;
use crate::components::Link;
use crate::components::navbar::{NavBar, NavBarBuilder};
use crate::components::searchbar::{SearchBar, SearchBarBuilder};

#[derive(TemplateOnce)]
#[template(path = "list.stpl")]
struct ListPage<'a> {
    pub messages: Vec<&'a str>,
    pub navbar: NavBar,
    pub search_bar: SearchBar,
}

#[tracing::instrument( name = "List", skip_all)]
#[has_permissions("admin")]
#[get("/list")]
pub async fn list(flash_messages: IncomingFlashMessages) -> Result<impl Responder, actix_web::Error> {
    let messages: Vec<&str> = flash_messages.iter().map(|f| f.content()).collect();

    let navbar = NavBarBuilder::default()
        .username("admin".to_string())
        .email("admin@richmond-county.k12.va.us".to_string())
        .is_admin(true)
        .add_link(Link::Active { name: "Asset-Items".into(), url: "/web/list".into() })
        .add_link(Link::Normal { name: "User-Items".into(), url: "/web/home".into() })
        .add_link(Link::Disabled { name: "Schools".into(), url: "/web/home".into() })
        .add_link(Link::Disabled { name: "Rooms".into(), url: "/web/home".into() })
        .build()
        .map_err(e500)?;

    let search_bar = SearchBarBuilder::default()
        .title("Assets".to_string())
        .form_url("/web/list".to_string())
        .search_filter((None, vec!["all".to_string(), "assets".to_string(), "model".to_string(), "serial #".to_string()]))
        .add_link(Link::Normal { name: "Add".into(), url: "/web/home".into() })
        .add_link(Link::Disabled { name: "Upload".into(), url: "/web/home".into() })
        .build()
        .map_err(e500)?;
      
    let body = ListPage {
        messages,
        navbar,
        search_bar,
    }
    .render_once()
    .map_err(e500)?;
   
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
    
}