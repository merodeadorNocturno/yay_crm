use crate::models::{
    sales_model::{
        SalesFunnel, SalesFunnelTag, SchoolLevel, SchoolLevelTag, ServicesOffered,
        ServicesOfferedTag,
    },
    users_model::{Roles, RolesTag},
};
use uuid::Uuid;

handlebars_helper!(str_equal: |s1: String, s2: String| s1 == s2);

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
            value: SalesFunnel::PROSPECT,
            text: SalesFunnel::PROSPECT.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::RESEARCH,
            text: SalesFunnel::RESEARCH.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::NEED,
            text: SalesFunnel::NEED.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::QUOTE,
            text: SalesFunnel::QUOTE.to_string(),
            selected: false,
        },
        SalesFunnelTag {
            value: SalesFunnel::NEGOTIATION,
            text: SalesFunnel::NEGOTIATION.to_string(),
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

pub fn get_school_level_tags() -> Vec<SchoolLevelTag> {
    vec![
        SchoolLevelTag {
            value: SchoolLevel::NURSERY,
            text: SchoolLevel::NURSERY.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::KINDER,
            text: SchoolLevel::KINDER.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::ELEMENTARY,
            text: SchoolLevel::ELEMENTARY.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::MIDDLESCHOOL,
            text: SchoolLevel::MIDDLESCHOOL.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::HIGHSCHOOL,
            text: SchoolLevel::HIGHSCHOOL.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::TECHNICALSCHOOL,
            text: SchoolLevel::TECHNICALSCHOOL.to_string(),
            selected: false,
        },
        SchoolLevelTag {
            value: SchoolLevel::UNIVERSITY,
            text: SchoolLevel::UNIVERSITY.to_string(),
            selected: false,
        },
    ]
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

    for role in &mut roles_tag {
        if user_role == role.value {
            role.selected = true;
        }
    }

    roles_tag
}

pub fn create_school_level_tags(school_level: Vec<SchoolLevel>) -> Vec<SchoolLevelTag> {
    let mut level_tag = get_school_level_tags();

    for level in &mut level_tag {
        if school_level.contains(&level.value) {
            level.selected = true;
        }
    }

    level_tag
}
