# Code Conventions & Style Guide

> This file ensures consistent code style survives context rot.
> Re-injected fresh each turn. Claude follows these without needing to remember.

---

## Rust Conventions

### Error Handling
```rust
// Use thiserror for custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IndexerError {
    #[error("Failed to read file: {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Invalid path: {path}")]
    InvalidPath { path: String },
}
```

### Async Runtime
- Use `tokio` for async
- Prefer `tokio::spawn` for background tasks
- Use `tokio::sync::mpsc` for channels

### Logging
```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument(skip(self))]
async fn index_file(&self, path: &Path) -> Result<(), IndexerError> {
    info!(?path, "Indexing file");
    // ...
}
```

### Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_indexer() {
        // ...
    }
}
```

---

## TypeScript Conventions

### Component Structure
```typescript
// PascalCase.tsx for components
// Props interface above component
interface FeedItemProps {
  item: FeedItem;
  onDismiss: (id: string) => void;
}

export const FeedItem = ({ item, onDismiss }: FeedItemProps) => {
  // Prefer const + arrow functions
  const handleClick = () => {
    onDismiss(item.id);
  };

  return (
    // JSX
  );
};
```

### Hooks
```typescript
// use-kebab-case.ts for hooks
export const useFeedItems = () => {
  // Hook implementation
};
```

### Type Definitions
```typescript
// types.ts or inline
type Status = 'pending' | 'active' | 'completed';

interface FeedItem {
  id: string;
  title: string;
  relevanceScore: number;
  status: Status;
}
```

---

## Naming Conventions

| Context | Convention | Example |
|---------|------------|---------|
| Files | kebab-case | `feed-item.tsx` |
| Components | PascalCase | `FeedItem` |
| Functions (TS) | camelCase | `handleClick` |
| Functions (Rust) | snake_case | `handle_click` |
| Types/Interfaces | PascalCase | `FeedItem` |
| Constants | SCREAMING_SNAKE | `MAX_ITEMS` |
| CSS Variables | kebab-case | `--bg-primary` |

---

## Design System Quick Reference

```css
/* Backgrounds */
--bg-primary: #0A0A0A;
--bg-secondary: #141414;
--bg-tertiary: #1F1F1F;

/* Text */
--text-primary: #FFFFFF;
--text-secondary: #A0A0A0;
--text-muted: #666666;

/* Accents */
--accent-primary: #FFFFFF;
--accent-gold: #D4AF37;  /* Use sparingly */

/* Borders */
--border: #2A2A2A;

/* Status */
--success: #22C55E;
--error: #EF4444;
```

---

## Code Quality Rules

1. **No over-engineering** - Only build what's needed now
2. **No premature abstraction** - Three similar lines > one premature helper
3. **Minimal error handling** - Trust internal code, validate at boundaries
4. **No feature flags** - Just change the code
5. **No backwards-compat hacks** - Delete unused code completely

---

## Import Order

### TypeScript
```typescript
// 1. React/framework
import { useState, useEffect } from 'react';

// 2. External packages
import { invoke } from '@tauri-apps/api';

// 3. Internal modules (absolute)
import { useFeedItems } from '@/hooks/use-feed-items';

// 4. Relative imports
import { FeedItem } from './feed-item';

// 5. Types (if separate)
import type { FeedItemProps } from './types';
```

### Rust
```rust
// 1. std library
use std::path::Path;

// 2. External crates
use tokio::sync::mpsc;
use tracing::info;

// 3. Crate modules
use crate::db::Database;

// 4. Super/self
use super::IndexerConfig;
```

---

*This file is auto-loaded every turn. No need to remember - just follow.*
