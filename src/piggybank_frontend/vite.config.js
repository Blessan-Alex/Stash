import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import path from 'path';

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:4943',
        changeOrigin: true,
        secure: false,
      },
    },
  },
  define: {
    global: 'window',
    'process.env': {},
  },
  build: {
    target: 'es2020',
    rollupOptions: {
      external: ['buffer'],
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      target: 'es2020',
    },
    include: [
      '@dfinity/agent',
      '@dfinity/auth-client',
      '@dfinity/principal',
    ],
  },
});
