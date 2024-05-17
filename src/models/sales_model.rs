use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SalesFunnel {
    LOST,
    PROSPECT,
    RESEARCH,
    NEED,
    QUOTE,
    NEGOTIATION,
    WIN,
}

impl fmt::Display for SalesFunnel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SalesFunnel::LOST => write!(f, "Perdido 0%"),
            SalesFunnel::PROSPECT => write!(f, "Prospecto 0%"),
            SalesFunnel::RESEARCH => write!(f, "Investigación 10%"),
            SalesFunnel::NEED => write!(f, "Necesidad 20%"),
            SalesFunnel::QUOTE => write!(f, "Cotización 50%"),
            SalesFunnel::NEGOTIATION => write!(f, "Negociación 80%"),
            SalesFunnel::WIN => write!(f, "Ganado 100%"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SalesFunnelTag {
    pub value: SalesFunnel,
    pub text: String,
    pub selected: bool,
    pub icon: Option<String>,
    pub css: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ServicesOffered {
    BRANDING,
    WEBSERVICES,
    DIGITALSTRATEGY,
    ATTRACTIONOFNEWCLIENTS,
    SALESMANAGEMENT,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ServicesOfferedTag {
    pub value: ServicesOffered,
    pub text: String,
    pub selected: bool,
    pub icon: Option<String>,
}

impl fmt::Display for ServicesOffered {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServicesOffered::BRANDING => write!(f, "BRANDING"),
            ServicesOffered::WEBSERVICES => write!(f, "WEB SERVICES"),
            ServicesOffered::DIGITALSTRATEGY => write!(f, "DIGITAL STRATEGY"),
            ServicesOffered::ATTRACTIONOFNEWCLIENTS => write!(f, "ATTRACTION OF NEW CLIENTS"),
            ServicesOffered::SALESMANAGEMENT => write!(f, "SALES MANAGEMENT"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IsDeleted {
    pub deleted: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SchoolLevel {
    NURSERY,
    KINDER,
    ELEMENTARY,
    MIDDLESCHOOL,
    HIGHSCHOOL,
    TECHNICALSCHOOL,
    UNIVERSITY,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SchoolLevelTag {
    pub value: SchoolLevel,
    pub text: String,
    pub selected: bool,
}

impl fmt::Display for SchoolLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SchoolLevel::NURSERY => write!(f, "Guardería"),
            SchoolLevel::KINDER => write!(f, "Kinder"),
            SchoolLevel::ELEMENTARY => write!(f, "Primaria"),
            SchoolLevel::MIDDLESCHOOL => write!(f, "Secundaria"),
            SchoolLevel::HIGHSCHOOL => write!(f, "Preparatoria"),
            SchoolLevel::TECHNICALSCHOOL => write!(f, "Escuela Técnica"),
            SchoolLevel::UNIVERSITY => write!(f, "Universidad"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GeneralTags<T> {
    pub section: T,
    pub funnel_tag: Vec<SalesFunnelTag>,
    pub services_tag: Vec<ServicesOfferedTag>,
}
