/*
 * Deep Lynx
 *
 * The construction of megaprojects has consistently demonstrated challenges for project managers in regard to meeting cost, schedule, and performance requirements. Megaproject construction challenges are common place within megaprojects with many active projects in the United States failing to meet cost and schedule efforts by significant margins. Currently, engineering teams operate in siloed tools and disparate teams where connections across design, procurement, and construction systems are translated manually or over brittle point-to-point integrations. The manual nature of data exchange increases the risk of silent errors in the reactor design, with each silent error cascading across the design. These cascading errors lead to uncontrollable risk during construction, resulting in significant delays and cost overruns. Deep Lynx allows for an integrated platform during design and operations of mega projects. The Deep Lynx Core API delivers a few main features. 1. Provides a set of methods and endpoints for manipulating data in an object oriented database. This allows us to store complex datatypes as records and then to compile them into actual, modifiable objects at run-time. Users can store taxonomies or ontologies in a readable format. 2. Provides methods for storing and retrieving data in a graph database. This data is structured and validated against the aformentioned object oriented database before storage.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RelationshipPair : 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RelationshipPair {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "relationship_type", skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<String>,
    #[serde(rename = "relationship_id", skip_serializing_if = "Option::is_none")]
    pub relationship_id: Option<String>,
    #[serde(rename = "origin_metatype_id", skip_serializing_if = "Option::is_none")]
    pub origin_metatype_id: Option<String>,
    #[serde(rename = "destination_metatype_id", skip_serializing_if = "Option::is_none")]
    pub destination_metatype_id: Option<String>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "archived", skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(rename = "container_id", skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "modified_at", skip_serializing_if = "Option::is_none")]
    pub modified_at: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "modified_by", skip_serializing_if = "Option::is_none")]
    pub modified_by: Option<String>,
    #[serde(rename = "origin_metatype_name", skip_serializing_if = "Option::is_none")]
    pub origin_metatype_name: Option<String>,
    #[serde(rename = "destination_metatype_name", skip_serializing_if = "Option::is_none")]
    pub destination_metatype_name: Option<String>,
    #[serde(rename = "relationship_pair_name", skip_serializing_if = "Option::is_none")]
    pub relationship_pair_name: Option<String>,
    #[serde(rename = "destination_metatype", skip_serializing_if = "Option::is_none")]
    pub destination_metatype: Option<Box<crate::models::RelationshipPairDestinationMetatype>>,
    #[serde(rename = "origin_metatype", skip_serializing_if = "Option::is_none")]
    pub origin_metatype: Option<Box<crate::models::RelationshipPairDestinationMetatype>>,
    #[serde(rename = "relationship", skip_serializing_if = "Option::is_none")]
    pub relationship: Option<Box<crate::models::RelationshipPairDestinationMetatype>>,
}

impl RelationshipPair {
    /// 
    pub fn new(name: String, id: String) -> RelationshipPair {
        RelationshipPair {
            name,
            description: None,
            relationship_type: None,
            relationship_id: None,
            origin_metatype_id: None,
            destination_metatype_id: None,
            id,
            archived: None,
            container_id: None,
            created_at: None,
            modified_at: None,
            created_by: None,
            modified_by: None,
            origin_metatype_name: None,
            destination_metatype_name: None,
            relationship_pair_name: None,
            destination_metatype: None,
            origin_metatype: None,
            relationship: None,
        }
    }
}

