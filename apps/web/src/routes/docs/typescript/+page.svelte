<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>TypeScript & JavaScript - Spore CLI Documentation</title>
	<meta name="description" content="Build and manage TypeScript and JavaScript packages with Spore. Learn about type definitions, builds, and best practices." />
	<meta property="og:title" content="TypeScript & JavaScript - Spore CLI" />
	<meta property="og:description" content="Build and manage TypeScript and JavaScript packages with Spore. Learn about type definitions, builds, and best practices." />
	<meta property="og:url" content="https://spore.facile.studio/docs/typescript" />
	<meta name="twitter:title" content="TypeScript & JavaScript - Spore CLI" />
	<meta name="twitter:description" content="Build and manage TypeScript and JavaScript packages with Spore. Learn about type definitions, builds, and best practices." />
	<link rel="canonical" href="https://spore.facile.studio/docs/typescript" />
</svelte:head>

<div class="max-w-4xl mx-auto py-12 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			TypeScript & JavaScript
		</h1>
		<p class="text-lg text-muted-foreground">
			Build and manage TypeScript and JavaScript packages with Spore.
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Overview</h2>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Spore provides first-class support for TypeScript and JavaScript packages, including automatic type generation,
			module resolution, and seamless integration with popular build tools.
		</p>
	</section>

	<!-- TypeScript Configuration -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">TypeScript Configuration</h2>

		<div>
			<h3 class="text-lg font-semibold mb-3">Recommended tsconfig.json</h3>
			<CodeBlock label="tsconfig.json">{@html `{
  <span class="text-zinc-400">"compilerOptions"</span>: {
    <span class="text-zinc-400">"target"</span>: <span class="text-zinc-100">"ES2020"</span>,
    <span class="text-zinc-400">"module"</span>: <span class="text-zinc-100">"ESNext"</span>,
    <span class="text-zinc-400">"lib"</span>: [<span class="text-zinc-100">"ES2020"</span>],
    <span class="text-zinc-400">"moduleResolution"</span>: <span class="text-zinc-100">"bundler"</span>,
    <span class="text-zinc-400">"declaration"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"declarationMap"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"sourceMap"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"outDir"</span>: <span class="text-zinc-100">"./dist"</span>,
    <span class="text-zinc-400">"rootDir"</span>: <span class="text-zinc-100">"./src"</span>,
    <span class="text-zinc-400">"strict"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"esModuleInterop"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"skipLibCheck"</span>: <span class="text-zinc-100">true</span>,
    <span class="text-zinc-400">"forceConsistentCasingInFileNames"</span>: <span class="text-zinc-100">true</span>
  },
  <span class="text-zinc-400">"include"</span>: [<span class="text-zinc-100">"src/**/*"</span>],
  <span class="text-zinc-400">"exclude"</span>: [<span class="text-zinc-100">"node_modules"</span>, <span class="text-zinc-100">"dist"</span>, <span class="text-zinc-100">"**/*.test.ts"</span>]
}`}</CodeBlock>
		</div>
	</section>

	<!-- Package.json Setup -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package.json Setup</h2>

		<div>
			<h3 class="text-lg font-semibold mb-3">Example package.json</h3>
			<CodeBlock label="package.json">{@html `{
  <span class="text-zinc-400">"name"</span>: <span class="text-zinc-100">"@your-org/package-name"</span>,
  <span class="text-zinc-400">"version"</span>: <span class="text-zinc-100">"1.0.0"</span>,
  <span class="text-zinc-400">"type"</span>: <span class="text-zinc-100">"module"</span>,
  <span class="text-zinc-400">"main"</span>: <span class="text-zinc-100">"./dist/index.js"</span>,
  <span class="text-zinc-400">"module"</span>: <span class="text-zinc-100">"./dist/index.js"</span>,
  <span class="text-zinc-400">"types"</span>: <span class="text-zinc-100">"./dist/index.d.ts"</span>,
  <span class="text-zinc-400">"exports"</span>: {
    <span class="text-zinc-400">"."</span>: {
      <span class="text-zinc-400">"types"</span>: <span class="text-zinc-100">"./dist/index.d.ts"</span>,
      <span class="text-zinc-400">"import"</span>: <span class="text-zinc-100">"./dist/index.js"</span>,
      <span class="text-zinc-400">"require"</span>: <span class="text-zinc-100">"./dist/index.cjs"</span>
    }
  },
  <span class="text-zinc-400">"scripts"</span>: {
    <span class="text-zinc-400">"build"</span>: <span class="text-zinc-100">"spore build"</span>,
    <span class="text-zinc-400">"test"</span>: <span class="text-zinc-100">"spore test"</span>,
    <span class="text-zinc-400">"lint"</span>: <span class="text-zinc-100">"spore lint"</span>,
    <span class="text-zinc-400">"dev"</span>: <span class="text-zinc-100">"spore dev"</span>
  },
  <span class="text-zinc-400">"devDependencies"</span>: {
    <span class="text-zinc-400">"typescript"</span>: <span class="text-zinc-100">"^5.0.0"</span>,
    <span class="text-zinc-400">"@types/node"</span>: <span class="text-zinc-100">"^20.0.0"</span>
  }
}`}</CodeBlock>
		</div>
	</section>

	<!-- Building Packages -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Building Packages</h2>

		<div class="space-y-6">
			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:hammer-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div class="flex-1">
						<h3 class="font-semibold text-foreground mb-2">Build Command</h3>
						<div class="mb-3">
							<CodeBlock label="bash" copy="spore build">spore build</CodeBlock>
						</div>
						<p class="text-sm text-muted-foreground">
							Automatically detects and builds TypeScript files, generates type definitions, and bundles your package.
						</p>
					</div>
				</div>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:eye-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div class="flex-1">
						<h3 class="font-semibold text-foreground mb-2">Watch Mode</h3>
						<div class="mb-3">
							<CodeBlock label="bash" copy="spore dev">spore dev</CodeBlock>
						</div>
						<p class="text-sm text-muted-foreground">
							Watches for file changes and rebuilds automatically during development.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Type Definitions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Type Definitions</h2>

		<p class="text-muted-foreground mb-6 leading-relaxed">
			Spore automatically generates and includes TypeScript type definitions when you publish packages.
			Ensure your <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">tsconfig.json</code> has <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">"declaration": true</code> enabled.
		</p>

		<div>
			<h3 class="text-lg font-semibold mb-3">Publishing with Types</h3>
			<CodeBlock label="bash" copy={`spore build\nspore publish`}>{@html `<span class="text-zinc-400"># Build and publish with type definitions</span>
spore build
spore publish

<span class="text-zinc-400"># Types are automatically included in the package</span>`}</CodeBlock>
		</div>
	</section>

	<!-- Module Formats -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Module Formats</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">ESM (ECMAScript Modules)</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-3">Modern standard, recommended for new projects</p>
				<CodeBlock label="typescript" copy="import { myFunction } from '@org/package';">{@html `<span class="text-zinc-400">import</span> { <span class="text-zinc-100">myFunction</span> } <span class="text-zinc-400">from</span> <span class="text-zinc-100">'@org/package'</span>;`}</CodeBlock>
			</div>

			<div class="border rounded-lg p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code-2" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">CommonJS</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-3">Legacy format, supported for compatibility</p>
				<CodeBlock label="javascript" copy="const { myFunction } = require('@org/package');">{@html `<span class="text-zinc-400">const</span> { <span class="text-zinc-100">myFunction</span> } = <span class="text-zinc-300">require</span>(<span class="text-zinc-100">'@org/package'</span>);`}</CodeBlock>
			</div>
		</div>
	</section>

	<!-- Testing -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Testing</h2>

		<div>
			<h3 class="text-lg font-semibold mb-3">Running Tests</h3>
			<CodeBlock label="bash" copy="spore test">{@html `<span class="text-zinc-400"># Run all tests</span>
spore test

<span class="text-zinc-400"># Run tests in watch mode</span>
spore test --watch

<span class="text-zinc-400"># Run tests with coverage</span>
spore test --coverage`}</CodeBlock>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>

		<div class="space-y-4">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Use strict TypeScript settings</p>
					<p class="text-sm text-muted-foreground">Enable strict mode for better type safety and catching potential issues early.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Include source maps</p>
					<p class="text-sm text-muted-foreground">Help with debugging in production and development environments.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Export types explicitly</p>
					<p class="text-sm text-muted-foreground">Make your API surface clear and documented for better developer experience.</p>
				</div>
			</div>

			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
				<div>
					<p class="font-semibold">Use semantic versioning</p>
					<p class="text-sm text-muted-foreground">Follow semver conventions for version updates to ensure compatibility.</p>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Publishing Packages</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to publish your TypeScript packages to the registry.
				</p>
			</a>

			<a href="/docs/package-development" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:terminal" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Explore all package-related CLI commands and options.
				</p>
			</a>
		</div>
	</section>
</div>
