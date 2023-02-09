use super::Link;


#[derive(Debug, derive_builder::Builder)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct SearchBar {
    pub title: String,
    #[builder(setter(strip_option))]
    pub selected_search: Option<String>,
    #[builder(setter(strip_option))]
    pub search_filter_list: Option<Vec<String>>,
    #[builder(setter(strip_option))]
    pub selected_filter: Option<String>,
    #[builder(setter(each(name = "add_link")))]
    pub links: Vec<Link>,
}

impl SearchBarBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(ref sel) = self.selected_filter {

        }

        Ok(())
    }
}



