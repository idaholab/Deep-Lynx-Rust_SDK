/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// Transformation : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Transformation {
    #[serde(rename = "conditions")]
    pub conditions: Vec<crate::models::TransformationCondition>,
    #[serde(rename = "keys")]
    pub keys: Vec<crate::models::TransformationKey>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type_mapping_id", skip_serializing_if = "Option::is_none")]
    pub type_mapping_id: Option<String>,
    #[serde(rename = "metatype_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metatype_id: Option<Option<String>>,
    #[serde(rename = "metatype_relationship_pair_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metatype_relationship_pair_id: Option<Option<String>>,
    #[serde(rename = "origin_id_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_id_key: Option<Option<String>>,
    #[serde(rename = "destination_id_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_id_key: Option<Option<String>>,
    #[serde(rename = "unique_identifier_key", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub unique_identifier_key: Option<Option<String>>,
    #[serde(rename = "root_array", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub root_array: Option<Option<String>>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "archived")]
    pub archived: bool,
    #[serde(rename = "metatype_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metatype_name: Option<Option<String>>,
    #[serde(rename = "metatype_relationship_pair_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metatype_relationship_pair_name: Option<Option<String>>,
    #[serde(rename = "container_id", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "shape_hash", skip_serializing_if = "Option::is_none")]
    pub shape_hash: Option<String>,
    #[serde(rename = "data_source_id", skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
}

impl Transformation {
    /// 
    pub fn new(conditions: Vec<crate::models::TransformationCondition>, keys: Vec<crate::models::TransformationKey>, archived: bool) -> Transformation {
        Transformation {
            conditions,
            keys,
            id: None,
            type_mapping_id: None,
            metatype_id: None,
            metatype_relationship_pair_id: None,
            origin_id_key: None,
            destination_id_key: None,
            unique_identifier_key: None,
            root_array: None,
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            archived,
            metatype_name: None,
            metatype_relationship_pair_name: None,
            container_id: None,
            shape_hash: None,
            data_source_id: None,
        }
    }
}


