/**
 * Vitest Test Setup
 *
 * This file runs before each test file.
 * Configure global test utilities here.
 */

import '@testing-library/jest-dom';

// Mock Tauri API for tests (since we can't access native APIs in jsdom)
vi.mock('@tauri-apps/api', () => ({
  invoke: vi.fn(),
  event: {
    listen: vi.fn(() => Promise.resolve(() => {})),
    emit: vi.fn(),
  },
}));

// Mock Tauri plugin opener
vi.mock('@tauri-apps/plugin-opener', () => ({
  open: vi.fn(),
}));

// Global test utilities
beforeEach(() => {
  // Reset mocks before each test
  vi.clearAllMocks();
});

// Suppress console errors in tests unless explicitly testing error handling
const originalError = console.error;
beforeAll(() => {
  console.error = (...args) => {
    // Allow React testing library warnings through
    if (typeof args[0] === 'string' && args[0].includes('Warning:')) {
      return;
    }
    originalError(...args);
  };
});

afterAll(() => {
  console.error = originalError;
});
