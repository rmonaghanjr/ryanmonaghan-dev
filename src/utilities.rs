const VIEWS_DIRECTORY: &str = "views";

pub fn get_view(view_name: &str) -> String {
    return format!("./{}/{}.html", VIEWS_DIRECTORY, view_name);
}
