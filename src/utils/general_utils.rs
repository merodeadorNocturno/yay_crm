use rand::{thread_rng, Rng};
use uuid::Uuid;

use crate::models::{
    sales_model::{
        SalesFunnel, SalesFunnelTag, SchoolLevel, SchoolLevelTag, ServicesOffered,
        ServicesOfferedTag,
    },
    users_model::{Roles, RolesTag},
};

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
            icon: Some("fas fa-copyright".to_string()),
        },
        ServicesOfferedTag {
            value: ServicesOffered::WEBSERVICES,
            text: ServicesOffered::WEBSERVICES.to_string(),
            selected: false,
            icon: Some("fas fa-globe-americas".to_string()),
        },
        ServicesOfferedTag {
            value: ServicesOffered::DIGITALSTRATEGY,
            text: ServicesOffered::DIGITALSTRATEGY.to_string(),
            selected: false,
            icon: Some("fas fa-robot".to_string()),
        },
        ServicesOfferedTag {
            value: ServicesOffered::ATTRACTIONOFNEWCLIENTS,
            text: ServicesOffered::ATTRACTIONOFNEWCLIENTS.to_string(),
            selected: false,
            icon: Some("fas fa-magnet".to_string()),
        },
        ServicesOfferedTag {
            value: ServicesOffered::SALESMANAGEMENT,
            text: ServicesOffered::SALESMANAGEMENT.to_string(),
            selected: false,
            icon: Some("far fa-money-bill-alt".to_string()),
        },
    ];

    let funnel_tag: Vec<SalesFunnelTag> = vec![
        SalesFunnelTag {
            value: SalesFunnel::LOST,
            text: SalesFunnel::LOST.to_string(),
            selected: false,
            icon: Some("fas fa-trash".to_string()),
            css: Some("is-lost".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::PROSPECT,
            text: SalesFunnel::PROSPECT.to_string(),
            selected: false,
            icon: Some("fas fa-binoculars".to_string()),
            css: Some("is-prospect".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::RESEARCH,
            text: SalesFunnel::RESEARCH.to_string(),
            selected: false,
            icon: Some("fas fa-user-secret".to_owned()),
            css: Some("is-research".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::NEED,
            text: SalesFunnel::NEED.to_string(),
            selected: false,
            icon: Some("fas fa-drumstick-bite".to_string()),
            css: Some("has-background-primary-100 has-text-primary-100-invert".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::QUOTE,
            text: SalesFunnel::QUOTE.to_string(),
            selected: false,
            icon: Some("fas fa-money-check-alt".to_string()),
            css: Some("is-quote".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::NEGOTIATION,
            text: SalesFunnel::NEGOTIATION.to_string(),
            selected: false,
            icon: Some("fas fa-people-arrows".to_string()),
            css: Some("is-negotiation".to_string()),
        },
        SalesFunnelTag {
            value: SalesFunnel::WIN,
            text: SalesFunnel::WIN.to_string(),
            selected: false,
            icon: Some("fas fa-handshake".to_string()),
            css: Some("is-win".to_string()),
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

fn get_rnd_position(my_usize: usize) -> u8 {
    let mut random_generator = thread_rng();
    let rnd_number = random_generator.gen_range(0..my_usize as u8);

    rnd_number
}

pub fn shuffle_id(uuid: String) -> String {
    let uuid_len = uuid.len();
    let mut rnd_char: char = '0';
    let end_search_index: usize = 2;

    let rnd_index = get_rnd_position(uuid_len);
    let rnd_pos = get_rnd_position(end_search_index);

    let init_or_end = rnd_pos * (uuid_len as u8 - 1);

    let mut vec_of_chars: Vec<char> = uuid.chars().collect();

    if let Some(my_char) = vec_of_chars.get(rnd_index.clone() as usize) {
        rnd_char = my_char.to_owned();
    }

    vec_of_chars.remove(rnd_index as usize);
    vec_of_chars.insert(init_or_end as usize, rnd_char);

    vec_of_chars.iter().collect()
}
