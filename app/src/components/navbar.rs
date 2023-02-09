

static NAV_LINKS: &'static [(&'static str, &'static str)] = &[("Asset-Items", "/"), ("User-Items", "/")];

#[derive(Debug, Clone)]
pub enum Link {
    Active {name: String, url: String},
    Disabled {name: String, url: String},
    Normal {name: String, url: String},
}

#[derive(Debug, derive_builder::Builder)]
#[builder(setter(into))]
pub struct NavBar {
    pub username: String,
    pub email: String,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<Link>,
    pub is_admin: bool,
}

// NavBar::from_client(client: &auth::Client, active_link: ActiveLink)