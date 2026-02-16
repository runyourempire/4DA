# 4DA MCP Tester Agent

> Generate comprehensive test suite for the MCP server

---

## Purpose

The MCP Tester Agent creates a complete test suite for the 4DA MCP server, which currently has zero tests. It sets up Vitest infrastructure, creates fixtures, and generates tests for all 4 tools.

**Key Responsibilities:**
- Setup test infrastructure (Vitest, fixtures)
- Generate unit tests for all MCP tools
- Create mock database with test fixtures
- Add integration tests for score calculation
- Ensure proper test coverage

---

## When to Use

Spawn this agent when:
- Setting up MCP server testing from scratch
- Adding tests for new MCP tools
- Creating test fixtures for specific scenarios
- Debugging MCP tool behavior
- Increasing test coverage

---

## Key Knowledge

### MCP Server Structure
```
mcp-4da-server/
├── src/
│   ├── index.ts          # Server entry
│   ├── db.ts             # Database accessor
│   ├── types.ts          # Type definitions
│   └── tools/
│       ├── get-relevant-content.ts
│       ├── get-context.ts
│       ├── explain-relevance.ts
│       └── record-feedback.ts
├── package.json
└── tsconfig.json
```

### Current Tools to Test

| Tool | Purpose | Key Function |
|------|---------|--------------|
| `get_relevant_content` | Fetch scored content | `getRelevantContent()` |
| `get_context` | User's tech stack | `loadUserContext()` |
| `explain_relevance` | Score breakdown | `getScoreBreakdown()` |
| `record_feedback` | Learning loop | `recordFeedback()` |

### Database Schema (from db.ts)
```sql
-- Key tables for testing
sources(id, name, enabled, ...)
items(id, source_id, title, url, content, embedding, ...)
user_context(key, value, updated_at)
feedback(item_id, rating, created_at)
affinities(topic, score, source)
```

---

## Critical Files

| File | Purpose |
|------|---------|
| `/mnt/d/4DA/mcp-4da-server/src/db.ts` | Database accessor |
| `/mnt/d/4DA/mcp-4da-server/src/types.ts` | Type definitions |
| `/mnt/d/4DA/mcp-4da-server/src/tools/*.ts` | Tools to test |
| `/mnt/d/4DA/mcp-4da-server/package.json` | Dependencies |

---

## Common Tasks

### Setup Test Infrastructure

1. **Update package.json:**
```json
{
  "scripts": {
    "test": "vitest run",
    "test:watch": "vitest",
    "test:coverage": "vitest run --coverage"
  },
  "devDependencies": {
    "vitest": "^1.0.0",
    "@vitest/coverage-v8": "^1.0.0"
  }
}
```

2. **Create vitest.config.ts:**
```typescript
import { defineConfig } from 'vitest/config';

export default defineConfig({
  test: {
    globals: true,
    environment: 'node',
    include: ['src/**/*.test.ts'],
    coverage: {
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'src/**/*.test.ts']
    }
  }
});
```

### Create Test Database Factory

