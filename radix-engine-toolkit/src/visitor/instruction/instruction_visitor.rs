// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::error::VisitorError;
use crate::model::instruction::Instruction;
use crate::visitor::{traverse_value, ManifestAstValueVisitor};

macro_rules! define_instruction_visitor {
    (
        $(#[$meta:meta])*
        $vis: vis trait $trait_ident: ident {
            $($method_ident: ident ($($arg_ident: ident: $arg_type: ty),*$(,)?));*
        }
    ) => {
        $(#[$meta])*
        $vis trait $trait_ident {
            $(
                fn $method_ident(
                    &mut self,
                    $($arg_ident: $arg_type,)*
                ) -> Result<(), $crate::error::VisitorError> {
                    Ok(())
                }
            )*
        }
    };
}

macro_rules! visit {
    ($visitors: expr, $method: ident, $($value: expr),*$(,)?) => {
        $visitors
            .iter_mut()
            .map(|visitor| visitor.$method($($value,)*))
            .collect::<Result<Vec<_>, _>>()
    };
}

define_instruction_visitor! {
    pub trait InstructionVisitor {
        visit_call_function(
            _package_address: &mut crate::model::value::ast::ManifestAstValue,
            _blueprint_name: &mut crate::model::value::ast::ManifestAstValue,
            _function_name: &mut crate::model::value::ast::ManifestAstValue,
            _arguments: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_call_method(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
            _method_name: &mut crate::model::value::ast::ManifestAstValue,
            _arguments: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_call_royalty_method(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
            _method_name: &mut crate::model::value::ast::ManifestAstValue,
            _arguments: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_call_metadata_method(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
            _method_name: &mut crate::model::value::ast::ManifestAstValue,
            _arguments: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_call_access_rules_method(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
            _method_name: &mut crate::model::value::ast::ManifestAstValue,
            _arguments: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_take_all_from_worktop(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _into_bucket: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_take_from_worktop(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
            _into_bucket: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_take_non_fungibles_from_worktop(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _ids: &mut Vec<crate::model::value::ast::ManifestAstValue>,
            _into_bucket: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_return_to_worktop(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_assert_worktop_contains(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_assert_worktop_contains_non_fungibles(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _ids: &mut Vec<crate::model::value::ast::ManifestAstValue>,
        );
        visit_pop_from_auth_zone(
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_push_to_auth_zone(
            _proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_clear_auth_zone();
        visit_clear_signature_proofs();
        visit_create_proof_from_auth_zone(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_auth_zone_of_all(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_auth_zone_of_amount(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_auth_zone_of_non_fungibles(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _ids: &mut Vec<crate::model::value::ast::ManifestAstValue>,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_bucket(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_bucket_of_all(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_bucket_of_amount(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_proof_from_bucket_of_non_fungibles(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
            _ids: &mut Vec<crate::model::value::ast::ManifestAstValue>,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_clone_proof(
            _proof: &mut crate::model::value::ast::ManifestAstValue,
            _into_proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_drop_proof(
            _proof: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_drop_all_proofs();
        visit_publish_package(
            _code: &mut crate::model::value::ast::ManifestAstValue,
            _schema: &mut crate::model::value::ast::ManifestAstValue,
            _royalty_config: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_publish_package_advanced(
            _code: &mut crate::model::value::ast::ManifestAstValue,
            _schema: &mut crate::model::value::ast::ManifestAstValue,
            _royalty_config: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
            _authority_rules: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_burn_resource(
            _bucket: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_recall_resource(
            _vault_id: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_set_metadata(
            _entity_address: &mut crate::model::value::ast::ManifestAstValue,
            _key: &mut crate::model::value::ast::ManifestAstValue,
            _value: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_remove_metadata(
            _entity_address: &mut crate::model::value::ast::ManifestAstValue,
            _key: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_set_package_royalty_config(
            _package_address: &mut crate::model::value::ast::ManifestAstValue,
            _royalty_config: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_set_component_royalty_config(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
            _royalty_config: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_claim_package_royalty(
            _package_address: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_claim_component_royalty(
            _component_address: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_set_authority_access_rule(
            _entity_address: &mut crate::model::value::ast::ManifestAstValue,
            _object_key: &mut crate::model::value::ast::ManifestAstValue,
            _authority_key: &mut crate::model::value::ast::ManifestAstValue,
            _rule: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_set_authority_mutability(
            _entity_address: &mut crate::model::value::ast::ManifestAstValue,
            _object_key: &mut crate::model::value::ast::ManifestAstValue,
            _authority_key: &mut crate::model::value::ast::ManifestAstValue,
            _mutability: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_mint_fungible(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _amount: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_mint_non_fungible(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _entries: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_mint_uuid_non_fungible(
            _resource_address: &mut crate::model::value::ast::ManifestAstValue,
            _entries: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_fungible_resource(
            _divisibility: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
            _access_rules: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_fungible_resource_with_initial_supply(
            _divisibility: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
            _access_rules: &mut crate::model::value::ast::ManifestAstValue,
            _initial_supply: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_non_fungible_resource(
            _id_type: &mut crate::model::value::ast::ManifestAstValue,
            _schema: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
            _access_rules: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_non_fungible_resource_with_initial_supply(
            _id_type: &mut crate::model::value::ast::ManifestAstValue,
            _schema: &mut crate::model::value::ast::ManifestAstValue,
            _metadata: &mut crate::model::value::ast::ManifestAstValue,
            _access_rules: &mut crate::model::value::ast::ManifestAstValue,
            _initial_supply: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_access_controller(
            _controlled_asset: &mut crate::model::value::ast::ManifestAstValue,
            _rule_set: &mut crate::model::value::ast::ManifestAstValue,
            _timed_recovery_delay_in_minutes: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_validator(
            _key: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_identity();
        visit_create_identity_advanced(
            _config: &mut crate::model::value::ast::ManifestAstValue,
        );
        visit_create_account();
        visit_create_account_advanced(
            _config: &mut crate::model::value::ast::ManifestAstValue,
        );

        post_visit()
    }
}

/// A function which traverses [`Instruction`]s calling the value visitors first and then calling
/// the instruction visitors
pub fn traverse_instruction(
    instruction: &mut Instruction,
    value_visitors: &mut [&mut dyn ManifestAstValueVisitor],
    instructions_visitors: &mut [&mut dyn InstructionVisitor],
) -> Result<(), VisitorError> {
    match instruction {
        Instruction::CallFunction {
            package_address,
            blueprint_name,
            function_name,
            arguments,
        } => {
            traverse_value(package_address, value_visitors)?;
            traverse_value(blueprint_name, value_visitors)?;
            traverse_value(function_name, value_visitors)?;
            arguments
                .iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_call_function,
                package_address,
                blueprint_name,
                function_name,
                arguments,
            )?;
        }
        Instruction::CallMethod {
            component_address,
            method_name,
            arguments,
        } => {
            traverse_value(component_address, value_visitors)?;
            traverse_value(method_name, value_visitors)?;
            arguments
                .iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_call_method,
                component_address,
                method_name,
                arguments,
            )?;
        }
        Instruction::CallRoyaltyMethod {
            component_address,
            method_name,
            arguments,
        } => {
            traverse_value(component_address, value_visitors)?;
            traverse_value(method_name, value_visitors)?;
            arguments
                .iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_call_royalty_method,
                component_address,
                method_name,
                arguments,
            )?;
        }
        Instruction::CallMetadataMethod {
            component_address,
            method_name,
            arguments,
        } => {
            traverse_value(component_address, value_visitors)?;
            traverse_value(method_name, value_visitors)?;
            arguments
                .iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_call_metadata_method,
                component_address,
                method_name,
                arguments,
            )?;
        }
        Instruction::CallAccessRulesMethod {
            component_address,
            method_name,
            arguments,
        } => {
            traverse_value(component_address, value_visitors)?;
            traverse_value(method_name, value_visitors)?;
            arguments
                .iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_call_access_rules_method,
                component_address,
                method_name,
                arguments,
            )?;
        }
        Instruction::TakeAllFromWorktop {
            resource_address,
            into_bucket,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(into_bucket, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_take_all_from_worktop,
                resource_address,
                into_bucket,
            )?;
        }
        Instruction::TakeFromWorktop {
            resource_address,
            amount,
            into_bucket,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            traverse_value(into_bucket, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_take_from_worktop,
                resource_address,
                amount,
                into_bucket,
            )?;
        }
        Instruction::TakeNonFungiblesFromWorktop {
            resource_address,
            ids,
            into_bucket,
        } => {
            traverse_value(resource_address, value_visitors)?;
            ids.iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            traverse_value(into_bucket, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_take_non_fungibles_from_worktop,
                resource_address,
                ids,
                into_bucket,
            )?;
        }
        Instruction::ReturnToWorktop { bucket } => {
            traverse_value(bucket, value_visitors)?;
            visit!(instructions_visitors, visit_return_to_worktop, bucket,)?;
        }
        Instruction::AssertWorktopContains {
            resource_address,
            amount,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_assert_worktop_contains,
                resource_address,
                amount,
            )?;
        }
        Instruction::AssertWorktopContainsNonFungibles {
            resource_address,
            ids,
        } => {
            traverse_value(resource_address, value_visitors)?;
            ids.iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            visit!(
                instructions_visitors,
                visit_assert_worktop_contains_non_fungibles,
                resource_address,
                ids,
            )?;
        }
        Instruction::PopFromAuthZone { into_proof } => {
            traverse_value(into_proof, value_visitors)?;
            visit!(instructions_visitors, visit_pop_from_auth_zone, into_proof,)?;
        }
        Instruction::PushToAuthZone { proof } => {
            traverse_value(proof, value_visitors)?;
            visit!(instructions_visitors, visit_push_to_auth_zone, proof,)?;
        }
        Instruction::ClearAuthZone {} => {
            visit!(instructions_visitors, visit_clear_auth_zone,)?;
        }
        Instruction::ClearSignatureProofs {} => {
            visit!(instructions_visitors, visit_clear_signature_proofs,)?;
        }
        Instruction::CreateProofFromAuthZone {
            resource_address,
            into_proof,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_auth_zone,
                resource_address,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromAuthZoneOfAll {
            resource_address,
            into_proof,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_auth_zone_of_all,
                resource_address,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromAuthZoneOfAmount {
            resource_address,
            amount,
            into_proof,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_auth_zone_of_amount,
                resource_address,
                amount,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromAuthZoneOfNonFungibles {
            resource_address,
            ids,
            into_proof,
        } => {
            traverse_value(resource_address, value_visitors)?;
            ids.iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_auth_zone_of_non_fungibles,
                resource_address,
                ids,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromBucket { bucket, into_proof } => {
            traverse_value(bucket, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_bucket,
                bucket,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromBucketOfAll { bucket, into_proof } => {
            traverse_value(bucket, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_bucket_of_all,
                bucket,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromBucketOfAmount {
            bucket,
            amount,
            into_proof,
        } => {
            traverse_value(bucket, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_bucket_of_amount,
                bucket,
                amount,
                into_proof,
            )?;
        }
        Instruction::CreateProofFromBucketOfNonFungibles {
            bucket,
            ids,
            into_proof,
        } => {
            traverse_value(bucket, value_visitors)?;
            ids.iter_mut()
                .map(|value| traverse_value(value, value_visitors))
                .collect::<Result<Vec<_>, VisitorError>>()?;
            traverse_value(into_proof, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_proof_from_bucket_of_non_fungibles,
                bucket,
                ids,
                into_proof,
            )?;
        }
        Instruction::CloneProof { proof, into_proof } => {
            traverse_value(proof, value_visitors)?;
            traverse_value(into_proof, value_visitors)?;
            visit!(instructions_visitors, visit_clone_proof, proof, into_proof,)?;
        }
        Instruction::DropProof { proof } => {
            traverse_value(proof, value_visitors)?;
            visit!(instructions_visitors, visit_drop_proof, proof,)?;
        }
        Instruction::DropAllProofs {} => {
            visit!(instructions_visitors, visit_drop_all_proofs,)?;
        }
        Instruction::PublishPackage {
            code,
            schema,
            royalty_config,
            metadata,
        } => {
            traverse_value(code, value_visitors)?;
            traverse_value(schema, value_visitors)?;
            traverse_value(royalty_config, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_publish_package,
                code,
                schema,
                royalty_config,
                metadata,
            )?;
        }
        Instruction::PublishPackageAdvanced {
            code,
            schema,
            royalty_config,
            metadata,
            authority_rules,
        } => {
            traverse_value(code, value_visitors)?;
            traverse_value(schema, value_visitors)?;
            traverse_value(royalty_config, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            traverse_value(authority_rules, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_publish_package_advanced,
                code,
                schema,
                royalty_config,
                metadata,
                authority_rules,
            )?;
        }
        Instruction::BurnResource { bucket } => {
            traverse_value(bucket, value_visitors)?;
            visit!(instructions_visitors, visit_burn_resource, bucket,)?;
        }
        Instruction::RecallResource { vault_id, amount } => {
            traverse_value(vault_id, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_recall_resource,
                vault_id,
                amount,
            )?;
        }
        Instruction::SetMetadata {
            entity_address,
            key,
            value,
        } => {
            traverse_value(entity_address, value_visitors)?;
            traverse_value(key, value_visitors)?;
            traverse_value(value, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_set_metadata,
                entity_address,
                key,
                value,
            )?;
        }
        Instruction::RemoveMetadata {
            entity_address,
            key,
        } => {
            traverse_value(entity_address, value_visitors)?;
            traverse_value(key, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_remove_metadata,
                entity_address,
                key,
            )?;
        }
        Instruction::SetPackageRoyaltyConfig {
            package_address,
            royalty_config,
        } => {
            traverse_value(package_address, value_visitors)?;
            traverse_value(royalty_config, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_set_package_royalty_config,
                package_address,
                royalty_config,
            )?;
        }
        Instruction::SetComponentRoyaltyConfig {
            component_address,
            royalty_config,
        } => {
            traverse_value(component_address, value_visitors)?;
            traverse_value(royalty_config, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_set_component_royalty_config,
                component_address,
                royalty_config,
            )?;
        }
        Instruction::ClaimPackageRoyalty { package_address } => {
            traverse_value(package_address, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_claim_package_royalty,
                package_address,
            )?;
        }
        Instruction::ClaimComponentRoyalty { component_address } => {
            traverse_value(component_address, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_claim_component_royalty,
                component_address,
            )?;
        }
        Instruction::SetAuthorityAccessRule {
            entity_address,
            object_key,
            authority_key,
            rule,
        } => {
            traverse_value(entity_address, value_visitors)?;
            traverse_value(object_key, value_visitors)?;
            traverse_value(authority_key, value_visitors)?;
            traverse_value(rule, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_set_authority_access_rule,
                entity_address,
                object_key,
                authority_key,
                rule,
            )?;
        }
        Instruction::SetAuthorityMutability {
            entity_address,
            object_key,
            authority_key,
            mutability,
        } => {
            traverse_value(entity_address, value_visitors)?;
            traverse_value(object_key, value_visitors)?;
            traverse_value(authority_key, value_visitors)?;
            traverse_value(mutability, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_set_authority_mutability,
                entity_address,
                object_key,
                authority_key,
                mutability,
            )?;
        }
        Instruction::MintFungible {
            resource_address,
            amount,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(amount, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_mint_fungible,
                resource_address,
                amount,
            )?;
        }
        Instruction::MintNonFungible {
            resource_address,
            entries,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(entries, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_mint_non_fungible,
                resource_address,
                entries,
            )?;
        }
        Instruction::MintUuidNonFungible {
            resource_address,
            entries,
        } => {
            traverse_value(resource_address, value_visitors)?;
            traverse_value(entries, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_mint_uuid_non_fungible,
                resource_address,
                entries,
            )?;
        }
        Instruction::CreateFungibleResource {
            divisibility,
            metadata,
            access_rules,
        } => {
            traverse_value(divisibility, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            traverse_value(access_rules, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_fungible_resource,
                divisibility,
                metadata,
                access_rules,
            )?;
        }
        Instruction::CreateFungibleResourceWithInitialSupply {
            divisibility,
            metadata,
            access_rules,
            initial_supply,
        } => {
            traverse_value(divisibility, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            traverse_value(access_rules, value_visitors)?;
            traverse_value(initial_supply, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_fungible_resource_with_initial_supply,
                divisibility,
                metadata,
                access_rules,
                initial_supply,
            )?;
        }
        Instruction::CreateNonFungibleResource {
            id_type,
            schema,
            metadata,
            access_rules,
        } => {
            traverse_value(id_type, value_visitors)?;
            traverse_value(schema, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            traverse_value(access_rules, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_non_fungible_resource,
                id_type,
                schema,
                metadata,
                access_rules,
            )?;
        }
        Instruction::CreateNonFungibleResourceWithInitialSupply {
            id_type,
            schema,
            metadata,
            access_rules,
            initial_supply,
        } => {
            traverse_value(id_type, value_visitors)?;
            traverse_value(schema, value_visitors)?;
            traverse_value(metadata, value_visitors)?;
            traverse_value(access_rules, value_visitors)?;
            traverse_value(initial_supply, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_non_fungible_resource_with_initial_supply,
                id_type,
                schema,
                metadata,
                access_rules,
                initial_supply,
            )?;
        }
        Instruction::CreateAccessController {
            controlled_asset,
            rule_set,
            timed_recovery_delay_in_minutes,
        } => {
            traverse_value(controlled_asset, value_visitors)?;
            traverse_value(rule_set, value_visitors)?;
            traverse_value(timed_recovery_delay_in_minutes, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_access_controller,
                controlled_asset,
                rule_set,
                timed_recovery_delay_in_minutes,
            )?;
        }
        Instruction::CreateValidator { key } => {
            traverse_value(key, value_visitors)?;
            visit!(instructions_visitors, visit_create_validator, key,)?;
        }
        Instruction::CreateIdentity {} => {
            visit!(instructions_visitors, visit_create_identity,)?;
        }
        Instruction::CreateIdentityAdvanced { config } => {
            traverse_value(config, value_visitors)?;
            visit!(
                instructions_visitors,
                visit_create_identity_advanced,
                config,
            )?;
        }
        Instruction::CreateAccount {} => {
            visit!(instructions_visitors, visit_create_account,)?;
        }
        Instruction::CreateAccountAdvanced { config } => {
            traverse_value(config, value_visitors)?;
            visit!(instructions_visitors, visit_create_account_advanced, config,)?;
        }
    };
    visit!(instructions_visitors, post_visit,)?;
    Ok(())
}
