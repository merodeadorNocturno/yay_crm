use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum SalesFunnel {
    LOST,
    NEGOTIATION,
    PROSPECT,
    QUOTE,
    RESEARCH,
    WIN,
}

impl fmt::Display for SalesFunnel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SalesFunnel::LOST => write!(f, "LOST"),
            // "<div class=\"notification is-small has-text-centered is-lost\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Lost</span></span></div>"),
            SalesFunnel::NEGOTIATION => write!(f, "NEGOTIATION"),
            // "<div class=\"notification is-small has-text-centered is-negociation\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Negociación</span></span></div>"),
            SalesFunnel::PROSPECT => write!(f, "PROSPECT"),
            // "<div class=\"notification is-small has-text-centered is-prospect\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Prospecto</span></span></div>"),
            SalesFunnel::QUOTE => write!(f, "QUOTE"),
            // "<div class=\"notification is-small has-text-centered is-quote\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Cotización</span></span></div>"),
            SalesFunnel::RESEARCH => write!(f, "RESEARCH"),
            // "<div class=\"notification is-small has-text-centered is-research\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Investigación</span></span></div>"),
            SalesFunnel::WIN => write!(f, "WIN"),
            // "<div class=\"notification is-small has-text-centered is-win\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Wing</span></span></div>"),
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
