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

use scrypto::prelude::*;

use crate::transaction_types::*;

pub struct PresentedProofsDetector {
    presented_proofs: IndexSet<ResourceAddress>,
}

impl ManifestSummaryCallback for PresentedProofsDetector {
    fn on_create_proof(&mut self, resource_address: &ResourceAddress) {
        self.presented_proofs.insert(*resource_address);
    }
}

impl ExecutionSummaryCallback for PresentedProofsDetector {}
