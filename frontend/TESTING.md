# Frontend Testing Guide

## Test Suite Overview

This project uses **Jest** and **React Testing Library** for testing the Next.js frontend application.

## Test Coverage

### Components Tests
- **LoadingSpinner**: Test animations and size variants
- **ErrorAlert**: Test error display and retry functionality  
- **MetricCard**: Test metric display with variants and trends
- **PlayerCard**: Test player information display and actions

### Hooks Tests
- **use-api hooks**: Test data fetching with SWR (usePlayers, usePlayer, useDashboard, usePlayerStats)

### Integration Tests
- **Dashboard Page**: Test player selection, filters, and stats display
- **Players Page**: Test search, filtering, and player list display

### Utilities Tests
- **cn function**: Test className merging with Tailwind CSS

## Running Tests

### Run all tests
```bash
npm test
```

### Run tests in watch mode (for development)
```bash
npm run test:watch
```

### Run tests with coverage report
```bash
npm run test:coverage
```

### Run specific test file
```bash
npm test -- player-card.test.tsx
```

### Run tests matching pattern
```bash
npm test -- --testNamePattern="LoadingSpinner"
```

## Test Structure

```
frontend/
├── __tests__/
│   ├── components/          # Component unit tests
│   │   ├── loading-spinner.test.tsx
│   │   ├── error-alert.test.tsx
│   │   ├── metric-card.test.tsx
│   │   └── player-card.test.tsx
│   ├── hooks/              # Custom hooks tests
│   │   └── use-api.test.tsx
│   ├── integration/        # Integration tests
│   │   ├── dashboard.test.tsx
│   │   └── players-page.test.tsx
│   └── lib/                # Utilities tests
│       └── utils.test.ts
├── jest.config.js          # Jest configuration
└── jest.setup.js           # Test setup (jest-dom)
```

## Writing Tests

### Component Test Example
```typescript
import { render, screen } from '@testing-library/react';
import { MyComponent } from '@/components/my-component';

describe('MyComponent', () => {
  it('renders correctly', () => {
    render(<MyComponent />);
    expect(screen.getByText('Hello')).toBeInTheDocument();
  });
});
```

### Hook Test Example
```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { useMyHook } from '@/hooks/use-my-hook';

describe('useMyHook', () => {
  it('fetches data', async () => {
    const { result } = renderHook(() => useMyHook());
    
    await waitFor(() => {
      expect(result.current.data).toBeDefined();
    });
  });
});
```

## CI/CD Integration

Tests can be run in CI/CD pipelines:

```yaml
# Example GitHub Actions
- name: Run tests
  run: npm test -- --ci --coverage
```

## Test Results

Current test status:
- ✅ 8 test suites passing
- ✅ 63 tests passing
- ✅ 0 snapshots
- 🎯 All components, hooks, and integration flows tested

## Mocking

### Next.js Router
```typescript
jest.mock('next/navigation', () => ({
  useRouter: () => ({
    push: jest.fn(),
    refresh: jest.fn(),
  }),
}));
```

### Next.js Link
```typescript
jest.mock('next/link', () => {
  return ({ children, href, ...props }: any) => {
    return <a href={href} {...props}>{children}</a>;
  };
});
```

### SWR
```typescript
jest.mock('swr', () => {
  return jest.fn(() => ({
    data: mockData,
    error: undefined,
    isLoading: false,
    mutate: jest.fn(),
  }));
});
```

## Best Practices

1. **Test user behavior**, not implementation details
2. Use **semantic queries** (getByRole, getByLabelText) over test IDs
3. **Mock external dependencies** (API calls, router, etc.)
4. Write **descriptive test names** that explain what is being tested
5. Group related tests using **describe blocks**
6. Use **waitFor** for async operations
7. Clean up after tests with **jest.clearAllMocks()**

## Troubleshooting

### Tests timeout
Increase timeout in jest.config.js:
```javascript
testTimeout: 10000
```

### Can't find module
Check moduleNameMapper in jest.config.js matches your tsconfig paths.

### Recharts warnings
Recharts warnings in tests are expected (no DOM dimensions in test environment).

## Resources

- [Jest Documentation](https://jestjs.io/)
- [React Testing Library](https://testing-library.com/react)
- [Testing Next.js](https://nextjs.org/docs/testing)
