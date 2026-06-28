<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Build Optimization - Spore CLI Documentation</title>
	<meta name="description" content="Optimize your builds for maximum performance and efficiency with Spore's build optimization features." />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Build Optimization
		</h1>
		<p class="text-lg text-muted-foreground">
			Optimize your builds for maximum performance and efficiency.
		</p>
	</div>

	<div class="prose prose-slate max-w-none">
		<h2 class="text-2xl font-bold mb-4">Build Performance</h2>

		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">Parallel Building</h3>
			<p class="text-sm text-muted-foreground mb-3">Leverage multiple CPU cores for faster builds</p>
			<CodeBlock label="bash" copy={`# Build with parallel processing
spore build --parallel

# Specify number of workers
spore build --parallel 4

# Build specific packages in parallel
spore build @org/package-* --parallel`}>{`# Build with parallel processing
spore build --parallel

# Specify number of workers
spore build --parallel 4

# Build specific packages in parallel
spore build @org/package-* --parallel`}</CodeBlock>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Caching Strategies</h2>

		<div class="space-y-4">
			<div class="bg-muted border border-border rounded-lg p-4">
				<h3 class="font-semibold text-foreground mb-2">Build Cache</h3>
				<p class="text-sm text-muted-foreground mb-3">Spore automatically caches build outputs to speed up subsequent builds</p>
				<CodeBlock label="bash" copy={`# View cache status
spore cache status

# Clear cache
spore cache clean

# Build with cache disabled
spore build --no-cache`}>{`# View cache status
spore cache status

# Clear cache
spore cache clean

# Build with cache disabled
spore build --no-cache`}</CodeBlock>
			</div>

			<div class="bg-muted border border-border rounded-lg p-4">
				<h3 class="font-semibold text-foreground mb-2">Dependency Cache</h3>
				<p class="text-sm text-muted-foreground mb-3">Cache dependencies for faster installs</p>
				<CodeBlock label="bash" copy={`# Cache location
~/.spore/cache/

# Prune unused cached packages
spore cache prune

# Verify cache integrity
spore cache verify`}>{`# Cache location
~/.spore/cache/

# Prune unused cached packages
spore cache prune

# Verify cache integrity
spore cache verify`}</CodeBlock>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Bundle Optimization</h2>

		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Tree Shaking</h3>
				<p class="text-sm text-muted-foreground mb-3">Remove unused code from bundles</p>
				<CodeBlock label="spore.config.js">{`// spore.config.js
export default {
  build: {
    treeShake: true,
    sideEffects: false,
    pure: ['console.log', 'console.debug']
  }
}`}</CodeBlock>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Code Splitting</h3>
				<p class="text-sm text-muted-foreground mb-3">Split code into smaller chunks</p>
				<CodeBlock label="javascript">{`// Dynamic imports for code splitting
const MyComponent = () => import('./MyComponent');

// spore.config.js
export default {
  build: {
    splitting: true,
    chunkSizeLimit: 500 // KB
  }
}`}</CodeBlock>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Minification</h3>
				<p class="text-sm text-muted-foreground mb-3">Reduce bundle size with minification</p>
				<CodeBlock label="bash" copy={`# Build with minification
spore build --minify

# Advanced minification options
spore build --minify --mangle --compress`}>{`# Build with minification
spore build --minify

# Advanced minification options
spore build --minify --mangle --compress`}</CodeBlock>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Incremental Builds</h2>

		<div class="bg-muted border border-border rounded-lg p-4 mb-6">
			<h3 class="font-semibold text-foreground mb-2">Smart Rebuilds</h3>
			<p class="text-sm text-muted-foreground mb-3">Only rebuild what has changed</p>
			<CodeBlock label="bash" copy={`# Enable incremental builds
spore build --incremental

# Watch mode with incremental builds
spore dev --incremental

# Force full rebuild
spore build --force`}>{`# Enable incremental builds
spore build --incremental

# Watch mode with incremental builds
spore dev --incremental

# Force full rebuild
spore build --force`}</CodeBlock>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Build Analysis</h2>

		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Bundle Analysis</h3>
				<p class="text-sm text-muted-foreground mb-3">Analyze bundle size and composition</p>
				<CodeBlock label="bash" copy={`# Generate bundle analysis
spore build --analyze

# Output detailed stats
spore build --stats detailed

# Save stats to file
spore build --stats-json stats.json`}>{`# Generate bundle analysis
spore build --analyze

# Output detailed stats
spore build --stats detailed

# Save stats to file
spore build --stats-json stats.json`}</CodeBlock>
			</div>

			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">Performance Metrics</h3>
				<p class="text-sm text-muted-foreground mb-3">Measure build performance</p>
				<CodeBlock label="bash" copy={`# Show build timings
spore build --timings

# Profile build process
spore build --profile

# Generate performance report
spore build --perf-report`}>{`# Show build timings
spore build --timings

# Profile build process
spore build --profile

# Generate performance report
spore build --perf-report`}</CodeBlock>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Configuration</h2>

		<div class="bg-muted rounded-lg p-4 mb-6">
			<h3 class="font-semibold mb-2">spore.config.js</h3>
			<p class="text-sm text-muted-foreground mb-3">Complete build optimization configuration</p>
			<CodeBlock label="spore.config.js">{`export default {
  build: {
    // Output configuration
    outDir: 'dist',
    sourcemap: true,
    minify: 'terser',

    // Optimization
    target: 'es2020',
    treeShake: true,
    splitting: true,

    // Performance
    parallel: true,
    workers: 4,
    incremental: true,
    cache: true,

    // Advanced
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: ['react', 'react-dom'],
          utils: ['lodash', 'date-fns']
        }
      }
    }
  }
}`}</CodeBlock>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">CI/CD Optimization</h2>

		<div class="space-y-4">
			<div class="bg-muted rounded-lg p-4">
				<h3 class="font-semibold mb-2">GitHub Actions</h3>
				<p class="text-sm text-muted-foreground mb-3">Optimize builds in CI</p>
				<CodeBlock label="yaml">{`name: Build
on: [push]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2

      - name: Cache Spore
        uses: actions/cache@v2
        with:
          path: ~/.spore/cache
          key: spore-\${{ runner.os }}-\${{ hashFiles('**/package-lock.json') }}

      - name: Install Spore
        run: curl -fsSL https://spore.space/install.sh | bash

      - name: Build
        run: spore build --parallel --cache`}</CodeBlock>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Best Practices</h2>

		<div class="space-y-3">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-muted-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use production builds for deployment</p>
					<p class="text-sm text-muted-foreground">Always use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore build --production</code> for production</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-muted-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Enable caching in CI/CD</p>
					<p class="text-sm text-muted-foreground">Cache dependencies and build outputs</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-muted-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Monitor bundle sizes</p>
					<p class="text-sm text-muted-foreground">Set up size limits and track changes</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-muted-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-medium">Use incremental builds in development</p>
					<p class="text-sm text-muted-foreground">Speed up development with smart rebuilds</p>
				</div>
			</div>
		</div>

		<h2 class="text-2xl font-bold mb-4 mt-8">Next Steps</h2>

		<div class="grid gap-4 mt-6">
			<a href="/docs/configuration" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Configuration</h3>
						<p class="text-sm text-muted-foreground">Learn about all configuration options</p>
					</div>
					<Icon icon="lucide:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>

			<a href="/docs/project-management" class="group border rounded-lg p-4 hover:bg-muted transition-colors">
				<div class="flex items-center justify-between">
					<div>
						<h3 class="font-semibold group-hover:text-primary">Project Commands</h3>
						<p class="text-sm text-muted-foreground">Explore build-related CLI commands</p>
					</div>
					<Icon icon="lucide:arrow-right-bold" class="w-5 h-5 text-muted-foreground group-hover:text-primary" />
				</div>
			</a>
		</div>
	</div>
</div>