//! Performance monitoring and optimization

use std::time::{Duration, Instant};
use candid::Principal;

#[derive(Debug, Clone)]
pub struct CallMetrics {
   pub method: String,
   pub canister_id: Principal,
   pub duration: Duration,
   pub success: bool,
}

pub struct PerformanceMonitor {
   metrics: Vec<CallMetrics>,
   enabled: bool,
}

impl PerformanceMonitor {
   pub fn new() -> Self {
       Self {
           metrics: Vec::new(),
           enabled: true,
       }
   }
   
   /// Start timing a call
   pub fn start_timer(&self) -> Instant {
       Instant::now()
   }
   
   /// Record call metrics
   pub fn record_call(
       &mut self,
       method: String,
       canister_id: Principal,
       start_time: Instant,
       success: bool,
   ) {
       if !self.enabled {
           return;
       }
       
       let metrics = CallMetrics {
           method,
           canister_id,
           duration: start_time.elapsed(),
           success,
       };
       
       self.metrics.push(metrics);
   }
   
   /// Get average call duration for a method
   pub fn average_duration(&self, method: &str) -> Option<Duration> {
       let method_metrics: Vec<&CallMetrics> = self.metrics
           .iter()
           .filter(|m| m.method == method && m.success)
           .collect();
           
       if method_metrics.is_empty() {
           return None;
       }
       
       let total: Duration = method_metrics
           .iter()
           .map(|m| m.duration)
           .sum();
           
       Some(total / method_metrics.len() as u32)
   }
   
   /// Get success rate for a method
   pub fn success_rate(&self, method: &str) -> f64 {
       let method_calls: Vec<&CallMetrics> = self.metrics
           .iter()
           .filter(|m| m.method == method)
           .collect();
           
       if method_calls.is_empty() {
           return 0.0;
       }
       
       let successful = method_calls.iter().filter(|m| m.success).count();
       successful as f64 / method_calls.len() as f64
   }
}
