/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Event : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "container_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<Option<String>>,
    #[serde(rename = "data_source_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<Option<String>>,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "event_config", skip_serializing_if = "Option::is_none")]
    pub event_config: Option<serde_json::Value>,
    #[serde(rename = "event")]
    pub event: serde_json::Value,
    #[serde(rename = "processed", skip_serializing_if = "Option::is_none")]
    pub processed: Option<bool>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
}

impl Event {
    /// 
    pub fn new(event_type: String, event: serde_json::Value) -> Event {
        Event {
            id: None,
            container_id: None,
            data_source_id: None,
            event_type,
            event_config: None,
            event,
            processed: None,
            created_at: None,
            created_by: None,
        }
    }
}


