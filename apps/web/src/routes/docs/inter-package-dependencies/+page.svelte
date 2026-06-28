<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Inter-Package Dependencies - Spore CLI Documentation</title>
	<meta name="description" content="Master inter-package dependencies in Spore. Learn how to define dependencies in package.yml, create packages that depend on other spore packages, handle mixed dependencies, and implement best practices for dependency management." />
	<meta property="og:title" content="Inter-Package Dependencies Guide - Spore CLI" />
	<meta property="og:description" content="Master inter-package dependencies in Spore. Learn how to define dependencies in package.yml, create packages that depend on other spore packages, handle mixed dependencies, and implement best practices for dependency management." />
	<meta property="og:image" content="/images/og/inter-package-dependencies.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/inter-package-dependencies" />
	<meta name="twitter:title" content="Inter-Package Dependencies Guide - Spore CLI" />
	<meta name="twitter:description" content="Master inter-package dependencies in Spore. Learn how to define dependencies in package.yml, create packages that depend on other spore packages, handle mixed dependencies, and implement best practices for dependency management." />
	<meta name="twitter:image" content="/images/og/inter-package-dependencies.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/inter-package-dependencies" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-10">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Inter-Package Dependencies
		</h1>
		<p class="text-lg text-muted-foreground">
			Comprehensive guide to managing dependencies between packages in your Spore monorepo
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Understanding Inter-Package Dependencies</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Inter-package dependencies allow packages within your monorepo to depend on each other,
			creating reusable building blocks that can be composed into larger applications.
			Spore handles both local package dependencies and external npm packages seamlessly.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-3 gap-6">
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:box" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Local Dependencies</h3>
				<p class="text-sm text-muted-foreground">
					Packages within your monorepo that depend on other local packages.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:globe" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Remote Dependencies</h3>
				<p class="text-sm text-muted-foreground">
					Published packages from Spore or other registries.
				</p>
			</div>
			<div class="border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:layers" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Mixed Dependencies</h3>
				<p class="text-sm text-muted-foreground">
					Combining local Spore packages with external npm dependencies.
				</p>
			</div>
		</div>
	</section>

	<!-- Package Dependencies in package.yml -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:file-text" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Defining Dependencies in package.yml</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">Basic dependency configuration</h3>
				<p class="text-muted-foreground mb-4">
					The <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">dependencies</code> section in your <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">package.yml</code> file defines which packages your package depends on:
				</p>

				<CodeBlock label="package.yml" copy={`name: ui-components
version: 1.0.0
description: "Reusable UI components"

dependencies:
  - types
  - utils
  - "@team/design-tokens"`}>{@html `<span class="text-zinc-400"># packages/ui-components/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">ui-components</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Reusable UI components"</span>

<span class="text-zinc-400"># Package dependencies</span>
<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">types</span>                     <span class="text-zinc-400"># Local package</span>
  - <span class="text-zinc-100">utils</span>                     <span class="text-zinc-400"># Local package</span>
  - <span class="text-zinc-100">"@team/design-tokens"</span>     <span class="text-zinc-400"># Remote package</span>`}</CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Advanced dependency specification</h3>
				<p class="text-muted-foreground mb-4">
					For more complex scenarios, you can specify version constraints, conditions, and dependency types:
				</p>

				<CodeBlock label="package.yml" copy={`name: advanced-package
version: 2.1.0

dependencies:
  - types
  - utils@^1.5.0
  - "@company/core@latest"

devDependencies:
  - "@team/testing-utils"
  - test-helpers

optionalDependencies:
  - "@team/analytics"

peerDependencies:
  - ui-framework@^3.0.0`}>{@html `<span class="text-zinc-400"># packages/advanced-package/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">advanced-package</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">2.1.0</span>

<span class="text-zinc-400"># Runtime dependencies</span>
<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">types</span>
  - <span class="text-zinc-100">utils@^1.5.0</span>              <span class="text-zinc-400"># Version constraint</span>
  - <span class="text-zinc-100">"@company/core@latest"</span>     <span class="text-zinc-400"># Latest version</span>

<span class="text-zinc-400"># Development dependencies</span>
<span class="text-zinc-400">devDependencies:</span>
  - <span class="text-zinc-100">"@team/testing-utils"</span>
  - <span class="text-zinc-100">test-helpers</span>

<span class="text-zinc-400"># Optional dependencies</span>
<span class="text-zinc-400">optionalDependencies:</span>
  - <span class="text-zinc-100">"@team/analytics"</span>         <span class="text-zinc-400"># Only if analytics enabled</span>

<span class="text-zinc-400"># Peer dependencies</span>
<span class="text-zinc-400">peerDependencies:</span>
  - <span class="text-zinc-100">ui-framework@^3.0.0</span>       <span class="text-zinc-400"># Must be provided by consumer</span>`}</CodeBlock>
			</div>
		</div>
	</section>

	<!-- Local Package Dependencies -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:home" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Local Package Dependencies</h2>
		</div>

		<div class="space-y-8">
			<p class="text-muted-foreground">
				Local dependencies are packages within your monorepo that depend on other packages in the same monorepo.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Example: Building layered architecture</h3>
				<p class="text-muted-foreground mb-4">
					Here's how to structure packages that build upon each other:
				</p>

				<div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
					<div>
						<h4 class="font-medium mb-3">Foundation Package: types</h4>
						<CodeBlock label="package.yml">{@html `<span class="text-zinc-400"># packages/types/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">types</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Shared TypeScript types"</span>

<span class="text-zinc-400"># No dependencies - foundation package</span>
<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"tsc"</span>
  <span class="text-zinc-400">check:</span> <span class="text-zinc-100">"tsc --noEmit"</span>`}</CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-3">Utility Package: utils</h4>
						<CodeBlock label="package.yml">{@html `<span class="text-zinc-400"># packages/utils/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">utils</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Utility functions"</span>

<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">types</span>                     <span class="text-zinc-400"># Uses shared types</span>

<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"tsc"</span>
  <span class="text-zinc-400">test:</span> <span class="text-zinc-100">"vitest run"</span>`}</CodeBlock>
					</div>
				</div>

				<div class="mt-6">
					<h4 class="font-medium mb-3">UI Components Package</h4>
					<CodeBlock label="package.yml" copy={`name: ui-components
version: 1.0.0
description: "Reusable UI components"

dependencies:
  - types
  - utils

scripts:
  build: "rollup -c"
  storybook: "storybook dev -p 6006"`}>{@html `<span class="text-zinc-400"># packages/ui-components/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">ui-components</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Reusable UI components"</span>

<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">types</span>                     <span class="text-zinc-400"># Shared types for props and state</span>
  - <span class="text-zinc-100">utils</span>                     <span class="text-zinc-400"># Helper functions for components</span>

<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"rollup -c"</span>
  <span class="text-zinc-400">storybook:</span> <span class="text-zinc-100">"storybook dev -p 6006"</span>`}</CodeBlock>
				</div>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="lucide:folder-tree" class="w-5 h-5 text-foreground" />
					<span>Dependency Chain</span>
				</h4>
				<pre class="text-sm font-mono"><code><span class="text-foreground">types</span> (foundation)
  ↑
