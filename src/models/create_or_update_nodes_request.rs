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
pub struct CreateOrUpdateNodesRequest {
    /// 
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "container_id")]
    pub container_id: String,
    /// Passing in just the original data id will attempt to update a node with the same composite id (data source id, metatype id, and original data id).
    #[serde(rename = "original_data_id", skip_serializing_if = "Option::is_none")]
    pub original_data_id: Option<String>,
    #[serde(rename = "data_source_id")]
    pub data_source_id: String,
    #[serde(rename = "metatype_id")]
    pub metatype_id: String,
    #[serde(rename = "properties", skip_serializing_if = "Option::is_none")]
    pub properties: Option<serde_json::Value>,
}

impl CreateOrUpdateNodesRequest {
    pub fn new(container_id: String, data_source_id: String, metatype_id: String) -> CreateOrUpdateNodesRequest {
        CreateOrUpdateNodesRequest {
            id: None,
            container_id,
            original_data_id: None,
            data_source_id,
            metatype_id,
            properties: None,
        }
    }
}


