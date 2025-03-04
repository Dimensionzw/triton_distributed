// SPDX-FileCopyrightText: Copyright (c) 2024-2025 NVIDIA CORPORATION & AFFILIATES. All rights reserved.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use async_trait::async_trait;

use super::*;

use crate::traits::events::EventPublisher;

#[async_trait]
impl EventPublisher for Namespace {
    fn subject(&self) -> String {
        format!("namespace.{}", self.name)
    }

    async fn publish(
        &self,
        event_name: impl AsRef<str> + Send + Sync,
        event: &(impl Serialize + Send + Sync),
    ) -> Result<()> {
        let bytes = serde_json::to_vec(event)?;
        self.publish_bytes(event_name, bytes).await
    }

    async fn publish_bytes(
        &self,
        event_name: impl AsRef<str> + Send + Sync,
        bytes: Vec<u8>,
    ) -> Result<()> {
        let subject = format!("{}.{}", self.subject(), event_name.as_ref());
        Ok(self
            .drt()
            .nats_client()
            .client()
            .publish(subject, bytes.into())
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // todo - make a distributed runtime fixture
    // todo - two options - fully mocked or integration test
    #[cfg(feature = "integration")]
    #[tokio::test]
    async fn test_publish() {
        // todo - use rtest - make fixtures
        let dtr = DistributedRuntime::from_settings(Runtime::single_threaded().unwrap())
            .await
            .unwrap();

        let ns = dtr.namespace("test".to_string()).unwrap();

        ns.publish("test", &"test".to_string()).await.unwrap();
    }
}
