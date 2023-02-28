/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Exporter : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Exporter {
    #[serde(rename = "adapter")]
    pub adapter: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "config")]
    pub config: Box<crate::models::ExporterConfig>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "container_id")]
    pub container_id: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "modified_at")]
    pub modified_at: String,
    #[serde(rename = "created_by")]
    pub created_by: String,
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    #[serde(rename = "status_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<Option<String>>,
    #[serde(rename = "destination_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<Option<String>>,
}

impl Exporter {
    /// 
    pub fn new(adapter: String, status: String, config: crate::models::ExporterConfig, id: String, container_id: String, created_at: String, modified_at: String, created_by: String, modified_by: String) -> Exporter {
        Exporter {
            adapter,
            status,
            config: Box::new(config),
            id,
            container_id,
            created_at,
            modified_at,
            created_by,
            modified_by,
            status_message: None,
            destination_type: None,
        }
    }
}

