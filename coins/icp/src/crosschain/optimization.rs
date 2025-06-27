//! Performance optimization for cross-chain operations

use std::collections::VecDeque;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct BatchProcessor {
   queue: Arc<Mutex<VecDeque<CrossChainMessage>>>,
   batch_size: usize,
   processing: Arc<Mutex<bool>>,
}

impl BatchProcessor {
   pub fn new(batch_size: usize) -> Self {
       Self {
           queue: Arc::new(Mutex::new(VecDeque::new())),
           batch_size,
           processing: Arc::new(Mutex::new(false)),
       }
   }
   
   /// Add message to batch queue
   pub async fn add_message(&self, message: CrossChainMessage) {
       let mut queue = self.queue.lock().await;
       queue.push_back(message);
       
       if queue.len() >= self.batch_size {
           self.trigger_batch_processing().await;
       }
   }
   
   /// Process a batch of messages
   async fn trigger_batch_processing(&self) {
       let mut processing = self.processing.lock().await;
       if *processing {
           return; // Already processing
       }
       *processing = true;
       
       // Process batch
       let mut queue = self.queue.lock().await;
       let batch: Vec<_> = queue.drain(..self.batch_size.min(queue.len())).collect();
       drop(queue);
       
       // Process batch concurrently
       let futures: Vec<_> = batch.into_iter()
           .map(|msg| self.process_message(msg))
           .collect();
       
       futures::future::join_all(futures).await;
       
       *processing = false;
   }
   
   async fn process_message(&self, message: CrossChainMessage) {
       // Actual processing logic
       tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
   }
}

use super::messages::CrossChainMessage;
