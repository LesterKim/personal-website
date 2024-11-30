use async_trait::async_trait;
use chrono::{DateTime, Utc};
use crate::domain::models::{BlogPost, Service};
use crate::ports::repositories::{BlogRepository, ServiceRepository, Error};

pub struct MockBlogRepository;

impl MockBlogRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BlogRepository for MockBlogRepository {
    async fn get_posts(&self) -> Result<Vec<BlogPost>, Error> {
        Ok(vec![
            BlogPost {
                id: "1".to_string(),
                title: "Building Scalable Web3 Infrastructure".to_string(),
                content: r#"Building Scalable Web3 Infrastructure presents unique challenges that require careful consideration of scalability, security, and user experience. In this post, I'll share insights from my experience building decentralized applications at scale.

## The Challenge

Traditional web applications follow a client-server model, but Web3 apps need to interact with blockchain networks, handle wallet connections, and manage state across multiple chains. This creates several key challenges:

```rust
// Example of a Web3 connection manager
pub struct Web3Manager {
    eth_client: EthereumClient,
    wallet_connection: Option<WalletConnection>,
    chain_id: u64,
}

impl Web3Manager {
    pub async fn connect_wallet(&mut self) -> Result<WalletConnection, Web3Error> {
        // Wallet connection logic
    }
}
```

## Best Practices

1. **State Management**
   - Use a robust state management solution
   - Implement proper error handling for blockchain interactions
   - Cache blockchain data when possible

2. **Security Considerations**
   - Always validate transactions client-side
   - Implement proper signature verification
   - Use secure RPC endpoints

3. **Performance Optimization**
   - Batch blockchain calls when possible
   - Implement efficient caching strategies
   - Use WebSocket connections for real-time updates

## Architecture Overview

Here's a high-level overview of a scalable Web3 infrastructure:

```rust
pub struct Web3Infrastructure {
    blockchain_service: BlockchainService,
    cache_layer: CacheLayer,
    transaction_manager: TransactionManager,
}

impl Web3Infrastructure {
    pub async fn process_transaction(&self, tx: Transaction) -> Result<TxHash, Error> {
        // 1. Validate transaction
        self.validate_transaction(&tx)?;
        
        // 2. Check cache
        if let Some(cached) = self.cache_layer.get(&tx.id).await? {
            return Ok(cached);
        }
        
        // 3. Submit to blockchain
        let hash = self.blockchain_service.submit(tx).await?;
        
        // 4. Cache result
        self.cache_layer.set(&tx.id, &hash).await?;
        
        Ok(hash)
    }
}
```

## Conclusion

Building scalable Web3 infrastructure requires a deep understanding of both blockchain technology and traditional web architecture. By following these best practices and implementing proper error handling, caching, and security measures, you can create robust decentralized applications that can handle growth and provide a great user experience.

Remember to always test thoroughly, especially edge cases around wallet connections and transaction handling. The Web3 space is still evolving, so staying up to date with the latest security practices is crucial."#.to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-30T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            },
            BlogPost {
                id: "2".to_string(),
                title: "The Future of DeFi: A Technical Perspective".to_string(),
                content: r#"The Future of DeFi: A Technical Perspective explores the evolving landscape of decentralized finance and its technical implications.

## Current State of DeFi

DeFi protocols have grown exponentially, handling billions in total value locked (TVL). However, several technical challenges remain:

```solidity
// Example of a basic DeFi lending protocol
contract LendingPool {
    mapping(address => uint256) public deposits;
    mapping(address => uint256) public borrows;
    
    function deposit() external payable {
        deposits[msg.sender] += msg.value;
        // Calculate and distribute yield
    }
    
    function borrow(uint256 amount) external {
        require(deposits[msg.sender] * 2 >= amount, "Insufficient collateral");
        borrows[msg.sender] += amount;
        // Transfer funds and update state
    }
}
```

## Technical Challenges

1. **Scalability**
   - High gas fees during peak usage
   - Network congestion
   - Cross-chain interoperability

2. **Security**
   - Smart contract vulnerabilities
   - Oracle manipulation
   - Flash loan attacks

3. **User Experience**
   - Complex interfaces
   - High transaction costs
   - Slow confirmation times

## Future Solutions

### Layer 2 Scaling

```rust
pub struct L2Solution {
    rollup_manager: RollupManager,
    state_validator: StateValidator,
}

impl L2Solution {
    pub async fn process_batch(&self, transactions: Vec<Transaction>) -> Result<BatchProof, Error> {
        // 1. Aggregate transactions
        let batch = self.rollup_manager.aggregate(transactions)?;
        
        // 2. Generate proof
        let proof = self.rollup_manager.generate_proof(batch).await?;
        
        // 3. Validate state transition
        self.state_validator.validate(proof.clone())?;
        
        Ok(proof)
    }
}
```

### Cross-chain Interoperability

The future of DeFi will likely be multi-chain, requiring robust cross-chain communication protocols and standards.

## Conclusion

The future of DeFi is bright, but solving these technical challenges requires innovative solutions and careful consideration of security implications. As we move forward, focusing on scalability, security, and user experience will be crucial for mainstream adoption."#.to_string(),
                published_at: DateTime::parse_from_rfc3339("2024-11-25T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            },
        ])
    }

    async fn get_post(&self, id: &str) -> Result<Option<BlogPost>, Error> {
        let posts = self.get_posts().await?;
        Ok(posts.into_iter().find(|post| post.id == id))
    }
}

pub struct MockServiceRepository;

impl MockServiceRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceRepository for MockServiceRepository {
    async fn get_services(&self) -> Result<Vec<Service>, Error> {
        Ok(vec![
            Service {
                id: "1".to_string(),
                name: "Technical Leadership".to_string(),
                description: "Strategic technical guidance for your startup's growth and success.".to_string(),
                price_range: "$200-500/hour".to_string(),
            },
            Service {
                id: "2".to_string(),
                name: "Web3 Development".to_string(),
                description: "End-to-end development of decentralized applications.".to_string(),
                price_range: "Custom pricing".to_string(),
            },
        ])
    }
} 