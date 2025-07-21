# Noted Sync System Schema & Architecture

## Overview
This document defines the synchronization schema and architecture for Noted's multi-device, encrypted sync system with support for multiple cloud providers.

## Sync Architecture Principles

### 1. Local-First Operation
- Full functionality without network
- Sync is enhancement, not requirement
- Optimistic updates with eventual consistency

### 2. Zero-Knowledge Encryption
- Client-side encryption before upload
- Server/cloud provider cannot decrypt data
- Key derivation from user passphrase

### 3. Conflict Resolution
- Last-write-wins with conflict detection
- Vector clocks for causality tracking
- Manual resolution for complex conflicts

### 4. Provider Agnostic
- Abstract sync protocol
- Adapter pattern for different providers
- Consistent data format across providers

## Sync Data Model

### Entity Hierarchy
```
User Account
  └── Devices (multiple)
  └── Encryption Keys
  └── Sync Spaces
      └── Channels
          └── Notes
              └── Attachments
      └── Tags
      └── Preferences
```

## Core Sync Schema

### 1. Sync Manifest
Central index of all synced entities for efficient delta sync.

```json
{
  "manifest": {
    "version": "1.0",
    "device_id": "uuid",
    "last_sync": "2024-01-20T10:30:00Z",
    "encryption": {
      "algorithm": "AES-256-GCM",
      "key_derivation": "Argon2id",
      "key_version": 1
    },
    "entities": {
      "channels": {
        "total": 15,
        "last_modified": "2024-01-20T10:30:00Z",
        "checksum": "sha256_hash"
      },
      "notes": {
        "total": 342,
        "last_modified": "2024-01-20T10:25:00Z",
        "checksum": "sha256_hash"
      },
      "attachments": {
        "total": 89,
        "last_modified": "2024-01-20T09:15:00Z",
        "checksum": "sha256_hash"
      }
    }
  }
}
```

### 2. Encrypted Entity Wrapper
All synced data wrapped with encryption metadata.

```json
{
  "entity_wrapper": {
    "id": "entity_uuid",
    "type": "note|channel|attachment|tag|preference",
    "version": 3,
    "device_id": "originating_device_uuid",
    "created_at": "2024-01-20T10:30:00Z",
    "modified_at": "2024-01-20T10:30:00Z",
    "vector_clock": {
      "device_1": 5,
      "device_2": 3,
      "device_3": 7
    },
    "encrypted_data": "base64_encrypted_payload",
    "encryption_iv": "base64_iv",
    "signature": "hmac_signature",
    "deleted": false
  }
}
```

### 3. Decrypted Entity Schemas

#### Channel Entity
```json
{
  "channel": {
    "id": "channel_uuid",
    "name": "Project Ideas",
    "icon": {
      "type": "emoji|image|system",
      "value": "💡"
    },
    "parent_id": "parent_channel_uuid",
    "position": 2,
    "settings": {
      "default_view": "card|chat",
      "sort_order": "manual|date_created|date_modified",
      "color_theme": "#4A90E2"
    },
    "metadata": {
      "created_at": "2024-01-20T10:30:00Z",
      "modified_at": "2024-01-20T10:30:00Z",
      "note_count": 42,
      "last_accessed": "2024-01-20T10:30:00Z"
    }
  }
}
```

#### Note Entity
```json
{
  "note": {
    "id": "note_uuid",
    "channel_id": "channel_uuid",
    "content": {
      "format": "markdown",
      "text": "# Meeting Notes\n\nDiscussed project timeline...",
      "preview": "Meeting Notes - Discussed project timeline"
    },
    "position": 5,
    "tags": ["meeting", "project-x"],
    "metadata": {
      "created_at": "2024-01-20T10:30:00Z",
      "modified_at": "2024-01-20T10:30:00Z",
      "word_count": 234,
      "has_attachments": true,
      "pinned": false,
      "archived": false
    },
    "attachments": [
      {
        "id": "attachment_uuid",
        "ref": "attachments/uuid/filename.jpg"
      }
    ]
  }
}
```

#### Attachment Entity
```json
{
  "attachment": {
    "id": "attachment_uuid",
    "note_id": "note_uuid",
    "filename": "screenshot.png",
    "mime_type": "image/png",
    "size": 1048576,
    "checksum": "sha256_hash",
    "metadata": {
      "width": 1920,
      "height": 1080,
      "duration": null,
      "thumbnail": {
        "ref": "thumbnails/uuid/thumb.jpg",
        "width": 200,
        "height": 150
      }
    },
    "storage": {
      "provider": "gdrive|icloud|onedrive|custom",
      "path": "noted/attachments/uuid/screenshot.png",
      "uploaded_at": "2024-01-20T10:30:00Z"
    }
  }
}
```

#### Tag Entity
```json
{
  "tag": {
    "id": "tag_uuid",
    "name": "important",
    "color": "#FF5733",
    "metadata": {
      "created_at": "2024-01-20T10:30:00Z",
      "usage_count": 15
    }
  }
}
```

#### Preference Entity
```json
{
  "preference": {
    "id": "pref_uuid",
    "key": "entry_menu_layout",
    "value": {
      "grid_columns": 3,
      "items": [
        {"type": "reminder", "position": 0},
        {"type": "todo", "position": 1},
        {"type": "ai_search", "position": 2}
      ]
    },
    "scope": "global|device",
    "metadata": {
      "modified_at": "2024-01-20T10:30:00Z"
    }
  }
}
```

### 4. Sync Operation Records

#### Change Log
Track all local changes for sync queue.

