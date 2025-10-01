# Vault Configuration for Production-like Development
# This configuration provides enhanced security while maintaining development usability

# Storage backend - using file storage for development
storage "file" {
  path = "/vault/data"
}

# API listener configuration
listener "tcp" {
  address = "0.0.0.0:8200"
  tls_disable = true  # Only for development - use TLS in production
  tls_min_version = "tls12"
  tls_cipher_suites = "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305,TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256"
  
  # Security headers
  x_forwarded_for_authorized_addrs = "127.0.0.1/8,172.16.0.0/12"
  x_forwarded_for_hop_skips = 1
  x_forwarded_for_reject_not_authorized = true
  x_forwarded_for_reject_not_present = true
}

# Telemetry configuration
telemetry {
  prometheus_retention_time = "24h"
  disable_hostname = true
  enable_hostname_label = false
}

# Audit logging
audit {
  enabled = true
  path = "/vault/audit"
  format = "json"
  retention = "90d"
  
  # Additional audit options
  log_raw = false
  log_request_body = false
  log_response_body = false
}

# High availability (disabled for development)
# ha_storage "consul" {
#   address = "consul:8500"
#   path = "vault/"
#   service = "vault"
# }

# Seal configuration (disabled for development)
# seal "awskms" {
#   region = "us-east-1"
#   kms_key_id = "alias/vault-key"
# }

# UI configuration
ui = true

# Default lease TTL
default_lease_ttl = "1h"
max_lease_ttl = "24h"

# Cluster configuration
cluster_name = "sut-vault-cluster"

# Disable mlock for development (enable in production)
disable_mlock = true

# Log level
log_level = "INFO"
log_format = "json"

# Plugin directory
plugin_directory = "/vault/plugins"

# API rate limiting
api_addr = "http://0.0.0.0:8200"
cluster_addr = "http://0.0.0.0:8201"

# Security settings
raw_storage_endpoint = false
disable_sealwrap = false
disable_performance_standby = false
disable_sentinel_trace = false
disable_indexing = false
disable_ui = false
disable_clustering = true

# Entropy augmentation (disabled for development)
# entropy "seal" {
#   mode = "augmentation"
# }

# License (if using Vault Enterprise features)
# license_path = "/vault/license/vault.hclic"