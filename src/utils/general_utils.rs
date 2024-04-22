// use std::fmt::Debug;

// use handlebars::{Handlebars, RenderError};
// use log::{debug, info};

// use serde::Serialize;
use crate::models::{
    sales_model::{SalesFunnel, SalesFunnelTag, ServicesOffered, ServicesOfferedTag},
    users_model::{Roles, RolesTag},
};
use uuid::Uuid;

// pub const DEFAULT_PASSWORD: &[u8; 248] = b"rq^FK?&!5.UVG*+;@!SnE_<gxjZB+A0#0<aU*:x~LOK/,hlQW]Y2oB8+!wxZysG?~{!_I!XbA?_|xta6.nL6k;)cgtIjhLlYp/t7ulx7.[W]%<_5Y|w8MJffH@-Sn]!f9Bz~ivHa8wYvRP:;Wzr2%gv_dbjr&n.UQC$;3|%{qDvKy=IA!p.7g<[0o:qim3!M!x*Sg1!B}5cyN#+!wl5qT8t!@4il]|mjK|xP[!cFMX!F&G!Q!#6,!!c#";
handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

// pub fn render_container<T: Serialize + Debug>(
//     template_path: &str,
//     object_to_render: &T,
// ) -> Result<String, RenderError> {
//     info!("Render Container");
//     let handlebars = Handlebars::new();
//     let render_ok = match handlebars.render_template(template_path, object_to_render) {
//         Ok(render_good) => Ok(render_good),
//         Err(e) => {
//             debug!("This is an error in render_container: {:?}", e.to_string());
//             Err(e)
//         }
//     };

//     render_ok
// }

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

pub fn get_roles_tag() -> Vec<RolesTag> {
    let roles_tag: Vec<RolesTag> = vec![
        RolesTag {
            value: Roles::EDITOR,
            text: Roles::EDITOR.to_string(),
            selected: false,
        },
        RolesTag {
            value: Roles::ADMIN,
            text: Roles::ADMIN.to_string(),
            selected: false,
        },
    ];

    roles_tag
}

pub fn get_options_and_services() -> (Vec<ServicesOfferedTag>, Vec<SalesFunnelTag>) {
    let services_tag: Vec<ServicesOfferedTag> = vec![
        ServicesOfferedTag {
            value: ServicesOffered::BRANDING,
            text: ServicesOffered::BRANDING.to_string(),
            selected: false,
        },
        ServicesOfferedTag {
            value: ServicesOffered::WEBSERVICES,
            text: ServicesOffered::WEBSERVICES.to_string(),
            selected: false,
        },
        ServicesOfferedTag {
            value: ServicesOffered::DIGITALSTRATEGY,
            text: ServicesOffered::DIGITALSTRATEGY.to_string(),
            selected: false,
        },
        ServicesOfferedTag {
            value: ServicesOffered::ATTRACTIONOFNEWCLIENTS,
            text: ServicesOffered::ATTRACTIONOFNEWCLIENTS.to_string(),
            selected: false,
        },
        ServicesOfferedTag {
            value: ServicesOffered::SALESMANAGEMENT,
            text: ServicesOffered::SALESMANAGEMENT.to_string(),
            selected: false,
        },
    ];

    let funnel_tag: Vec<SalesFunnelTag> = vec![
        SalesFunnelTag {
            value: SalesFunnel::LOST,
            text: SalesFunnel::LOST.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::NEGOTIATION,
            text: SalesFunnel::NEGOTIATION.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::PROSPECT,
            text: SalesFunnel::PROSPECT.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::QUOTE,
            text: SalesFunnel::QUOTE.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::RESEARCH,
            text: SalesFunnel::RESEARCH.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::WIN,
            text: SalesFunnel::WIN.to_string(),
            selected: false,
        },
    ];

    (services_tag, funnel_tag)
}

pub fn create_option_tags_info_for_services_and_funnel(
    enterprise_services_offered: Vec<ServicesOffered>,
    enterprise_sales_funnel: SalesFunnel,
) -> (Vec<ServicesOfferedTag>, Vec<SalesFunnelTag>) {
    let (mut services_tag, mut funnel_tag) = get_options_and_services();

    let services_offered = enterprise_services_offered;
    let sales_funnel = enterprise_sales_funnel;

    for service in &mut services_tag {
        if services_offered.contains(&service.value) {
            service.selected = true;
        }
    }

    for funnel in &mut funnel_tag {
        if sales_funnel == funnel.value {
            funnel.selected = true;
        }
    }

    (services_tag, funnel_tag)
}

pub fn create_role_tags_for_users(user_role: Roles) -> Vec<RolesTag> {
    let mut roles_tag = get_roles_tag();

    // let my_roles = user_role;

    for role in &mut roles_tag {
        if user_role == role.value {
            role.selected = true;
        }
    }

    roles_tag
}
