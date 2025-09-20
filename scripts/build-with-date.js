#!/usr/bin/env node

// Build script that sets build date environment variable
const { execSync } = require('child_process');

// Get current date in YYYY-MM-DD format
const buildDate = new Date().toISOString().split('T')[0];

// Set environment variable and run build
process.env.VITE_BUILD_DATE = buildDate;

console.log(`Building with build date: ${buildDate}`);

// Run the TypeScript compilation and Vite build
try {
  execSync('tsc', { stdio: 'inherit' });
  execSync('vite build', { stdio: 'inherit' });
  console.log('Build completed successfully!');
} catch (error) {
  console.error('Build failed:', error.message);
  process.exit(1);
}
