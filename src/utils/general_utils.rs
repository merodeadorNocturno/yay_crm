use handlebars::{Handlebars, RenderError};
use log::info;

use serde::Serialize;
use uuid::Uuid;

pub fn render_container<T: Serialize>(
    template_path: &str,
    object_to_render: &T,
) -> Result<String, RenderError> {
    info!("Render Container");
    let handlebars = Handlebars::new();
    let render_ok = handlebars.render_template(template_path, object_to_render)?;

    Ok(render_ok)
}

pub fn get_uuid() -> String {
    let mut buffer = Uuid::encode_buffer();
    let new_id = Uuid::new_v4().simple().encode_lower(&mut buffer);

    String::from(new_id)
}

// pub fn create_select_query(table_name: &str, search_by: &str, where_item_equals: &str) -> String {
//     format!(
//         "SELECT {0} FROM {1} WHERE {0} = '{2}'",
//         search_by, table_name, where_item_equals
//     )
// }
