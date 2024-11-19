// vite.config.ts
import { defineConfig } from 'vite';

export default defineConfig({
  test: {
    setupFiles: './testSetup.ts', // Path to the setup file
  },
});
