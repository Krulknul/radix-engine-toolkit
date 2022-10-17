use crate::error::Error;
use crate::export_request;
use crate::models::manifest_instructions::ManifestInstructionsKind;
use crate::models::serde::SignedTransactionIntent;
use crate::traits::{Request, Validate};
use crate::validation::validate_transaction_intent;
use scrypto::prelude::scrypto_decode;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::convert::TryInto;

// ==========================
// Request & Response Models
// ==========================

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileSignedTransactionIntentRequest {
    pub manifest_instructions_output_format: ManifestInstructionsKind,

    #[serde_as(as = "serde_with::hex::Hex")]
    pub compiled_signed_intent: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DecompileSignedTransactionIntentResponse {
    #[serde(flatten)]
    pub signed_intent: SignedTransactionIntent,
}

// ===========
// Validation
// ===========

impl Validate for DecompileSignedTransactionIntentRequest {
    fn validate(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Validate for DecompileSignedTransactionIntentResponse {
    fn validate(&self) -> Result<(), Error> {
        validate_transaction_intent(&self.signed_intent.transaction_intent)?;
        Ok(())
    }
}

// =======================
// Request Implementation
// =======================

impl<'r> Request<'r, DecompileSignedTransactionIntentResponse>
    for DecompileSignedTransactionIntentRequest
{
    fn handle_request(self) -> Result<DecompileSignedTransactionIntentResponse, Error> {
        let signed_transaction_intent: SignedTransactionIntent =
            scrypto_decode::<radix_transaction::model::SignedTransactionIntent>(
                &self.compiled_signed_intent,
            )?
            .try_into()?;
        let signed_transaction_intent: SignedTransactionIntent = signed_transaction_intent
            .convert_manifest_instructions_kind(self.manifest_instructions_output_format)?;

        let response: DecompileSignedTransactionIntentResponse =
            DecompileSignedTransactionIntentResponse {
                signed_intent: signed_transaction_intent,
            };
        Ok(response)
    }
}

export_request!(DecompileSignedTransactionIntentRequest as decompile_signed_transaction_intent);

// ======
// Tests
// ======

#[cfg(test)]
mod tests {
    // TODO: Unit tests for this request type
}