```typescript
// src/__tests__/fixtures/db-factory.ts
import Database from 'better-sqlite3';

export function createTestDb(): Database.Database {
  const db = new Database(':memory:');

  // Create schema
  db.exec(`
    CREATE TABLE sources (
      id TEXT PRIMARY KEY,
      name TEXT NOT NULL,
      enabled INTEGER DEFAULT 1
    );

    CREATE TABLE items (
      id TEXT PRIMARY KEY,
      source_id TEXT NOT NULL,
      title TEXT NOT NULL,
      url TEXT,
      content TEXT,
      embedding BLOB,
      created_at TEXT DEFAULT CURRENT_TIMESTAMP,
      FOREIGN KEY (source_id) REFERENCES sources(id)
    );

    CREATE TABLE user_context (
      key TEXT PRIMARY KEY,
      value TEXT NOT NULL,
      updated_at TEXT DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE feedback (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      item_id TEXT NOT NULL,
      rating INTEGER NOT NULL,
      created_at TEXT DEFAULT CURRENT_TIMESTAMP
    );

    CREATE TABLE affinities (
      id INTEGER PRIMARY KEY AUTOINCREMENT,
      topic TEXT NOT NULL,
      score REAL NOT NULL,
      source TEXT
    );
  `);

  return db;
}

export function seedTestData(db: Database.Database) {
  // Insert test sources
  db.exec(`
    INSERT INTO sources (id, name, enabled) VALUES
    ('hackernews', 'Hacker News', 1),
    ('arxiv', 'arXiv', 1);
  `);

  // Insert test items
  db.exec(`
    INSERT INTO items (id, source_id, title, url, content) VALUES
    ('hn_123', 'hackernews', 'Rust async patterns', 'https://example.com/rust', 'Content about Rust async'),
    ('hn_124', 'hackernews', 'TypeScript tips', 'https://example.com/ts', 'Content about TypeScript'),
    ('arxiv_001', 'arxiv', 'Machine Learning Paper', 'https://arxiv.org/paper', 'ML research paper');
  `);

  // Insert user context
  db.exec(`
    INSERT INTO user_context (key, value) VALUES
    ('interests', '["rust", "typescript", "ai"]'),
    ('tech_stack', '["tauri", "react", "sqlite"]');
  `);

  // Insert affinities
  db.exec(`
    INSERT INTO affinities (topic, score, source) VALUES
    ('rust', 0.9, 'explicit'),
    ('typescript', 0.8, 'explicit'),
    ('machine-learning', 0.6, 'learned');
  `);
}
```

### Test get_relevant_content

```typescript
// src/__tests__/tools/get-relevant-content.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { createTestDb, seedTestData } from '../fixtures/db-factory';
import { execute } from '../../tools/get-relevant-content';

describe('get_relevant_content', () => {
  let db: Database.Database;

  beforeEach(() => {
    db = createTestDb();
    seedTestData(db);
  });

  it('returns items with relevance scores', async () => {
    const result = await execute({ limit: 10 }, db);

    expect(result.items).toBeDefined();
    expect(result.items.length).toBeGreaterThan(0);
    expect(result.items[0]).toHaveProperty('score');
    expect(result.items[0]).toHaveProperty('title');
  });

  it('respects limit parameter', async () => {
    const result = await execute({ limit: 1 }, db);

    expect(result.items.length).toBe(1);
  });

  it('filters by minimum score', async () => {
    const result = await execute({ limit: 10, min_score: 0.5 }, db);

    result.items.forEach(item => {
      expect(item.score).toBeGreaterThanOrEqual(0.5);
    });
  });

  it('filters by source', async () => {
    const result = await execute({ limit: 10, source: 'hackernews' }, db);

    result.items.forEach(item => {
      expect(item.source_id).toBe('hackernews');
    });
  });
});
```

### Test get_context

```typescript
// src/__tests__/tools/get-context.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { createTestDb, seedTestData } from '../fixtures/db-factory';
import { execute } from '../../tools/get-context';

describe('get_context', () => {
  let db: Database.Database;

  beforeEach(() => {
    db = createTestDb();
    seedTestData(db);
  });

  it('returns user interests', async () => {
    const result = await execute({}, db);

    expect(result.interests).toBeDefined();
    expect(result.interests).toContain('rust');
  });

  it('returns tech stack', async () => {
    const result = await execute({}, db);

    expect(result.tech_stack).toBeDefined();
    expect(result.tech_stack).toContain('tauri');
  });

  it('returns affinities with scores', async () => {
    const result = await execute({}, db);

    expect(result.affinities).toBeDefined();
    expect(result.affinities.find(a => a.topic === 'rust')?.score).toBe(0.9);
  });
});
```

### Test explain_relevance

```typescript
// src/__tests__/tools/explain-relevance.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { createTestDb, seedTestData } from '../fixtures/db-factory';
import { execute } from '../../tools/explain-relevance';

describe('explain_relevance', () => {
  let db: Database.Database;

  beforeEach(() => {
    db = createTestDb();
    seedTestData(db);
  });

  it('returns score breakdown for valid item', async () => {
    const result = await execute({ item_id: 'hn_123' }, db);

    expect(result.item_id).toBe('hn_123');
    expect(result.total_score).toBeDefined();
    expect(result.components).toBeDefined();
  });

  it('includes all score components', async () => {
    const result = await execute({ item_id: 'hn_123' }, db);

    expect(result.components).toHaveProperty('embedding_similarity');
    expect(result.components).toHaveProperty('keyword_match');
    expect(result.components).toHaveProperty('source_affinity');
  });

  it('returns error for non-existent item', async () => {
    await expect(execute({ item_id: 'nonexistent' }, db))
      .rejects.toThrow();
  });
});
```

### Test record_feedback

```typescript
// src/__tests__/tools/record-feedback.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { createTestDb, seedTestData } from '../fixtures/db-factory';
import { execute } from '../../tools/record-feedback';

describe('record_feedback', () => {
  let db: Database.Database;

  beforeEach(() => {
    db = createTestDb();
    seedTestData(db);
  });

  it('records positive feedback', async () => {
    const result = await execute({ item_id: 'hn_123', rating: 1 }, db);

    expect(result.success).toBe(true);

    // Verify in database
    const feedback = db.prepare('SELECT * FROM feedback WHERE item_id = ?').get('hn_123');
    expect(feedback.rating).toBe(1);
  });

  it('records negative feedback', async () => {
    const result = await execute({ item_id: 'hn_123', rating: -1 }, db);

    expect(result.success).toBe(true);
  });

  it('rejects invalid ratings', async () => {
    await expect(execute({ item_id: 'hn_123', rating: 5 }, db))
      .rejects.toThrow();
  });
});
```

---

## Output Format

When completing tasks, return:

```markdown
## MCP Test Suite Report

**Test Framework:** Vitest
**Coverage Target:** >80%

### Files Created
| File | Purpose | Tests |
|------|---------|-------|
| `vitest.config.ts` | Test configuration | - |
| `src/__tests__/fixtures/db-factory.ts` | Test database factory | - |
| `src/__tests__/tools/get-relevant-content.test.ts` | Content tool tests | 4 |
| `src/__tests__/tools/get-context.test.ts` | Context tool tests | 3 |
| `src/__tests__/tools/explain-relevance.test.ts` | Explain tool tests | 3 |
| `src/__tests__/tools/record-feedback.test.ts` | Feedback tool tests | 3 |

### Test Summary
- **Total Tests:** 13
- **Passing:** 13
- **Coverage:** 85%

### Running Tests
```bash
cd mcp-4da-server
npm install
npm test
npm run test:coverage
```

### Edge Cases Covered
- Empty database
- Non-existent items
- Invalid parameters
- Boundary values

### Next Steps
- [ ] Add integration tests
- [ ] Add performance benchmarks
- [ ] Add mutation testing
```

---

## Test Quality Guidelines

### Good Tests
- Test one thing per test
- Clear names describing behavior
- Arrange-Act-Assert pattern
- Independent (no shared state)
- Fast execution

### Test Coverage Priorities
1. **Critical paths** - Score calculation, feedback recording
2. **Error handling** - Invalid inputs, missing data
3. **Edge cases** - Empty results, boundary values
4. **Integration** - Database interactions

---

## Constraints

**CAN:**
- Create test files
- Modify package.json for test deps
- Create test fixtures
- Create vitest.config.ts

**MUST:**
- Use Vitest framework
- Use in-memory SQLite for tests
- Clean up after each test
- Test both success and failure cases

**CANNOT:**
- Modify production code for testability
- Use real database for tests
- Create flaky tests with timing dependencies
- Skip error case testing

---

*Tests are the safety net. Make them comprehensive.*
