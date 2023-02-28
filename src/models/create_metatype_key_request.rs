/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateMetatypeKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "required", skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "property_name", skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "data_type", skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "cardinality", skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<i32>,
    #[serde(rename = "validation", skip_serializing_if = "Option::is_none")]
    pub validation: Option<Box<crate::models::KeyValidation>>,
    #[serde(rename = "unique", skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
    #[serde(rename = "options", skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "metatype_id")]
    pub metatype_id: String,
}

impl CreateMetatypeKeyRequest {
    pub fn new(name: String, description: String, metatype_id: String) -> CreateMetatypeKeyRequest {
        CreateMetatypeKeyRequest {
            name,
            required: None,
            property_name: None,
            description,
            data_type: None,
            cardinality: None,
            validation: None,
            unique: None,
            options: None,
            default_value: None,
            metatype_id,
        }
    }
}