<span class="text-foreground">utils</span> (depends on types)
  ↑
<span class="text-foreground">ui-components</span> (depends on types + utils)</code></pre>
			</div>
		</div>
	</section>

	<!-- Remote Package Dependencies -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:cloud" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Remote Package Dependencies</h2>
		</div>

		<div class="space-y-8">
			<p class="text-muted-foreground">
				Remote dependencies are packages published to registries like Spore, npm, or private registries.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Spore packages</h3>
				<p class="text-muted-foreground mb-4">
					Packages published to Spore use team namespaces and are prefixed with <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">@</code>:
				</p>

				<CodeBlock label="package.yml" copy={`name: shared-components
version: 1.0.0

dependencies:
  - "@acme/design-system@^2.1.0"
  - "@acme/theme-tokens@latest"
  - "@team/common-utils@~1.5.0"
  - types`}>{@html `<span class="text-zinc-400"># packages/shared-components/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">shared-components</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>

<span class="text-zinc-400">dependencies:</span>
  <span class="text-zinc-400"># Team packages from Spore</span>
  - <span class="text-zinc-100">"@acme/design-system@^2.1.0"</span>
  - <span class="text-zinc-100">"@acme/theme-tokens@latest"</span>
  - <span class="text-zinc-100">"@team/common-utils@~1.5.0"</span>

  <span class="text-zinc-400"># Local packages</span>
  - <span class="text-zinc-100">types</span>`}</CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Version constraints</h3>
				<p class="text-muted-foreground mb-4">
					Control which versions of remote packages are compatible with your package:
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-4">
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">^2.1.0</code></h4>
							<p class="text-sm text-muted-foreground">Compatible changes (2.1.0 - 3.0.0)</p>
						</div>
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">~1.5.0</code></h4>
							<p class="text-sm text-muted-foreground">Patch-level changes (1.5.0 - 1.6.0)</p>
						</div>
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">1.2.3</code></h4>
							<p class="text-sm text-muted-foreground">Exact version match</p>
						</div>
					</div>
					<div class="space-y-4">
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">latest</code></h4>
							<p class="text-sm text-muted-foreground">Always use the newest version</p>
						</div>
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">>=2.0.0</code></h4>
							<p class="text-sm text-muted-foreground">Any version 2.0.0 or higher</p>
						</div>
						<div class="border rounded p-4">
							<h4 class="font-medium mb-2"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">1.x</code></h4>
							<p class="text-sm text-muted-foreground">Any 1.x version</p>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Mixed Dependencies -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:layers" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Mixed Dependencies</h2>
		</div>

		<div class="space-y-8">
			<p class="text-muted-foreground">
				Real-world packages often combine local Spore packages with external npm packages
				and remote Spore packages for maximum flexibility.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Full-featured package example</h3>
				<CodeBlock label="package.yml" copy={`name: data-visualization
version: 2.0.0
description: "Charts and data visualization components"

dependencies:
  - types
  - utils
  - ui-components
  - "@team/theme-system@^1.0.0"
  - "@acme/math-utils@latest"
  - "d3@^7.8.0"
  - "lodash@^4.17.21"

devDependencies:
  - "@types/d3@^7.4.0"
  - "@storybook/react@^7.0.0"

peerDependencies:
  - "react@>=17.0.0"
  - "react-dom@>=17.0.0"`}>{@html `<span class="text-zinc-400"># packages/data-visualization/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">data-visualization</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">2.0.0</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Charts and data visualization components"</span>

<span class="text-zinc-400"># Runtime dependencies</span>
<span class="text-zinc-400">dependencies:</span>
  <span class="text-zinc-400"># Local packages</span>
  - <span class="text-zinc-100">types</span>                     <span class="text-zinc-400"># Shared types</span>
  - <span class="text-zinc-100">utils</span>                     <span class="text-zinc-400"># Data processing utilities</span>
  - <span class="text-zinc-100">ui-components</span>             <span class="text-zinc-400"># Base UI components</span>

  <span class="text-zinc-400"># Remote Spore packages</span>
  - <span class="text-zinc-100">"@team/theme-system@^1.0.0"</span>
  - <span class="text-zinc-100">"@acme/math-utils@latest"</span>

  <span class="text-zinc-400"># npm packages</span>
  - <span class="text-zinc-100">"d3@^7.8.0"</span>               <span class="text-zinc-400"># Chart library</span>
  - <span class="text-zinc-100">"lodash@^4.17.21"</span>         <span class="text-zinc-400"># Utilities</span>

<span class="text-zinc-400"># Development dependencies</span>
<span class="text-zinc-400">devDependencies:</span>
  - <span class="text-zinc-100">"@types/d3@^7.4.0"</span>        <span class="text-zinc-400"># TypeScript types</span>
  - <span class="text-zinc-100">"@storybook/react@^7.0.0"</span>  <span class="text-zinc-400"># Component documentation</span>

<span class="text-zinc-400"># Peer dependencies</span>
<span class="text-zinc-400">peerDependencies:</span>
  - <span class="text-zinc-100">"react@>=17.0.0"</span>          <span class="text-zinc-400"># Must be provided by app</span>
  - <span class="text-zinc-100">"react-dom@>=17.0.0"</span>

<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"rollup -c"</span>
  <span class="text-zinc-400">storybook:</span> <span class="text-zinc-100">"storybook dev"</span>
  <span class="text-zinc-400">test:</span> <span class="text-zinc-100">"vitest run"</span>`}</CodeBlock>
			</div>

			<div class="bg-muted/30 rounded-lg p-6">
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="lucide:info" class="w-5 h-5 text-foreground" />
					<span>Dependency Resolution Order</span>
				</h4>
				<p class="text-sm text-muted-foreground mb-3">
					Spore resolves dependencies in this order:
				</p>
				<div class="text-sm space-y-2">
					<div class="flex items-center space-x-3">
						<div class="w-6 h-6 bg-foreground text-background rounded-full flex items-center justify-center text-xs font-bold">1</div>
						<span>Local packages (from your monorepo)</span>
					</div>
					<div class="flex items-center space-x-3">
						<div class="w-6 h-6 bg-foreground text-background rounded-full flex items-center justify-center text-xs font-bold">2</div>
						<span>Remote Spore packages (from Spore)</span>
					</div>
					<div class="flex items-center space-x-3">
						<div class="w-6 h-6 bg-foreground text-background rounded-full flex items-center justify-center text-xs font-bold">3</div>
						<span>npm packages (from npm registry)</span>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-foreground" />
						Dependency Design
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Keep dependency trees shallow when possible</li>
						<li>• Use semantic versioning for all packages</li>
						<li>• Avoid circular dependencies between packages</li>
						<li>• Create small, focused packages with clear purposes</li>
						<li>• Document breaking changes in version bumps</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:shield" class="w-5 h-5 mr-2 text-foreground" />
						Version Management
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use specific version ranges rather than <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">*</code></li>
						<li>• Pin critical dependencies to exact versions</li>
						<li>• Regularly update and test dependency versions</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">peerDependencies</code> for shared libraries</li>
						<li>• Test across different dependency versions</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:zap" class="w-5 h-5 mr-2 text-foreground" />
						Performance
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">optionalDependencies</code> for feature flags</li>
						<li>• Minimize the number of dependencies per package</li>
						<li>• Consider bundle size impact of dependencies</li>
						<li>• Use tree-shaking friendly libraries</li>
						<li>• Lazy load heavy dependencies when possible</li>
					</ul>
				</div>

				<div class="border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:users" class="w-5 h-5 mr-2 text-foreground" />
						Team Collaboration
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Document dependency choices in package README</li>
						<li>• Use consistent naming for similar dependencies</li>
						<li>• Communicate breaking changes early</li>
						<li>• Maintain a dependency update schedule</li>
						<li>• Share reusable packages across teams</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Common Commands -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:terminal" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Managing Dependencies with CLI</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Adding dependencies</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Add a local package dependency</h4>
						<CodeBlock label="bash" copy="spore deps add types --app my-app"><span class="text-zinc-100">spore deps add types --app my-app</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Add a remote package with version</h4>
						<CodeBlock label="bash" copy={`spore deps add "@team/design-system@^2.0.0" --app frontend`}><span class="text-zinc-100">spore deps add "@team/design-system@^2.0.0" --app frontend</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Add a development dependency</h4>
						<CodeBlock label="bash" copy="spore deps add test-utils --dev --app my-package"><span class="text-zinc-100">spore deps add test-utils --dev --app my-package</span></CodeBlock>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Analyzing dependencies</h3>
				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">List all dependencies</h4>
						<CodeBlock label="bash" copy="spore deps list --app frontend"><span class="text-zinc-100">spore deps list --app frontend</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Show dependency tree</h4>
						<CodeBlock label="bash" copy="spore deps tree --app backend --depth 3"><span class="text-zinc-100">spore deps tree --app backend --depth 3</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Check for dependency issues</h4>
						<CodeBlock label="bash" copy="spore deps check"><span class="text-zinc-100">spore deps check</span></CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:alert-circle" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Troubleshooting Dependencies</h2>
		</div>

		<div class="space-y-6">
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border border-border bg-muted rounded-lg p-6">
					<h3 class="font-semibold mb-3 text-foreground">Common Issues</h3>
					<div class="space-y-4 text-sm">
						<div>
							<div class="font-medium text-foreground">Circular dependency detected</div>
							<div class="text-muted-foreground mt-1">Package A depends on B, which depends on A</div>
							<div class="text-muted-foreground mt-1"><strong>Solution:</strong> Refactor to extract shared logic into a third package</div>
						</div>
						<div>
							<div class="font-medium text-foreground">Version conflict</div>
							<div class="text-muted-foreground mt-1">Multiple packages require incompatible versions</div>
							<div class="text-muted-foreground mt-1"><strong>Solution:</strong> Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps sync</code> to align versions</div>
						</div>
						<div>
							<div class="font-medium text-foreground">Package not found</div>
							<div class="text-muted-foreground mt-1">Referenced package doesn't exist locally or remotely</div>
							<div class="text-muted-foreground mt-1"><strong>Solution:</strong> Check spelling and ensure package is published</div>
						</div>
					</div>
				</div>

				<div class="border border-border bg-muted rounded-lg p-6">
					<h3 class="font-semibold mb-3 text-foreground">Debug Commands</h3>
					<div class="space-y-3 text-sm">
						<div>
							<div class="font-medium text-foreground"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps why package-name</code></div>
							<div class="text-muted-foreground">Explain why a package is included</div>
						</div>
						<div>
							<div class="font-medium text-foreground"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps resolve --dry-run</code></div>
							<div class="text-muted-foreground">Preview dependency resolution without changes</div>
						</div>
						<div>
							<div class="font-medium text-foreground"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore link --verbose</code></div>
							<div class="text-muted-foreground">Show detailed linking information</div>
						</div>
						<div>
							<div class="font-medium text-foreground"><code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore deps outdated</code></div>
							<div class="text-muted-foreground">Check for newer package versions</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Resolving Complex Issues</h3>
				<div class="space-y-4">
					<div class="bg-muted/30 rounded-lg p-4">
						<h4 class="font-medium mb-2">1. Dependency conflicts between local and remote packages</h4>
						<p class="text-sm text-muted-foreground mb-3">
							When a local package has the same name as a remote package, local takes precedence.
						</p>
						<CodeBlock label="bash" copy="spore deps resolve --strategy latest --app affected-app">{@html `<span class="text-zinc-400"># Force resolution to use remote version</span>
<span class="text-zinc-100">spore deps resolve --strategy latest --app affected-app</span>`}</CodeBlock>
					</div>

					<div class="bg-muted/30 rounded-lg p-4">
						<h4 class="font-medium mb-2">2. TypeScript import errors after adding dependencies</h4>
						<p class="text-sm text-muted-foreground mb-3">
							TypeScript path mappings may need updating after linking packages.
						</p>
						<CodeBlock label="bash" copy="spore link --force">{@html `<span class="text-zinc-400"># Force re-linking with TypeScript config update</span>
<span class="text-zinc-100">spore link --force</span>`}</CodeBlock>
					</div>

					<div class="bg-muted/30 rounded-lg p-4">
						<h4 class="font-medium mb-2">3. Build failures due to missing peer dependencies</h4>
						<p class="text-sm text-muted-foreground mb-3">
							Some packages require peer dependencies to be installed by the consuming application.
						</p>
						<CodeBlock label="bash" copy="spore deps check --verbose">{@html `<span class="text-zinc-400"># Check what peer dependencies are missing</span>
<span class="text-zinc-100">spore deps check --verbose</span>`}</CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<a href="/docs/package-development" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to create and develop packages from scratch.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Spore links packages and manages them in your apps.
				</p>
			</a>

			<a href="/docs/publishing" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Publishing Packages</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Share your packages with your team and the community.
				</p>
			</a>

			<a href="/docs/project-management" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:settings" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Project Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Advanced project configuration and monorepo management.
				</p>
			</a>

			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code-2" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">TypeScript Integration</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Configure TypeScript for optimal development experience.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:life-buoy" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Solve common issues and debugging tips.
				</p>
			</a>
		</div>
	</section>
</div>
