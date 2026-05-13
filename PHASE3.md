# Phase 3: Svelte UI

## How to run frontend tests

1. Make sure you have Vitest installed:
```bash
npm install -D vitest @testing-library/svelte jsdom
```

2. Add this to your `vite.config.js` or `vitest.config.ts`:
```js
/// <reference types="vitest" />
import { defineConfig } from 'vite';
import { svelte } from '@sveltejs/vite-plugin-svelte';

export default defineConfig({
  plugins: [svelte()],
  test: {
    environment: 'jsdom',
    globals: true,
  },
});
```

3. Run tests:
```bash
npm run test
``` or `npx vitest`

## Current Phase 3 Tests
- Sidebar renders entities
- Highlights current turn
- Dispatches select event