```json
{
  "change_log": {
    "id": "change_uuid",
    "entity_type": "note",
    "entity_id": "note_uuid",
    "operation": "create|update|delete",
    "timestamp": "2024-01-20T10:30:00Z",
    "device_id": "device_uuid",
    "changes": {
      "content": {
        "old": "old text",
        "new": "new text"
      }
    },
    "sync_status": "pending|syncing|synced|conflict"
  }
}
```

#### Conflict Record
Store conflicts for user resolution.

```json
{
  "conflict": {
    "id": "conflict_uuid",
    "entity_type": "note",
    "entity_id": "note_uuid",
    "local_version": {
      "version": 5,
      "modified_at": "2024-01-20T10:30:00Z",
      "device_id": "device_1",
      "snapshot": {}
    },
    "remote_version": {
      "version": 5,
      "modified_at": "2024-01-20T10:28:00Z",
      "device_id": "device_2",
      "snapshot": {}
    },
    "detected_at": "2024-01-20T10:31:00Z",
    "resolution": null
  }
}
```

## Sync Protocol

### 1. Initial Sync Flow
```
1. Generate device ID and register
2. Derive encryption keys from passphrase
3. Download manifest
4. Compare with local state
5. Download missing entities
6. Upload local-only entities
7. Mark sync complete
```

### 2. Delta Sync Flow
```
1. Check manifest for changes since last sync
2. Download updated entities
3. Detect conflicts using vector clocks
4. Upload local changes
5. Update manifest
6. Handle conflicts if any
```

### 3. Conflict Resolution Strategy
```
1. Same entity modified on multiple devices
2. Compare vector clocks
3. If concurrent (no causal relationship):
   - For notes: Present both versions
   - For channels: Last-write-wins
   - For preferences: Device-specific wins
4. Update vector clocks after resolution
```

## Provider Adapters

### Storage Structure
```
/noted-sync/
  /manifest.json.enc
  /devices/
    /device_uuid.json.enc
  /channels/
    /channel_uuid.json.enc
  /notes/
    /note_uuid.json.enc
  /attachments/
    /attachment_uuid/
      /metadata.json.enc
      /file.enc
      /thumbnail.jpg.enc
  /tags/
    /tags.json.enc
  /preferences/
    /global.json.enc
    /device_uuid.json.enc
```

### Provider-Specific Implementations

#### Google Drive
- Use app-specific folder
- Batch API for efficiency
- Resume tokens for large uploads

#### iCloud
- CloudKit for metadata
- Document storage for attachments
- Subscription for real-time updates

#### OneDrive
- Delta API for changes
- Large file upload sessions
- Webhook notifications

#### Self-Hosted
- REST API with JWT auth
- WebSocket for real-time sync
- S3-compatible object storage

## Security Considerations

### Encryption Scheme
```
1. Master Key Derivation:
   - Argon2id(passphrase, salt, iterations=3, memory=64MB)

2. Entity Encryption:
   - AES-256-GCM for data
   - Unique IV per entity
   - HMAC-SHA256 for authentication

3. Key Rotation:
   - Version tracking
   - Re-encrypt on key change
   - Grace period for old keys
```

### Security Metadata
```json
{
  "security": {
    "key_version": 2,
    "key_rotation_date": "2024-01-20T10:30:00Z",
    "device_authorization": {
      "device_id": "device_uuid",
      "authorized_at": "2024-01-20T10:30:00Z",
      "last_seen": "2024-01-20T10:30:00Z",
      "capabilities": ["read", "write", "delete"]
    }
  }
}
```

## Performance Optimizations

### Sync Strategies

#### Incremental Sync
- Track entity versions
- Download only changes
- Merge changes locally

#### Batch Operations
- Group small changes
- Compress before encryption
- Parallel upload/download

#### Smart Scheduling
- Wi-Fi only for attachments
- Background sync intervals
- Battery-aware sync

### Caching Strategy
```json
{
  "cache_policy": {
    "manifest_ttl": 300,
    "entity_cache_size": "100MB",
    "attachment_cache_size": "500MB",
    "thumbnail_retention": "30d",
    "eviction_policy": "LRU"
  }
}
```

## Error Handling

### Sync Errors
```json
{
  "sync_error": {
    "code": "SYNC_CONFLICT|NETWORK_ERROR|QUOTA_EXCEEDED|AUTH_FAILED",
    "message": "Human readable error",
    "entity_id": "affected_entity_uuid",
    "retry_after": "2024-01-20T10:35:00Z",
    "resolution_hint": "Check network connection"
  }
}
```

### Recovery Procedures
1. **Network Failures**: Exponential backoff
2. **Quota Exceeded**: Queue for later
3. **Corruption**: Rebuild from local
4. **Auth Issues**: Re-authenticate

## Monitoring & Analytics

### Sync Metrics
```json
{
  "sync_metrics": {
    "last_successful_sync": "2024-01-20T10:30:00Z",
    "sync_duration_ms": 2500,
    "entities_synced": {
      "uploaded": 5,
      "downloaded": 12,
      "conflicts": 1
    },
    "bandwidth_used": {
      "upload_bytes": 1048576,
      "download_bytes": 2097152
    },
    "error_count": 0
  }
}
```

## Migration Strategy

### Version Compatibility
- Maintain backward compatibility
- Schema version in manifest
- Migration scripts for upgrades

### Data Export Format
```json
{
  "export": {
    "version": "1.0",
    "exported_at": "2024-01-20T10:30:00Z",
    "device_id": "device_uuid",
    "channels": [],
    "notes": [],
    "attachments": [],
    "tags": [],
    "preferences": []
  }
}
```
