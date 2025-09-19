#!/usr/bin/env node

import fs from 'fs';
import path from 'path';

const VERSION_ARG = process.argv[2];

if (!VERSION_ARG) {
  console.error('Usage: node update-version.js <version>');
  console.error('Example: node update-version.js 1.2.0');
  process.exit(1);
}

const files = [
  {
    path: 'package.json',
    update: (content) => {
      const pkg = JSON.parse(content);
      pkg.version = VERSION_ARG;
      return JSON.stringify(pkg, null, 2) + '\n';
    }
  },
  {
    path: 'README.md',
    update: (content) => {
      return content.replace(
        /rem_\d+\.\d+\.\d+_/g,
        `rem_${VERSION_ARG}_`
      ).replace(
        /download\/\d+\.\d+\.\d+\//g,
        `download/${VERSION_ARG}/`
      );
    }
  },
  {
    path: 'src-tauri/tauri.conf.json',
    update: (content) => {
      const config = JSON.parse(content);
      config.version = VERSION_ARG;
      return JSON.stringify(config, null, 2) + '\n';
    }
  }
];

console.log(`Updating version to ${VERSION_ARG}...`);

files.forEach(file => {
  try {
    const content = fs.readFileSync(file.path, 'utf8');
    const updatedContent = file.update(content);
    fs.writeFileSync(file.path, updatedContent);
    console.log(`✓ Updated ${file.path}`);
  } catch (error) {
    console.error(`✗ Failed to update ${file.path}: ${error.message}`);
  }
});

console.log('Version update completed.');
