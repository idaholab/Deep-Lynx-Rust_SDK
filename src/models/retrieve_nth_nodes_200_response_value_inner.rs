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
pub struct RetrieveNthNodes200ResponseValueInner {
    #[serde(rename = "origin_properties", skip_serializing_if = "Option::is_none")]
    pub origin_properties: Option<Box<crate::models::RetrieveNthNodes200ResponseValueInnerOriginProperties>>,
    #[serde(rename = "edge_properties", skip_serializing_if = "Option::is_none")]
    pub edge_properties: Option<serde_json::Value>,
    #[serde(rename = "destination_properties", skip_serializing_if = "Option::is_none")]
    pub destination_properties: Option<Box<crate::models::RetrieveNthNodes200ResponseValueInnerOriginProperties>>,
    #[serde(rename = "origin_id")]
    pub origin_id: String,
    #[serde(rename = "origin_container_id")]
    pub origin_container_id: String,
    #[serde(rename = "origin_data_source_id", skip_serializing_if = "Option::is_none")]
    pub origin_data_source_id: Option<String>,
    #[serde(rename = "origin_import_data_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_import_data_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_data_staging_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_data_staging_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_type_mapping_transformation_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_type_mapping_transformation_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_original_data_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_original_data_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_created_at", skip_serializing_if = "Option::is_none")]
    pub origin_created_at: Option<String>,
    #[serde(rename = "origin_modified_at", skip_serializing_if = "Option::is_none")]
    pub origin_modified_at: Option<String>,
    #[serde(rename = "origin_deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub origin_deleted_at: Option<Option<serde_json::Value>>,
    #[serde(rename = "origin_created_by", skip_serializing_if = "Option::is_none")]
    pub origin_created_by: Option<String>,
    #[serde(rename = "origin_modified_by", skip_serializing_if = "Option::is_none")]
    pub origin_modified_by: Option<String>,
    #[serde(rename = "origin_metatype_name", skip_serializing_if = "Option::is_none")]
    pub origin_metatype_name: Option<String>,
    #[serde(rename = "edge_id")]
    pub edge_id: String,
    #[serde(rename = "edge_container_id")]
    pub edge_container_id: String,
    #[serde(rename = "edge_relationship_pair_id", skip_serializing_if = "Option::is_none")]
    pub edge_relationship_pair_id: Option<String>,
    #[serde(rename = "edge_data_source_id", skip_serializing_if = "Option::is_none")]
    pub edge_data_source_id: Option<String>,
    #[serde(rename = "edge_import_data_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_import_data_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "edge_data_staging_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_data_staging_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "edge_type_mapping_transformation_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_type_mapping_transformation_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "edge_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "edge_created_at", skip_serializing_if = "Option::is_none")]
    pub edge_created_at: Option<String>,
    #[serde(rename = "edge_modified_at", skip_serializing_if = "Option::is_none")]
    pub edge_modified_at: Option<String>,
    #[serde(rename = "edge_deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub edge_deleted_at: Option<Option<serde_json::Value>>,
    #[serde(rename = "edge_modified_by", skip_serializing_if = "Option::is_none")]
    pub edge_modified_by: Option<String>,
    #[serde(rename = "edge_created_by", skip_serializing_if = "Option::is_none")]
    pub edge_created_by: Option<String>,
    #[serde(rename = "destination_id")]
    pub destination_id: String,
    #[serde(rename = "destination_container_id")]
    pub destination_container_id: String,
    #[serde(rename = "destination_data_source_id", skip_serializing_if = "Option::is_none")]
    pub destination_data_source_id: Option<String>,
    #[serde(rename = "destination_import_data_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_import_data_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_data_staging_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_data_staging_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_type_mapping_transformation_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_type_mapping_transformation_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_original_data_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_original_data_id: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_created_at", skip_serializing_if = "Option::is_none")]
    pub destination_created_at: Option<String>,
    #[serde(rename = "destination_modified_at", skip_serializing_if = "Option::is_none")]
    pub destination_modified_at: Option<String>,
    #[serde(rename = "destination_deleted_at", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub destination_deleted_at: Option<Option<serde_json::Value>>,
    #[serde(rename = "destination_created_by", skip_serializing_if = "Option::is_none")]
    pub destination_created_by: Option<String>,
    #[serde(rename = "destination_modified_by", skip_serializing_if = "Option::is_none")]
    pub destination_modified_by: Option<String>,
    #[serde(rename = "destination_metatype_name", skip_serializing_if = "Option::is_none")]
    pub destination_metatype_name: Option<String>,
    #[serde(rename = "lvl")]
    pub lvl: f32,
    #[serde(rename = "metatype_id", skip_serializing_if = "Option::is_none")]
    pub metatype_id: Option<Box<crate::models::RetrieveNthNodes200ResponseValueInnerMetatypeId>>,
    #[serde(rename = "destination_metatype_id", skip_serializing_if = "Option::is_none")]
    pub destination_metatype_id: Option<String>,
}

impl RetrieveNthNodes200ResponseValueInner {
    pub fn new(origin_id: String, origin_container_id: String, edge_id: String, edge_container_id: String, destination_id: String, destination_container_id: String, lvl: f32) -> RetrieveNthNodes200ResponseValueInner {
        RetrieveNthNodes200ResponseValueInner {
            origin_properties: None,
            edge_properties: None,
            destination_properties: None,
            origin_id,
            origin_container_id,
            origin_data_source_id: None,
            origin_import_data_id: None,
            origin_data_staging_id: None,
            origin_type_mapping_transformation_id: None,
            origin_original_data_id: None,
            origin_metadata: None,
            origin_created_at: None,
            origin_modified_at: None,
            origin_deleted_at: None,
            origin_created_by: None,
            origin_modified_by: None,
            origin_metatype_name: None,
            edge_id,
            edge_container_id,
            edge_relationship_pair_id: None,
            edge_data_source_id: None,
            edge_import_data_id: None,
            edge_data_staging_id: None,
            edge_type_mapping_transformation_id: None,
            edge_metadata: None,
            edge_created_at: None,
            edge_modified_at: None,
            edge_deleted_at: None,
            edge_modified_by: None,
            edge_created_by: None,
            destination_id,
            destination_container_id,
            destination_data_source_id: None,
            destination_import_data_id: None,
            destination_data_staging_id: None,
            destination_type_mapping_transformation_id: None,
            destination_original_data_id: None,
            destination_metadata: None,
            destination_created_at: None,
            destination_modified_at: None,
            destination_deleted_at: None,
            destination_created_by: None,
            destination_modified_by: None,
            destination_metatype_name: None,
            lvl,
            metatype_id: None,
            destination_metatype_id: None,
        }
    }
}

