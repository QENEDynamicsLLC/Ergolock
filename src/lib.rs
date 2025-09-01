//! # ErgoLock - Quantum-Safe Cryptography Platform
//! 
//! **DEMONSTRATION LIBRARY ONLY**
//! 
//! This library contains non-proprietary demonstration code for the ErgoLock
//! Quantum-Safe Cryptography Platform. Core proprietary technologies including
//! QENE™ (Quantum Entropy Nullification Engine) are not included.
//! 
//! For access to the complete technology stack, contact:
//! qenedynamics@protonmail.com

use std::fmt;

/// Demonstration quantum-safe encryption interface
pub struct QuantumSafeEncryption {
    performance_monitoring: bool,
    ai_enhancement: bool,
}

impl QuantumSafeEncryption {
    /// Create new quantum-safe encryption instance (demonstration)
    pub fn new() -> Self {
        Self {
            performance_monitoring: false,
            ai_enhancement: false,
        }
    }
    
    /// Enable performance monitoring (demonstration)
    pub fn with_performance_monitoring(mut self, enabled: bool) -> Self {
        self.performance_monitoring = enabled;
        self
    }
    
    /// Enable AI enhancement (demonstration)
    pub fn with_ai_enhancement(mut self, enabled: bool) -> Self {
        self.ai_enhancement = enabled;
        self
    }
    
    /// Encrypt data with quantum-safe algorithms (demonstration)
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // DEMONSTRATION ONLY - Real implementation requires licensing
        if data.is_empty() {
            return Err(EncryptionError::InvalidInput);
        }
        
        // Placeholder encryption for demonstration
        let mut encrypted = Vec::new();
        encrypted.extend_from_slice(b"ERGOLOCK_DEMO_");
        encrypted.extend_from_slice(data);
        encrypted.extend_from_slice(b"_QUANTUM_SAFE");
        
        Ok(encrypted)
    }
    
    /// Decrypt data (demonstration)
    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // DEMONSTRATION ONLY
        if encrypted.len() < 28 {
            return Err(EncryptionError::InvalidInput);
        }
        
        // Remove demo markers
        let data = &encrypted[14..encrypted.len()-12];
        Ok(data.to_vec())
    }
}

/// Demonstration network security interface
pub struct NetworkSecurity {
    protection_level: ProtectionLevel,
}

#[derive(Debug, Clone)]
pub enum ProtectionLevel {
    Standard,
    MilitaryGrade,
    QuantumSafe,
}

impl NetworkSecurity {
    /// Create military-grade network security (demonstration)
    pub fn military_grade() -> Self {
        Self {
            protection_level: ProtectionLevel::MilitaryGrade,
        }
    }
    
    /// Enable post-quantum protection (demonstration)
    pub fn with_post_quantum_protection(mut self) -> Self {
        self.protection_level = ProtectionLevel::QuantumSafe;
        self
    }
    
    /// Enable performance optimization (demonstration)
    pub fn with_performance_optimization(self) -> Self {
        // Demonstration only
        self
    }
    
    /// Establish secure channel (demonstration)
    pub fn establish_channel(&self, _endpoint: &str) -> Result<SecureChannel, NetworkError> {
        Ok(SecureChannel {
            protection: self.protection_level.clone(),
        })
    }
}

/// Demonstration secure channel
pub struct SecureChannel {
    protection: ProtectionLevel,
}

impl SecureChannel {
    /// Get protection level
    pub fn protection_level(&self) -> &ProtectionLevel {
        &self.protection
    }
}

/// Demonstration performance monitor
pub struct PerformanceMonitor;

impl PerformanceMonitor {
    /// Create new performance monitor (demonstration)
    pub fn new() -> Self {
        Self
    }
    
    /// Record performance metric (demonstration)
    pub fn record_metric(&self, _name: &str, _value: f64) {
        // Demonstration only
    }
}

/// Encryption errors (demonstration)
#[derive(Debug)]
pub enum EncryptionError {
    InvalidInput,
    ProcessingError,
}

impl fmt::Display for EncryptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncryptionError::InvalidInput => write!(f, "Invalid input data"),
            EncryptionError::ProcessingError => write!(f, "Processing error"),
        }
    }
}

impl std::error::Error for EncryptionError {}

/// Network errors (demonstration)
#[derive(Debug)]
pub enum NetworkError {
    ConnectionFailed,
    SecurityError,
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::ConnectionFailed => write!(f, "Connection failed"),
            NetworkError::SecurityError => write!(f, "Security error"),
        }
    }
}

impl std::error::Error for NetworkError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_demonstration_encryption() {
        let encryptor = QuantumSafeEncryption::new()
            .with_performance_monitoring(true)
            .with_ai_enhancement(true);
        
        let data = b"Test data";
        let encrypted = encryptor.encrypt(data).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();
        
        assert_eq!(data.to_vec(), decrypted);
    }

    #[test]
    fn test_network_security() {
        let security = NetworkSecurity::military_grade()
            .with_post_quantum_protection()
            .with_performance_optimization();
        
        let channel = security.establish_channel("demo.endpoint").unwrap();
        assert!(matches!(channel.protection_level(), ProtectionLevel::QuantumSafe));
    }
}
