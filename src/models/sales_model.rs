use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SalesFunnel {
    PROSPECT,
    RESEARCH,
    QUOTE,
    NEGOTIATION,
    WIN,
    LOST,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ServicesOffered {
    BRANDING,
    WEBSERVICES,
    DIGITALSTRATEGY,
    ATTRACTIONOFNEWCLIENTS,
    SALESMANAGEMENT,
}
