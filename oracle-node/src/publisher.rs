use crate::config::NearConfig;
use crate::types::PriceData;
use anyhow::{anyhow, Result};
use near_crypto::{InMemorySigner, SecretKey};
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction};
use near_primitives::types::{BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::json;
use std::sync::Arc;
use tracing::{info, debug};

pub struct NearPublisher {
    client: JsonRpcClient,
    signer: Arc<InMemorySigner>,
    contract_id: String,
}

impl NearPublisher {
    pub async fn new(config: &NearConfig) -> Result<Self> {
        let client = JsonRpcClient::connect(&config.rpc_url);

        // Parse private key
        let secret_key: SecretKey = config.private_key.parse()
            .map_err(|e| anyhow!("Failed to parse private key: {}", e))?;

        let signer = Arc::new(InMemorySigner::from_secret_key(
            config.signer_id.parse()?,
            secret_key,
        ));

        Ok(Self {
            client,
            signer,
            contract_id: config.contract_id.clone(),
        })
    }

    pub async fn publish_prices(&self, prices: Vec<PriceData>) -> Result<()> {
        info!("Publishing {} prices to NEAR contract {}", prices.len(), self.contract_id);

        // Convert to NEAR format
        let updates: Vec<_> = prices.iter()
            .map(|p| p.to_near_format())
            .collect();

        // Prepare function call
        let args = json!({
            "updates": updates
        });

        debug!("Transaction args: {}", args);

        // Get current nonce and block hash
        let access_key_query_response = self.client
            .call(methods::query::RpcQueryRequest {
                block_reference: BlockReference::Finality(Finality::Final),
                request: QueryRequest::ViewAccessKey {
                    account_id: self.signer.account_id.clone(),
                    public_key: self.signer.public_key.clone(),
                },
            })
            .await?;

        let current_nonce = match access_key_query_response.kind {
            QueryResponseKind::AccessKey(access_key) => access_key.nonce,
            _ => return Err(anyhow!("Failed to get access key")),
        };

        let block_hash = access_key_query_response.block_hash;

        // Create transaction
        let transaction = Transaction {
            signer_id: self.signer.account_id.clone(),
            public_key: self.signer.public_key.clone(),
            nonce: current_nonce + 1,
            receiver_id: self.contract_id.parse()?,
            block_hash,
            actions: vec![Action::FunctionCall(Box::new(FunctionCallAction {
                method_name: "submit_price".to_string(),
                args: FunctionArgs::from(serde_json::to_vec(&args)?),
                gas: 30_000_000_000_000, // 30 TGas
                deposit: 0,
            }))],
        };

        // Sign and send transaction
        let signed_transaction = transaction.sign(&*self.signer);
        
        let request = methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction,
        };

        let response = self.client.call(request).await?;

        info!("Transaction successful: {}", response.transaction.hash);
        debug!("Transaction status: {:?}", response.status);

        Ok(())
    }

    pub async fn check_contract_status(&self) -> Result<()> {
        // Query contract to verify it's accessible
        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::ViewAccount {
                account_id: self.contract_id.parse()?,
            },
        };

        let response = self.client.call(request).await?;
        info!("Contract status verified: {}", self.contract_id);
        debug!("Response: {:?}", response);

        Ok(())
    }

    pub async fn query_supported_assets(&self) -> Result<Vec<String>> {
        let args = json!({});

        let request = methods::query::RpcQueryRequest {
            block_reference: BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: self.contract_id.parse()?,
                method_name: "get_supported_assets".to_string(),
                args: FunctionArgs::from(serde_json::to_vec(&args)?),
            },
        };

        let response = self.client.call(request).await?;

        if let QueryResponseKind::CallResult(result) = response.kind {
            let assets: Vec<String> = serde_json::from_slice(&result.result)?;
            Ok(assets)
        } else {
            Err(anyhow!("Unexpected response type"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Integration tests require a running NEAR testnet node
    // Run with: cargo test --features integration-tests
}

