use chrono::{prelude::*, DateTime};
use std::collections::HashMap;

pub fn format_date_in_language(date: &DateTime<Local>, lang: &str) -> String {
    let months_en = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let months_es = vec![
        "Enero",
        "Febrero",
        "Marzo",
        "Abril",
        "Mayo",
        "Junio",
        "Julio",
        "Agosto",
        "Septiembre",
        "Octubre",
        "Noviembre",
        "Diciembre",
    ];

    let mut days_es: HashMap<String, String> = HashMap::new();
    days_es.insert("Mon".to_string(), "Lun".to_string());
    days_es.insert("Tue".to_string(), "Mar".to_string());
    days_es.insert("Wed".to_string(), "Mié".to_string());
    days_es.insert("Thu".to_string(), "Jue".to_string());
    days_es.insert("Fri".to_string(), "Vie".to_string());
    days_es.insert("Sat".to_string(), "Sab".to_string());
    days_es.insert("Sun".to_string(), "Dom".to_string());

    let mut months_map = HashMap::new();
    months_map.insert("en", months_en.clone());
    months_map.insert("es", months_es);

    let month = date.month() as usize - 1;
    let month_name = months_map.get(lang).unwrap_or(&months_en)[month];

    let weekday_name = if lang == "en" {
        &date.weekday().to_string()
    } else {
        match days_es.get(&date.weekday().to_string()) {
            Some(day) => &day,
            None => &date.weekday().to_string(),
        }
    };

    format!(
        "{}, {} de {} de {}, {}:{}",
        weekday_name,
        date.day().to_string(),
        month_name,
        date.year(),
        date.hour(),
        date.minute(),
    )
}
