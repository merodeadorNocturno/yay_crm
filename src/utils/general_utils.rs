use std::fmt::Debug;

use handlebars::{Handlebars, RenderError};
use log::{debug, info};

use serde::Serialize;
use uuid::Uuid;

// pub const DEFAULT_PASSWORD: &[u8; 248] = b"rq^FK?&!5.UVG*+;@!SnE_<gxjZB+A0#0<aU*:x~LOK/,hlQW]Y2oB8+!wxZysG?~{!_I!XbA?_|xta6.nL6k;)cgtIjhLlYp/t7ulx7.[W]%<_5Y|w8MJffH@-Sn]!f9Bz~ivHa8wYvRP:;Wzr2%gv_dbjr&n.UQC$;3|%{qDvKy=IA!p.7g<[0o:qim3!M!x*Sg1!B}5cyN#+!wl5qT8t!@4il]|mjK|xP[!cFMX!F&G!Q!#6,!!c#";
handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

pub fn render_container<T: Serialize + Debug>(
    template_path: &str,
    object_to_render: &T,
) -> Result<String, RenderError> {
    info!("Render Container");
    let handlebars = Handlebars::new();
    let render_ok = match handlebars.render_template(template_path, object_to_render) {
        Ok(render_good) => Ok(render_good),
        Err(e) => {
            debug!("This is an error in render_container: {:?}", e.to_string());
            Err(e)
        }
    };

    render_ok
}

pub fn get_uuid() -> String {
    let mut buffer = Uuid::encode_buffer();
    let new_id = Uuid::new_v4().simple().encode_lower(&mut buffer);

    String::from(new_id)
}

// pub fn print_type<T>(_: &T) {
//     debug!(
//         "@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@ {:?}",
//         std::any::type_name::<T>()
//     );
// }

// pub fn create_select_query(table_name: &str, search_by: &str, where_item_equals: &str) -> String {
//     format!(
//         "SELECT {0} FROM {1} WHERE {0} = '{2}'",
//         search_by, table_name, where_item_equals
//     )
// }
