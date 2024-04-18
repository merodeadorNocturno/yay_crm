use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
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
        SalesFunnel::LOST => write!(f, "<div class=\"notification is-small has-text-centered is-lost\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Lost</span></span></div>"),
        SalesFunnel::NEGOTIATION => write!(f, "<div class=\"notification is-small has-text-centered is-negociation\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Negociación</span></span></div>"),
        SalesFunnel::PROSPECT => write!(f, "<div class=\"notification is-small has-text-centered is-prospect\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Prospecto</span></span></div>"),
        SalesFunnel::QUOTE => write!(f, "<div class=\"notification is-small has-text-centered is-quote\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Cotización</span></span></div>"),
        SalesFunnel::RESEARCH => write!(f, "<div class=\"notification is-small has-text-centered is-research\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Investigación</span></span></div>"),
        SalesFunnel::WIN => write!(f, "<div class=\"notification is-small has-text-centered is-win\"><span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-trash\"></i><span class=\"tooltiptext\">Wing</span></span></div>"),
      }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServicesOffered {
    BRANDING,
    WEBSERVICES,
    DIGITALSTRATEGY,
    ATTRACTIONOFNEWCLIENTS,
    SALESMANAGEMENT,
}

// impl fmt::Display for ServicesOffered {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ServicesOffered::BRANDING => write!(f, "<span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-copyright\"></i><span class=\"tooltiptext\">Branding</span></span>"),
//             ServicesOffered::WEBSERVICES => write!(f, "<span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-globe-americas\"></i><span class=\"tooltiptext\">Web Services</span></span>"),
//             ServicesOffered::DIGITALSTRATEGY => write!(f, "<span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-robot\"></i><span class=\"tooltiptext\">Digital Strategy</span></span>"),
//             ServicesOffered::ATTRACTIONOFNEWCLIENTS => write!(f, "<span class=\"icon is-small is-left tooltip\"><i class=\"fas fa-magnet\"></i><span class=\"tooltiptext\">Attract new clients</span></span>"),
//             ServicesOffered::SALESMANAGEMENT => write!(f, "<span class=\"icon is-small is-left tooltip\"><i class=\"far fa-money-bill-alt\"></i><span class=\"tooltiptext\">Sales Management</span></span>"),
//         }
//     }
// }
