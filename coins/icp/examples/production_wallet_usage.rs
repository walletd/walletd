use walletd_icp::production::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize with extreme security configuration
    let config = WalletManagerConfig {
        security_config: SecurityConfig {
            use_hsm: true,
            hsm_config: HsmConfig {
                provider: "AWS CloudHSM".to_string(),
                cluster_id: "cluster-xxxxx".to_string(),
            },
            audit_path: "/secure/audit/",
            key_rotation_days: 30,
        },
        storage_config: StorageConfig {
            primary_path: "/data/primary",
            replica_paths: vec![
                "/data/replica1".to_string(),
                "/data/replica2".to_string(),
                "/data/replica3".to_string(),
            ],
            replication_mode: ReplicationMode::QuorumBased(2),
            backup_config: BackupConfig {
                s3_bucket: "wallet-backups".to_string(),
                encryption_key: std::env::var("BACKUP_KEY")?,
                retention_days: 90,
            },
        },
        pool_config: ConnectionPoolConfig {
            min_connections: 10,
            max_connections: 100,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300),
            health_check_interval: Duration::from_secs(60),
        },
        monitoring_config: MonitoringConfig {
            prometheus_addr: "0.0.0.0:9090".to_string(),
            alerting_config: AlertingConfig {
                pagerduty_token: std::env::var("PAGERDUTY_TOKEN").ok(),
                slack_webhook: std::env::var("SLACK_WEBHOOK").ok(),
                email_config: Some(EmailConfig {
                    smtp_host: "smtp.company.com".to_string(),
                    from_address: "alerts@company.com".to_string(),
                    to_addresses: vec!["oncall@company.com".to_string()],
                }),
            },
        },
        rate_limit_config: RateLimitConfig {
            window_size: Duration::from_secs(60),
            max_requests: 100,
            burst_size: 20,
        },
        max_concurrent_operations: 1000,
    };
    
    // Create wallet manager
    let manager = EnterpriseWalletManager::new(config).await?;
    
    // Create wallets for users
    let wallet1 = manager.create_wallet("user-001", "auth-token-xxx").await?;
    println!("Created wallet: {:?}", wallet1);
    
    // Execute transaction with all security checks
    let tx_result = manager.execute_transaction(TransactionRequest {
        user_id: "user-001".to_string(),
        auth_token: "auth-token-xxx".to_string(),
        to: "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(),
        amount: 100_000_000, // 1 ICP
        memo: Some(12345),
        two_fa_code: Some("123456".to_string()),
    }).await?;
    
    println!("Transaction completed: {:?}", tx_result);
    
    // Batch operations
    let balances = manager.batch_get_balances(vec![
        "user-001".to_string(),
        "user-002".to_string(),
        "user-003".to_string(),
    ]).await?;
    
    println!("Balances: {:?}", balances);
    
    Ok(())
}
