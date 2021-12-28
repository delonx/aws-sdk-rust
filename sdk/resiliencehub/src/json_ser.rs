// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_draft_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddDraftAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.app_arn {
        object.key("appArn").string(var_1);
    }
    if let Some(var_2) = &input.resource_mappings {
        let mut array_3 = object.key("resourceMappings").start_array();
        for item_4 in var_2 {
            {
                let mut object_5 = array_3.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource_mapping(
                    &mut object_5,
                    item_4,
                )?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_6) = &input.client_token {
        object.key("clientToken").string(var_6);
    }
    if let Some(var_7) = &input.description {
        object.key("description").string(var_7);
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8);
    }
    if let Some(var_9) = &input.policy_arn {
        object.key("policyArn").string(var_9);
    }
    if let Some(var_10) = &input.tags {
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12).string(value_13);
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_recommendation_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRecommendationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_14) = &input.assessment_arn {
        object.key("assessmentArn").string(var_14);
    }
    if let Some(var_15) = &input.bucket_name {
        object.key("bucketName").string(var_15);
    }
    if let Some(var_16) = &input.client_token {
        object.key("clientToken").string(var_16);
    }
    if let Some(var_17) = &input.format {
        object.key("format").string(var_17.as_str());
    }
    if let Some(var_18) = &input.name {
        object.key("name").string(var_18);
    }
    if let Some(var_19) = &input.recommendation_ids {
        let mut array_20 = object.key("recommendationIds").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21);
            }
        }
        array_20.finish();
    }
    if let Some(var_22) = &input.recommendation_types {
        let mut array_23 = object.key("recommendationTypes").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.tags {
        let mut object_26 = object.key("tags").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27).string(value_28);
            }
        }
        object_26.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_29) = &input.client_token {
        object.key("clientToken").string(var_29);
    }
    if let Some(var_30) = &input.data_location_constraint {
        object.key("dataLocationConstraint").string(var_30.as_str());
    }
    if let Some(var_31) = &input.policy {
        let mut object_32 = object.key("policy").start_object();
        for (key_33, value_34) in var_31 {
            {
                let mut object_35 = object_32.key(key_33.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_failure_policy(
                    &mut object_35,
                    value_34,
                )?;
                object_35.finish();
            }
        }
        object_32.finish();
    }
    if let Some(var_36) = &input.policy_description {
        object.key("policyDescription").string(var_36);
    }
    if let Some(var_37) = &input.policy_name {
        object.key("policyName").string(var_37);
    }
    if let Some(var_38) = &input.tags {
        let mut object_39 = object.key("tags").start_object();
        for (key_40, value_41) in var_38 {
            {
                object_39.key(key_40).string(value_41);
            }
        }
        object_39.finish();
    }
    if let Some(var_42) = &input.tier {
        object.key("tier").string(var_42.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_43) = &input.app_arn {
        object.key("appArn").string(var_43);
    }
    if let Some(var_44) = &input.client_token {
        object.key("clientToken").string(var_44);
    }
    if let Some(var_45) = &input.force_delete {
        object.key("forceDelete").boolean(*var_45);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_46) = &input.assessment_arn {
        object.key("assessmentArn").string(var_46);
    }
    if let Some(var_47) = &input.client_token {
        object.key("clientToken").string(var_47);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_recommendation_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteRecommendationTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_48) = &input.client_token {
        object.key("clientToken").string(var_48);
    }
    if let Some(var_49) = &input.recommendation_template_arn {
        object.key("recommendationTemplateArn").string(var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_50) = &input.client_token {
        object.key("clientToken").string(var_50);
    }
    if let Some(var_51) = &input.policy_arn {
        object.key("policyArn").string(var_51);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_52) = &input.app_arn {
        object.key("appArn").string(var_52);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.assessment_arn {
        object.key("assessmentArn").string(var_53);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_version_resources_resolution_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppVersionResourcesResolutionStatusInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_54) = &input.app_arn {
        object.key("appArn").string(var_54);
    }
    if let Some(var_55) = &input.app_version {
        object.key("appVersion").string(var_55);
    }
    if let Some(var_56) = &input.resolution_id {
        object.key("resolutionId").string(var_56);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_app_version_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAppVersionTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_57) = &input.app_arn {
        object.key("appArn").string(var_57);
    }
    if let Some(var_58) = &input.app_version {
        object.key("appVersion").string(var_58);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_draft_app_version_resources_import_status_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeDraftAppVersionResourcesImportStatusInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_59) = &input.app_arn {
        object.key("appArn").string(var_59);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_60) = &input.policy_arn {
        object.key("policyArn").string(var_60);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_import_resources_to_draft_app_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ImportResourcesToDraftAppVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_61) = &input.app_arn {
        object.key("appArn").string(var_61);
    }
    if let Some(var_62) = &input.source_arns {
        let mut array_63 = object.key("sourceArns").start_array();
        for item_64 in var_62 {
            {
                array_63.value().string(item_64);
            }
        }
        array_63.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_alarm_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAlarmRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_65) = &input.assessment_arn {
        object.key("assessmentArn").string(var_65);
    }
    if let Some(var_66) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_66).into()),
        );
    }
    if let Some(var_67) = &input.next_token {
        object.key("nextToken").string(var_67);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_component_compliances_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppComponentCompliancesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_68) = &input.assessment_arn {
        object.key("assessmentArn").string(var_68);
    }
    if let Some(var_69) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_69).into()),
        );
    }
    if let Some(var_70) = &input.next_token {
        object.key("nextToken").string(var_70);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_component_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppComponentRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_71) = &input.assessment_arn {
        object.key("assessmentArn").string(var_71);
    }
    if let Some(var_72) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_72).into()),
        );
    }
    if let Some(var_73) = &input.next_token {
        object.key("nextToken").string(var_73);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_74) = &input.app_arn {
        object.key("appArn").string(var_74);
    }
    if let Some(var_75) = &input.app_version {
        object.key("appVersion").string(var_75);
    }
    if let Some(var_76) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    if let Some(var_77) = &input.next_token {
        object.key("nextToken").string(var_77);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_78) = &input.app_arn {
        object.key("appArn").string(var_78);
    }
    if let Some(var_79) = &input.app_version {
        object.key("appVersion").string(var_79);
    }
    if let Some(var_80) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_80).into()),
        );
    }
    if let Some(var_81) = &input.next_token {
        object.key("nextToken").string(var_81);
    }
    if let Some(var_82) = &input.resolution_id {
        object.key("resolutionId").string(var_82);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_app_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAppVersionsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_83) = &input.app_arn {
        object.key("appArn").string(var_83);
    }
    if let Some(var_84) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    if let Some(var_85) = &input.next_token {
        object.key("nextToken").string(var_85);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_sop_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSopRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_86) = &input.assessment_arn {
        object.key("assessmentArn").string(var_86);
    }
    if let Some(var_87) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_87).into()),
        );
    }
    if let Some(var_88) = &input.next_token {
        object.key("nextToken").string(var_88);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_test_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTestRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.assessment_arn {
        object.key("assessmentArn").string(var_89);
    }
    if let Some(var_90) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_90).into()),
        );
    }
    if let Some(var_91) = &input.next_token {
        object.key("nextToken").string(var_91);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_unsupported_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListUnsupportedAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_92) = &input.app_arn {
        object.key("appArn").string(var_92);
    }
    if let Some(var_93) = &input.app_version {
        object.key("appVersion").string(var_93);
    }
    if let Some(var_94) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_94).into()),
        );
    }
    if let Some(var_95) = &input.next_token {
        object.key("nextToken").string(var_95);
    }
    if let Some(var_96) = &input.resolution_id {
        object.key("resolutionId").string(var_96);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_publish_app_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishAppVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_97) = &input.app_arn {
        object.key("appArn").string(var_97);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_draft_app_version_template_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutDraftAppVersionTemplateInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_98) = &input.app_arn {
        object.key("appArn").string(var_98);
    }
    if let Some(var_99) = &input.app_template_body {
        object.key("appTemplateBody").string(var_99);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_draft_app_version_resource_mappings_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveDraftAppVersionResourceMappingsInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_100) = &input.app_arn {
        object.key("appArn").string(var_100);
    }
    if let Some(var_101) = &input.app_registry_app_names {
        let mut array_102 = object.key("appRegistryAppNames").start_array();
        for item_103 in var_101 {
            {
                array_102.value().string(item_103);
            }
        }
        array_102.finish();
    }
    if let Some(var_104) = &input.logical_stack_names {
        let mut array_105 = object.key("logicalStackNames").start_array();
        for item_106 in var_104 {
            {
                array_105.value().string(item_106);
            }
        }
        array_105.finish();
    }
    if let Some(var_107) = &input.resource_group_names {
        let mut array_108 = object.key("resourceGroupNames").start_array();
        for item_109 in var_107 {
            {
                array_108.value().string(item_109);
            }
        }
        array_108.finish();
    }
    if let Some(var_110) = &input.resource_names {
        let mut array_111 = object.key("resourceNames").start_array();
        for item_112 in var_110 {
            {
                array_111.value().string(item_112);
            }
        }
        array_111.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_resolve_app_version_resources_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResolveAppVersionResourcesInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.app_arn {
        object.key("appArn").string(var_113);
    }
    if let Some(var_114) = &input.app_version {
        object.key("appVersion").string(var_114);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_start_app_assessment_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StartAppAssessmentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_115) = &input.app_arn {
        object.key("appArn").string(var_115);
    }
    if let Some(var_116) = &input.app_version {
        object.key("appVersion").string(var_116);
    }
    if let Some(var_117) = &input.assessment_name {
        object.key("assessmentName").string(var_117);
    }
    if let Some(var_118) = &input.client_token {
        object.key("clientToken").string(var_118);
    }
    if let Some(var_119) = &input.tags {
        let mut object_120 = object.key("tags").start_object();
        for (key_121, value_122) in var_119 {
            {
                object_120.key(key_121).string(value_122);
            }
        }
        object_120.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.tags {
        let mut object_124 = object.key("tags").start_object();
        for (key_125, value_126) in var_123 {
            {
                object_124.key(key_125).string(value_126);
            }
        }
        object_124.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_app_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAppInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_127) = &input.app_arn {
        object.key("appArn").string(var_127);
    }
    if let Some(var_128) = &input.clear_resiliency_policy_arn {
        object.key("clearResiliencyPolicyArn").boolean(*var_128);
    }
    if let Some(var_129) = &input.description {
        object.key("description").string(var_129);
    }
    if let Some(var_130) = &input.policy_arn {
        object.key("policyArn").string(var_130);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_resiliency_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResiliencyPolicyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_131) = &input.data_location_constraint {
        object
            .key("dataLocationConstraint")
            .string(var_131.as_str());
    }
    if let Some(var_132) = &input.policy {
        let mut object_133 = object.key("policy").start_object();
        for (key_134, value_135) in var_132 {
            {
                let mut object_136 = object_133.key(key_134.as_str()).start_object();
                crate::json_ser::serialize_structure_crate_model_failure_policy(
                    &mut object_136,
                    value_135,
                )?;
                object_136.finish();
            }
        }
        object_133.finish();
    }
    if let Some(var_137) = &input.policy_arn {
        object.key("policyArn").string(var_137);
    }
    if let Some(var_138) = &input.policy_description {
        object.key("policyDescription").string(var_138);
    }
    if let Some(var_139) = &input.policy_name {
        object.key("policyName").string(var_139);
    }
    if let Some(var_140) = &input.tier {
        object.key("tier").string(var_140.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource_mapping(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ResourceMapping,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_141) = &input.resource_name {
        object.key("resourceName").string(var_141);
    }
    if let Some(var_142) = &input.logical_stack_name {
        object.key("logicalStackName").string(var_142);
    }
    if let Some(var_143) = &input.app_registry_app_name {
        object.key("appRegistryAppName").string(var_143);
    }
    if let Some(var_144) = &input.resource_group_name {
        object.key("resourceGroupName").string(var_144);
    }
    if let Some(var_145) = &input.mapping_type {
        object.key("mappingType").string(var_145.as_str());
    }
    if let Some(var_146) = &input.physical_resource_id {
        let mut object_147 = object.key("physicalResourceId").start_object();
        crate::json_ser::serialize_structure_crate_model_physical_resource_id(
            &mut object_147,
            var_146,
        )?;
        object_147.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_failure_policy(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FailurePolicy,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    {
        object.key("rtoInSecs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rto_in_secs).into()),
        );
    }
    {
        object.key("rpoInSecs").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.rpo_in_secs).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_physical_resource_id(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::PhysicalResourceId,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_148) = &input.identifier {
        object.key("identifier").string(var_148);
    }
    if let Some(var_149) = &input.r#type {
        object.key("type").string(var_149.as_str());
    }
    if let Some(var_150) = &input.aws_region {
        object.key("awsRegion").string(var_150);
    }
    if let Some(var_151) = &input.aws_account_id {
        object.key("awsAccountId").string(var_151);
    }
    Ok(())
}