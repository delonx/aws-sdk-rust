// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_add_layer_version_permission_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddLayerVersionPermissionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("Action").string(var_1);
    }
    if let Some(var_2) = &input.organization_id {
        object.key("OrganizationId").string(var_2);
    }
    if let Some(var_3) = &input.principal {
        object.key("Principal").string(var_3);
    }
    if let Some(var_4) = &input.statement_id {
        object.key("StatementId").string(var_4);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_add_permission_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::AddPermissionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_5) = &input.action {
        object.key("Action").string(var_5);
    }
    if let Some(var_6) = &input.event_source_token {
        object.key("EventSourceToken").string(var_6);
    }
    if let Some(var_7) = &input.principal {
        object.key("Principal").string(var_7);
    }
    if let Some(var_8) = &input.revision_id {
        object.key("RevisionId").string(var_8);
    }
    if let Some(var_9) = &input.source_account {
        object.key("SourceAccount").string(var_9);
    }
    if let Some(var_10) = &input.source_arn {
        object.key("SourceArn").string(var_10);
    }
    if let Some(var_11) = &input.statement_id {
        object.key("StatementId").string(var_11);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_alias_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAliasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_12) = &input.description {
        object.key("Description").string(var_12);
    }
    if let Some(var_13) = &input.function_version {
        object.key("FunctionVersion").string(var_13);
    }
    if let Some(var_14) = &input.name {
        object.key("Name").string(var_14);
    }
    if let Some(var_15) = &input.routing_config {
        let mut object_16 = object.key("RoutingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_alias_routing_configuration(
            &mut object_16,
            var_15,
        )?;
        object_16.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_code_signing_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCodeSigningConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_17) = &input.allowed_publishers {
        let mut object_18 = object.key("AllowedPublishers").start_object();
        crate::json_ser::serialize_structure_crate_model_allowed_publishers(
            &mut object_18,
            var_17,
        )?;
        object_18.finish();
    }
    if let Some(var_19) = &input.code_signing_policies {
        let mut object_20 = object.key("CodeSigningPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_code_signing_policies(
            &mut object_20,
            var_19,
        )?;
        object_20.finish();
    }
    if let Some(var_21) = &input.description {
        object.key("Description").string(var_21);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_event_source_mapping_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateEventSourceMappingInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_22) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    if let Some(var_23) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_23);
    }
    if let Some(var_24) = &input.destination_config {
        let mut object_25 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_25,
            var_24,
        )?;
        object_25.finish();
    }
    if let Some(var_26) = &input.enabled {
        object.key("Enabled").boolean(*var_26);
    }
    if let Some(var_27) = &input.event_source_arn {
        object.key("EventSourceArn").string(var_27);
    }
    if let Some(var_28) = &input.filter_criteria {
        let mut object_29 = object.key("FilterCriteria").start_object();
        crate::json_ser::serialize_structure_crate_model_filter_criteria(&mut object_29, var_28)?;
        object_29.finish();
    }
    if let Some(var_30) = &input.function_name {
        object.key("FunctionName").string(var_30);
    }
    if let Some(var_31) = &input.function_response_types {
        let mut array_32 = object.key("FunctionResponseTypes").start_array();
        for item_33 in var_31 {
            {
                array_32.value().string(item_33.as_str());
            }
        }
        array_32.finish();
    }
    if let Some(var_34) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    if let Some(var_35) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_35).into()),
        );
    }
    if let Some(var_36) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_36).into()),
        );
    }
    if let Some(var_37) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_37).into()),
        );
    }
    if let Some(var_38) = &input.queues {
        let mut array_39 = object.key("Queues").start_array();
        for item_40 in var_38 {
            {
                array_39.value().string(item_40);
            }
        }
        array_39.finish();
    }
    if let Some(var_41) = &input.self_managed_event_source {
        let mut object_42 = object.key("SelfManagedEventSource").start_object();
        crate::json_ser::serialize_structure_crate_model_self_managed_event_source(
            &mut object_42,
            var_41,
        )?;
        object_42.finish();
    }
    if let Some(var_43) = &input.source_access_configurations {
        let mut array_44 = object.key("SourceAccessConfigurations").start_array();
        for item_45 in var_43 {
            {
                let mut object_46 = array_44.value().start_object();
                crate::json_ser::serialize_structure_crate_model_source_access_configuration(
                    &mut object_46,
                    item_45,
                )?;
                object_46.finish();
            }
        }
        array_44.finish();
    }
    if let Some(var_47) = &input.starting_position {
        object.key("StartingPosition").string(var_47.as_str());
    }
    if let Some(var_48) = &input.starting_position_timestamp {
        object
            .key("StartingPositionTimestamp")
            .date_time(var_48, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_49) = &input.topics {
        let mut array_50 = object.key("Topics").start_array();
        for item_51 in var_49 {
            {
                array_50.value().string(item_51);
            }
        }
        array_50.finish();
    }
    if let Some(var_52) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_52).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_function_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateFunctionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_53) = &input.architectures {
        let mut array_54 = object.key("Architectures").start_array();
        for item_55 in var_53 {
            {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    if let Some(var_56) = &input.code {
        let mut object_57 = object.key("Code").start_object();
        crate::json_ser::serialize_structure_crate_model_function_code(&mut object_57, var_56)?;
        object_57.finish();
    }
    if let Some(var_58) = &input.code_signing_config_arn {
        object.key("CodeSigningConfigArn").string(var_58);
    }
    if let Some(var_59) = &input.dead_letter_config {
        let mut object_60 = object.key("DeadLetterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dead_letter_config(
            &mut object_60,
            var_59,
        )?;
        object_60.finish();
    }
    if let Some(var_61) = &input.description {
        object.key("Description").string(var_61);
    }
    if let Some(var_62) = &input.environment {
        let mut object_63 = object.key("Environment").start_object();
        crate::json_ser::serialize_structure_crate_model_environment(&mut object_63, var_62)?;
        object_63.finish();
    }
    if let Some(var_64) = &input.file_system_configs {
        let mut array_65 = object.key("FileSystemConfigs").start_array();
        for item_66 in var_64 {
            {
                let mut object_67 = array_65.value().start_object();
                crate::json_ser::serialize_structure_crate_model_file_system_config(
                    &mut object_67,
                    item_66,
                )?;
                object_67.finish();
            }
        }
        array_65.finish();
    }
    if let Some(var_68) = &input.function_name {
        object.key("FunctionName").string(var_68);
    }
    if let Some(var_69) = &input.handler {
        object.key("Handler").string(var_69);
    }
    if let Some(var_70) = &input.image_config {
        let mut object_71 = object.key("ImageConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_config(&mut object_71, var_70)?;
        object_71.finish();
    }
    if let Some(var_72) = &input.kms_key_arn {
        object.key("KMSKeyArn").string(var_72);
    }
    if let Some(var_73) = &input.layers {
        let mut array_74 = object.key("Layers").start_array();
        for item_75 in var_73 {
            {
                array_74.value().string(item_75);
            }
        }
        array_74.finish();
    }
    if let Some(var_76) = &input.memory_size {
        object.key("MemorySize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_76).into()),
        );
    }
    if let Some(var_77) = &input.package_type {
        object.key("PackageType").string(var_77.as_str());
    }
    if input.publish {
        object.key("Publish").boolean(input.publish);
    }
    if let Some(var_78) = &input.role {
        object.key("Role").string(var_78);
    }
    if let Some(var_79) = &input.runtime {
        object.key("Runtime").string(var_79.as_str());
    }
    if let Some(var_80) = &input.tags {
        let mut object_81 = object.key("Tags").start_object();
        for (key_82, value_83) in var_80 {
            {
                object_81.key(key_82).string(value_83);
            }
        }
        object_81.finish();
    }
    if let Some(var_84) = &input.timeout {
        object.key("Timeout").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_84).into()),
        );
    }
    if let Some(var_85) = &input.tracing_config {
        let mut object_86 = object.key("TracingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_tracing_config(&mut object_86, var_85)?;
        object_86.finish();
    }
    if let Some(var_87) = &input.vpc_config {
        let mut object_88 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config(&mut object_88, var_87)?;
        object_88.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_publish_layer_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishLayerVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_89) = &input.compatible_architectures {
        let mut array_90 = object.key("CompatibleArchitectures").start_array();
        for item_91 in var_89 {
            {
                array_90.value().string(item_91.as_str());
            }
        }
        array_90.finish();
    }
    if let Some(var_92) = &input.compatible_runtimes {
        let mut array_93 = object.key("CompatibleRuntimes").start_array();
        for item_94 in var_92 {
            {
                array_93.value().string(item_94.as_str());
            }
        }
        array_93.finish();
    }
    if let Some(var_95) = &input.content {
        let mut object_96 = object.key("Content").start_object();
        crate::json_ser::serialize_structure_crate_model_layer_version_content_input(
            &mut object_96,
            var_95,
        )?;
        object_96.finish();
    }
    if let Some(var_97) = &input.description {
        object.key("Description").string(var_97);
    }
    if let Some(var_98) = &input.license_info {
        object.key("LicenseInfo").string(var_98);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_publish_version_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PublishVersionInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_99) = &input.code_sha256 {
        object.key("CodeSha256").string(var_99);
    }
    if let Some(var_100) = &input.description {
        object.key("Description").string(var_100);
    }
    if let Some(var_101) = &input.revision_id {
        object.key("RevisionId").string(var_101);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_function_code_signing_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionCodeSigningConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_102) = &input.code_signing_config_arn {
        object.key("CodeSigningConfigArn").string(var_102);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_function_concurrency_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionConcurrencyInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_103) = &input.reserved_concurrent_executions {
        object.key("ReservedConcurrentExecutions").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_103).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_function_event_invoke_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutFunctionEventInvokeConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_104) = &input.destination_config {
        let mut object_105 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_105,
            var_104,
        )?;
        object_105.finish();
    }
    if let Some(var_106) = &input.maximum_event_age_in_seconds {
        object.key("MaximumEventAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_106).into()),
        );
    }
    if let Some(var_107) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_107).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_provisioned_concurrency_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutProvisionedConcurrencyConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_108) = &input.provisioned_concurrent_executions {
        object.key("ProvisionedConcurrentExecutions").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_108).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_109) = &input.tags {
        let mut object_110 = object.key("Tags").start_object();
        for (key_111, value_112) in var_109 {
            {
                object_110.key(key_111).string(value_112);
            }
        }
        object_110.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_alias_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAliasInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_113) = &input.description {
        object.key("Description").string(var_113);
    }
    if let Some(var_114) = &input.function_version {
        object.key("FunctionVersion").string(var_114);
    }
    if let Some(var_115) = &input.revision_id {
        object.key("RevisionId").string(var_115);
    }
    if let Some(var_116) = &input.routing_config {
        let mut object_117 = object.key("RoutingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_alias_routing_configuration(
            &mut object_117,
            var_116,
        )?;
        object_117.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_code_signing_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCodeSigningConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_118) = &input.allowed_publishers {
        let mut object_119 = object.key("AllowedPublishers").start_object();
        crate::json_ser::serialize_structure_crate_model_allowed_publishers(
            &mut object_119,
            var_118,
        )?;
        object_119.finish();
    }
    if let Some(var_120) = &input.code_signing_policies {
        let mut object_121 = object.key("CodeSigningPolicies").start_object();
        crate::json_ser::serialize_structure_crate_model_code_signing_policies(
            &mut object_121,
            var_120,
        )?;
        object_121.finish();
    }
    if let Some(var_122) = &input.description {
        object.key("Description").string(var_122);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_event_source_mapping_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateEventSourceMappingInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_123) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_123).into()),
        );
    }
    if let Some(var_124) = &input.bisect_batch_on_function_error {
        object.key("BisectBatchOnFunctionError").boolean(*var_124);
    }
    if let Some(var_125) = &input.destination_config {
        let mut object_126 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_126,
            var_125,
        )?;
        object_126.finish();
    }
    if let Some(var_127) = &input.enabled {
        object.key("Enabled").boolean(*var_127);
    }
    if let Some(var_128) = &input.filter_criteria {
        let mut object_129 = object.key("FilterCriteria").start_object();
        crate::json_ser::serialize_structure_crate_model_filter_criteria(&mut object_129, var_128)?;
        object_129.finish();
    }
    if let Some(var_130) = &input.function_name {
        object.key("FunctionName").string(var_130);
    }
    if let Some(var_131) = &input.function_response_types {
        let mut array_132 = object.key("FunctionResponseTypes").start_array();
        for item_133 in var_131 {
            {
                array_132.value().string(item_133.as_str());
            }
        }
        array_132.finish();
    }
    if let Some(var_134) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_134).into()),
        );
    }
    if let Some(var_135) = &input.maximum_record_age_in_seconds {
        object.key("MaximumRecordAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_136).into()),
        );
    }
    if let Some(var_137) = &input.parallelization_factor {
        object.key("ParallelizationFactor").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_137).into()),
        );
    }
    if let Some(var_138) = &input.source_access_configurations {
        let mut array_139 = object.key("SourceAccessConfigurations").start_array();
        for item_140 in var_138 {
            {
                let mut object_141 = array_139.value().start_object();
                crate::json_ser::serialize_structure_crate_model_source_access_configuration(
                    &mut object_141,
                    item_140,
                )?;
                object_141.finish();
            }
        }
        array_139.finish();
    }
    if let Some(var_142) = &input.tumbling_window_in_seconds {
        object.key("TumblingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_142).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_function_code_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionCodeInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_143) = &input.architectures {
        let mut array_144 = object.key("Architectures").start_array();
        for item_145 in var_143 {
            {
                array_144.value().string(item_145.as_str());
            }
        }
        array_144.finish();
    }
    if input.dry_run {
        object.key("DryRun").boolean(input.dry_run);
    }
    if let Some(var_146) = &input.image_uri {
        object.key("ImageUri").string(var_146);
    }
    if input.publish {
        object.key("Publish").boolean(input.publish);
    }
    if let Some(var_147) = &input.revision_id {
        object.key("RevisionId").string(var_147);
    }
    if let Some(var_148) = &input.s3_bucket {
        object.key("S3Bucket").string(var_148);
    }
    if let Some(var_149) = &input.s3_key {
        object.key("S3Key").string(var_149);
    }
    if let Some(var_150) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_150);
    }
    if let Some(var_151) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&aws_smithy_types::base64::encode(var_151));
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_function_configuration_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionConfigurationInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_152) = &input.dead_letter_config {
        let mut object_153 = object.key("DeadLetterConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_dead_letter_config(
            &mut object_153,
            var_152,
        )?;
        object_153.finish();
    }
    if let Some(var_154) = &input.description {
        object.key("Description").string(var_154);
    }
    if let Some(var_155) = &input.environment {
        let mut object_156 = object.key("Environment").start_object();
        crate::json_ser::serialize_structure_crate_model_environment(&mut object_156, var_155)?;
        object_156.finish();
    }
    if let Some(var_157) = &input.file_system_configs {
        let mut array_158 = object.key("FileSystemConfigs").start_array();
        for item_159 in var_157 {
            {
                let mut object_160 = array_158.value().start_object();
                crate::json_ser::serialize_structure_crate_model_file_system_config(
                    &mut object_160,
                    item_159,
                )?;
                object_160.finish();
            }
        }
        array_158.finish();
    }
    if let Some(var_161) = &input.handler {
        object.key("Handler").string(var_161);
    }
    if let Some(var_162) = &input.image_config {
        let mut object_163 = object.key("ImageConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_image_config(&mut object_163, var_162)?;
        object_163.finish();
    }
    if let Some(var_164) = &input.kms_key_arn {
        object.key("KMSKeyArn").string(var_164);
    }
    if let Some(var_165) = &input.layers {
        let mut array_166 = object.key("Layers").start_array();
        for item_167 in var_165 {
            {
                array_166.value().string(item_167);
            }
        }
        array_166.finish();
    }
    if let Some(var_168) = &input.memory_size {
        object.key("MemorySize").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_168).into()),
        );
    }
    if let Some(var_169) = &input.revision_id {
        object.key("RevisionId").string(var_169);
    }
    if let Some(var_170) = &input.role {
        object.key("Role").string(var_170);
    }
    if let Some(var_171) = &input.runtime {
        object.key("Runtime").string(var_171.as_str());
    }
    if let Some(var_172) = &input.timeout {
        object.key("Timeout").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_172).into()),
        );
    }
    if let Some(var_173) = &input.tracing_config {
        let mut object_174 = object.key("TracingConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_tracing_config(&mut object_174, var_173)?;
        object_174.finish();
    }
    if let Some(var_175) = &input.vpc_config {
        let mut object_176 = object.key("VpcConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_vpc_config(&mut object_176, var_175)?;
        object_176.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_function_event_invoke_config_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateFunctionEventInvokeConfigInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_177) = &input.destination_config {
        let mut object_178 = object.key("DestinationConfig").start_object();
        crate::json_ser::serialize_structure_crate_model_destination_config(
            &mut object_178,
            var_177,
        )?;
        object_178.finish();
    }
    if let Some(var_179) = &input.maximum_event_age_in_seconds {
        object.key("MaximumEventAgeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_179).into()),
        );
    }
    if let Some(var_180) = &input.maximum_retry_attempts {
        object.key("MaximumRetryAttempts").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_180).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_alias_routing_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AliasRoutingConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_181) = &input.additional_version_weights {
        let mut object_182 = object.key("AdditionalVersionWeights").start_object();
        for (key_183, value_184) in var_181 {
            {
                object_182.key(key_183).number(
                    #[allow(clippy::useless_conversion)]
                    aws_smithy_types::Number::Float((*value_184).into()),
                );
            }
        }
        object_182.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_allowed_publishers(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AllowedPublishers,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_185) = &input.signing_profile_version_arns {
        let mut array_186 = object.key("SigningProfileVersionArns").start_array();
        for item_187 in var_185 {
            {
                array_186.value().string(item_187);
            }
        }
        array_186.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_code_signing_policies(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CodeSigningPolicies,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_188) = &input.untrusted_artifact_on_deployment {
        object
            .key("UntrustedArtifactOnDeployment")
            .string(var_188.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_destination_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DestinationConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_189) = &input.on_success {
        let mut object_190 = object.key("OnSuccess").start_object();
        crate::json_ser::serialize_structure_crate_model_on_success(&mut object_190, var_189)?;
        object_190.finish();
    }
    if let Some(var_191) = &input.on_failure {
        let mut object_192 = object.key("OnFailure").start_object();
        crate::json_ser::serialize_structure_crate_model_on_failure(&mut object_192, var_191)?;
        object_192.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter_criteria(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FilterCriteria,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_193) = &input.filters {
        let mut array_194 = object.key("Filters").start_array();
        for item_195 in var_193 {
            {
                let mut object_196 = array_194.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_196, item_195)?;
                object_196.finish();
            }
        }
        array_194.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_self_managed_event_source(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SelfManagedEventSource,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_197) = &input.endpoints {
        let mut object_198 = object.key("Endpoints").start_object();
        for (key_199, value_200) in var_197 {
            {
                let mut array_201 = object_198.key(key_199.as_str()).start_array();
                for item_202 in value_200 {
                    {
                        array_201.value().string(item_202);
                    }
                }
                array_201.finish();
            }
        }
        object_198.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_source_access_configuration(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::SourceAccessConfiguration,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_203) = &input.r#type {
        object.key("Type").string(var_203.as_str());
    }
    if let Some(var_204) = &input.uri {
        object.key("URI").string(var_204);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_function_code(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FunctionCode,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_205) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&aws_smithy_types::base64::encode(var_205));
    }
    if let Some(var_206) = &input.s3_bucket {
        object.key("S3Bucket").string(var_206);
    }
    if let Some(var_207) = &input.s3_key {
        object.key("S3Key").string(var_207);
    }
    if let Some(var_208) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_208);
    }
    if let Some(var_209) = &input.image_uri {
        object.key("ImageUri").string(var_209);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dead_letter_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DeadLetterConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_210) = &input.target_arn {
        object.key("TargetArn").string(var_210);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_environment(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Environment,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_211) = &input.variables {
        let mut object_212 = object.key("Variables").start_object();
        for (key_213, value_214) in var_211 {
            {
                object_212.key(key_213).string(value_214);
            }
        }
        object_212.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_file_system_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::FileSystemConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_215) = &input.arn {
        object.key("Arn").string(var_215);
    }
    if let Some(var_216) = &input.local_mount_path {
        object.key("LocalMountPath").string(var_216);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_image_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ImageConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_217) = &input.entry_point {
        let mut array_218 = object.key("EntryPoint").start_array();
        for item_219 in var_217 {
            {
                array_218.value().string(item_219);
            }
        }
        array_218.finish();
    }
    if let Some(var_220) = &input.command {
        let mut array_221 = object.key("Command").start_array();
        for item_222 in var_220 {
            {
                array_221.value().string(item_222);
            }
        }
        array_221.finish();
    }
    if let Some(var_223) = &input.working_directory {
        object.key("WorkingDirectory").string(var_223);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tracing_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TracingConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_224) = &input.mode {
        object.key("Mode").string(var_224.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_vpc_config(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::VpcConfig,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_225) = &input.subnet_ids {
        let mut array_226 = object.key("SubnetIds").start_array();
        for item_227 in var_225 {
            {
                array_226.value().string(item_227);
            }
        }
        array_226.finish();
    }
    if let Some(var_228) = &input.security_group_ids {
        let mut array_229 = object.key("SecurityGroupIds").start_array();
        for item_230 in var_228 {
            {
                array_229.value().string(item_230);
            }
        }
        array_229.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_layer_version_content_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::LayerVersionContentInput,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_231) = &input.s3_bucket {
        object.key("S3Bucket").string(var_231);
    }
    if let Some(var_232) = &input.s3_key {
        object.key("S3Key").string(var_232);
    }
    if let Some(var_233) = &input.s3_object_version {
        object.key("S3ObjectVersion").string(var_233);
    }
    if let Some(var_234) = &input.zip_file {
        object
            .key("ZipFile")
            .string_unchecked(&aws_smithy_types::base64::encode(var_234));
    }
    Ok(())
}

pub fn serialize_structure_crate_model_on_success(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnSuccess,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_235) = &input.destination {
        object.key("Destination").string(var_235);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_on_failure(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::OnFailure,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_236) = &input.destination {
        object.key("Destination").string(var_236);
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_237) = &input.pattern {
        object.key("Pattern").string(var_237);
    }
    Ok(())
}
