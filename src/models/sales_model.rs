use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SalesFunnel {
    LOST,
    NEED,
    NEGOTIATION,
    PROSPECT,
    QUOTE,
    RESEARCH,
    WIN,
}

impl fmt::Display for SalesFunnel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SalesFunnel::LOST => write!(f, "Perdido"),
            SalesFunnel::NEGOTIATION => write!(f, "Negociación"),
            SalesFunnel::NEED => write!(f, "Necesidad"),
            SalesFunnel::PROSPECT => write!(f, "Prospecto"),
            SalesFunnel::QUOTE => write!(f, "Cotización"),
            SalesFunnel::RESEARCH => write!(f, "Investigación"),
            SalesFunnel::WIN => write!(f, "Ganado"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SalesFunnelTag {
    pub value: SalesFunnel,
    pub text: String,
    pub selected: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum ServicesOffered {
    BRANDING,
    WEBSERVICES,
    DIGITALSTRATEGY,
    ATTRACTIONOFNEWCLIENTS,
    SALESMANAGEMENT,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ServicesOfferedTag {
    pub value: ServicesOffered,
    pub text: String,
    pub selected: bool,
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
