const { execSync } = require('child_process');
process.env.LEPTOS_OUTPUT_NAME = '{{project-name}}';

try {
    execSync('cargo leptos build --release', { stdio: 'inherit' });
    execSync('worker-build --release --features worker', { stdio: 'inherit' });
    console.log('Build completed successfully!');
} catch (error) {
    console.error('Build failed:', error.message);
    process.exit(1);
}