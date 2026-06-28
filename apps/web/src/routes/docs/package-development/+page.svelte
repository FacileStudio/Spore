<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Package Development - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to create, manage, and publish packages with Spore CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta property="og:title" content="Package Development Guide - Spore CLI" />
	<meta property="og:description" content="Learn how to create, manage, and publish packages with Spore CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta property="og:image" content="/images/og/package-development.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/package-development" />
	<meta name="twitter:title" content="Package Development Guide - Spore CLI" />
	<meta name="twitter:description" content="Learn how to create, manage, and publish packages with Spore CLI. Covers package development workflows, dependency management, and publishing strategies." />
	<meta name="twitter:image" content="/images/og/package-development.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/package-development" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Package Development
		</h1>
		<p class="text-lg text-muted-foreground">
			Comprehensive guide to creating, developing, and managing packages with Spore CLI
		</p>
	</div>

	<!-- Package Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Development Workflow</h2>
		<p class="text-muted-foreground leading-relaxed mb-6">
			Spore CLI provides powerful tools for package development, from creation to publishing.
			Packages are reusable libraries that can be shared across your monorepo or published to registries
			for broader use.
		</p>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<div class="border border-border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:plus-circle" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Creation</h3>
				<p class="text-sm text-muted-foreground">
					Generate new packages with proper structure, configuration, and TypeScript setup.
				</p>
			</div>
			<div class="border border-border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:code" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Development</h3>
				<p class="text-sm text-muted-foreground">
					Build, test, and iterate on packages with watch mode and hot reloading.
				</p>
			</div>
			<div class="border border-border rounded-lg p-6">
				<div class="w-12 h-12 bg-muted rounded-lg flex items-center justify-center mb-4">
					<Icon icon="lucide:upload" class="w-6 h-6 text-foreground" />
				</div>
				<h3 class="font-semibold mb-2">Publishing</h3>
				<p class="text-sm text-muted-foreground">
					Publish packages to registries with versioning and dependency management.
				</p>
			</div>
		</div>
	</section>

	<!-- Creating Packages -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:add-circle-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Creating Packages</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate a new package</h3>
				<CodeBlock label="bash" copy="spore init:package utils">spore init:package &lt;package-name&gt;</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Creates a new package in the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/</code> directory with complete structure and configuration.
				</p>
			</div>

			<div>
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="lucide:folder" class="w-5 h-5 text-foreground" />
					<span>Generated Package Structure</span>
				</h4>
				<CodeBlock label="packages/utils"><span class="text-zinc-100">packages/utils/
├── src/
│   ├── index.ts          </span><span class="text-zinc-400"># Main entry point</span><span class="text-zinc-100">
│   └── types.ts         </span><span class="text-zinc-400"># Type definitions</span><span class="text-zinc-100">
├── tests/               </span><span class="text-zinc-400"># Test files</span><span class="text-zinc-100">
│   └── index.test.ts
├── package.yml           </span><span class="text-zinc-400"># Package configuration</span><span class="text-zinc-100">
├── package.json        </span><span class="text-zinc-400"># npm package file</span><span class="text-zinc-100">
├── tsconfig.json       </span><span class="text-zinc-400"># TypeScript config</span><span class="text-zinc-100">
├── rollup.config.js     </span><span class="text-zinc-400"># Build configuration</span><span class="text-zinc-100">
└── README.md            </span><span class="text-zinc-400"># Package documentation</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Package Development -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:code" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Package Development</h2>
		</div>

		<div class="space-y-8">
			<div>
				<h3 class="text-lg font-semibold mb-3">Development workflow commands</h3>
				<p class="text-muted-foreground mb-4">
					Navigate to your package directory to run package-specific commands:
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Build the package</h4>
						<CodeBlock label="bash" copy="cd packages/utils && spore run build">cd packages/utils && spore run build</CodeBlock>
						<p class="text-sm text-muted-foreground mt-2">
							Compiles TypeScript and bundles the package for distribution.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Watch mode for development</h4>
						<CodeBlock label="bash" copy="spore run build --watch">spore run build --watch</CodeBlock>
						<p class="text-sm text-muted-foreground mt-2">
							Automatically rebuilds when source files change.
						</p>
					</div>

					<div>
						<h4 class="font-medium mb-2">Run tests</h4>
						<CodeBlock label="bash" copy="spore run test">spore run test</CodeBlock>
						<p class="text-sm text-muted-foreground mt-2">
							Runs the test suite with coverage reporting.
						</p>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Package configuration</h3>
				<p class="text-muted-foreground mb-4">
					The <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">package.yml</code> file defines your package metadata and build configuration:
				</p>

				<CodeBlock label="package.yml"><span class="text-zinc-400"># Package metadata</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">utils</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"Shared utility functions and helpers"</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">1.0.0</span>
<span class="text-zinc-400">author:</span> <span class="text-zinc-100">"Your Team"</span>

<span class="text-zinc-400"># Build scripts</span>
<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"rollup -c"</span>
  <span class="text-zinc-400">test:</span> <span class="text-zinc-100">"vitest run"</span>
  <span class="text-zinc-400">test:watch:</span> <span class="text-zinc-100">"vitest"</span>
  <span class="text-zinc-400">lint:</span> <span class="text-zinc-100">"eslint src/ --ext .ts"</span>

<span class="text-zinc-400"># Package dependencies</span>
<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">types</span>                     <span class="text-zinc-400"># Local dependency</span>
  - <span class="text-zinc-100">"@team/shared@^1.0.0"</span>     <span class="text-zinc-400"># Remote dependency</span>

<span class="text-zinc-400"># Development dependencies</span>
<span class="text-zinc-400">devDependencies:</span>
  - <span class="text-zinc-100">"@types/node@^20.0.0"</span>     <span class="text-zinc-400"># npm types</span>

<span class="text-zinc-400"># Tags for discoverability</span>
<span class="text-zinc-400">tags:</span>
  - <span class="text-zinc-100">utilities</span>
  - <span class="text-zinc-100">helpers</span>
  - <span class="text-zinc-100">typescript</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Best Practices</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="space-y-6">
				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:check-circle" class="w-5 h-5 mr-2 text-foreground" />
						Package Design
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Keep packages focused and single-purpose</li>
						<li>• Use descriptive names and clear documentation</li>
						<li>• Follow semantic versioning strictly</li>
						<li>• Provide comprehensive TypeScript types</li>
						<li>• Include usage examples in README</li>
					</ul>
				</div>

				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:code" class="w-5 h-5 mr-2 text-foreground" />
						Development
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Use watch mode during development</li>
						<li>• Write tests before implementing features</li>
						<li>• Maintain high test coverage (>90%)</li>
						<li>• Use linting and formatting tools</li>
						<li>• Document public APIs with JSDoc</li>
					</ul>
				</div>
			</div>

			<div class="space-y-6">
				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:upload" class="w-5 h-5 mr-2 text-foreground" />
						Publishing
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Always run tests before publishing</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">--dry-run</code> to preview changes</li>
						<li>• Update CHANGELOG for each release</li>
						<li>• Tag releases in version control</li>
						<li>• Consider backward compatibility</li>
					</ul>
				</div>

				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:shield-alert" class="w-5 h-5 mr-2 text-foreground" />
						Common Pitfalls
					</h3>
					<ul class="text-sm text-muted-foreground space-y-2">
						<li>• Don't publish without testing</li>
						<li>• Avoid circular dependencies</li>
						<li>• Don't include development files in builds</li>
						<li>• Keep bundle sizes reasonable</li>
						<li>• Version dependencies appropriately</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Aliases in Development -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:link" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Using Aliases in Package Development</h2>
		</div>

		<div class="space-y-8">
			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:lightbulb" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Why Use Aliases in Package Development?</h3>
						<p class="text-sm text-muted-foreground">
							Package aliases provide clean, consistent import paths that make your packages easier to use and maintain.
							They abstract away complex package structures and enable seamless refactoring.
						</p>
					</div>
				</div>
			</div>

			<!-- Setting Up Aliases for New Packages -->
			<div class="border border-border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Setting Up Aliases for New Packages</h3>
				<p class="text-muted-foreground mb-4">
					When creating packages, consider how they will be imported and choose meaningful names that work well with aliases.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Creation with Alias-Friendly Names</h4>
						<CodeBlock label="bash" copy="spore init:package user-management">spore init:package user-management</CodeBlock>
					</div>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
						<div class="border border-border rounded p-4">
							<h4 class="font-semibold mb-2 text-foreground">Good Package Names</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">user-management</code> → <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">@/user-management</code></li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">payment-processing</code> → <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">@/payment-processing</code></li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">data-validation</code> → <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">@/data-validation</code></li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">auth-middleware</code> → <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">@/auth-middleware</code></li>
							</ul>
						</div>

						<div class="border border-border rounded p-4">
							<h4 class="font-semibold mb-2 text-foreground">Avoid These Names</h4>
							<ul class="text-sm text-muted-foreground space-y-1">
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">utils</code> (too generic)</li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">helpers</code> (unclear purpose)</li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">common</code> (not descriptive)</li>
								<li>• <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">shared</code> (overly broad)</li>
							</ul>
						</div>
					</div>
				</div>
			</div>

			<!-- Package Structure with Aliases -->
			<div class="border border-border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Package Structure for Alias Usage</h3>
				<p class="text-muted-foreground mb-4">
					Structure your packages to work optimally with the alias system and provide clear entry points.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-semibold mb-2">Example: Well-Structured Package</h4>
						<CodeBlock label="packages/user-management"><span class="text-zinc-100">packages/user-management/
├── src/
│   ├── index.ts          </span><span class="text-zinc-400"># Main exports</span><span class="text-zinc-100">
│   ├── types.ts         </span><span class="text-zinc-400"># Type definitions</span><span class="text-zinc-100">
│   ├── services/
│   │   ├── UserService.ts
│   │   └── AuthService.ts
│   └── utils/
│       ├── validation.ts
│       └── formatting.ts
├── package.yml
└── README.md</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Clear Export Structure</h4>
						<CodeBlock label="index.ts"><span class="text-zinc-400">// packages/user-management/src/index.ts</span>
<span class="text-zinc-300">export</span> {"{ "}<span class="text-zinc-100">UserService, AuthService</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'./services'</span>;
<span class="text-zinc-300">export</span> {"{ "}<span class="text-zinc-100">validateUser, formatUserName</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'./utils'</span>;
<span class="text-zinc-300">export</span> <span class="text-zinc-300">type</span> {"{ "}<span class="text-zinc-100">User, AuthToken, UserRole</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'./types'</span>;

<span class="text-zinc-400">// Clean imports from consuming apps:</span>
<span class="text-zinc-400">// import {"{ UserService, User }"} from '@/user-management';</span></CodeBlock>
					</div>
				</div>
			</div>

			<!-- Development Workflow with Aliases -->
			<div class="border border-border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-4">Development Workflow</h3>
				<p class="text-muted-foreground mb-4">
					Optimize your package development workflow by leveraging aliases and linking modes.
				</p>

				<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
					<div class="space-y-4">
						<h4 class="font-semibold">Development Phase</h4>
						<div class="border border-border rounded p-4 bg-muted">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:code" class="w-4 h-4 text-foreground" />
								<span class="font-medium text-foreground">Use Symlink Mode</span>
							</div>
							<CodeBlock label="bash" copy="spore link --symlink">spore link --symlink</CodeBlock>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Live updates while coding</li>
								<li>• Instant feedback in consuming apps</li>
								<li>• Perfect for debugging issues</li>
							</ul>
						</div>

						<div class="border border-border rounded p-4 bg-muted">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:eye" class="w-4 h-4 text-foreground" />
								<span class="font-medium text-foreground">Watch Mode Development</span>
							</div>
							<CodeBlock label="bash" copy="spore run build --watch">spore run build --watch</CodeBlock>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Automatic rebuilds on changes</li>
								<li>• Works with symlinked packages</li>
								<li>• Maintains type safety</li>
							</ul>
						</div>
					</div>

					<div class="space-y-4">
						<h4 class="font-semibold">Testing & Production</h4>
						<div class="border border-border rounded p-4 bg-muted">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:copy" class="w-4 h-4 text-foreground" />
								<span class="font-medium text-foreground">Use Copy Mode</span>
							</div>
							<CodeBlock label="bash" copy="spore link">spore link</CodeBlock>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Stable production environment</li>
								<li>• Docker and CI/CD compatible</li>
								<li>• Exact version control</li>
							</ul>
						</div>

						<div class="border border-border rounded p-4 bg-muted">
							<div class="flex items-center space-x-2 mb-2">
								<Icon icon="lucide:test-tube" class="w-4 h-4 text-foreground" />
								<span class="font-medium text-foreground">Pre-commit Testing</span>
							</div>
							<CodeBlock label="bash" copy="spore run test">spore run test</CodeBlock>
							<ul class="text-xs text-muted-foreground mt-2 space-y-1">
								<li>• Automatic package updates</li>
								<li>• Ensures consistency</li>
								<li>• Validates alias imports</li>
							</ul>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Naming Strategies -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Package Naming Strategies for Aliases</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				Choose package names that work well with your alias system and provide clear, intuitive imports for your team.
			</p>

			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:layers" class="w-5 h-5 mr-2 text-foreground" />
						Domain-Driven Naming
					</h3>
					<div class="space-y-3">
						<p class="text-sm text-muted-foreground">
							Organize packages around business domains rather than technical layers.
						</p>
						<CodeBlock><span class="text-zinc-400"># Business domain packages</span>
<span class="text-zinc-100">user-account</span>     <span class="text-zinc-400"># @/user-account</span>
<span class="text-zinc-100">order-management</span> <span class="text-zinc-400"># @/order-management</span>
<span class="text-zinc-100">product-catalog</span>  <span class="text-zinc-400"># @/product-catalog</span>
<span class="text-zinc-100">payment-gateway</span>  <span class="text-zinc-400"># @/payment-gateway</span></CodeBlock>
					</div>
				</div>

				<div class="border border-border rounded-lg p-6">
					<h3 class="font-semibold mb-3 flex items-center">
						<Icon icon="lucide:component" class="w-5 h-5 mr-2 text-foreground" />
						Feature-Based Naming
					</h3>
					<div class="space-y-3">
						<p class="text-sm text-muted-foreground">
							Name packages after specific features or capabilities they provide.
						</p>
						<CodeBlock><span class="text-zinc-400"># Feature-based packages</span>
<span class="text-zinc-100">email-templates</span>  <span class="text-zinc-400"># @/email-templates</span>
<span class="text-zinc-100">data-export</span>      <span class="text-zinc-400"># @/data-export</span>
<span class="text-zinc-100">search-filters</span>   <span class="text-zinc-400"># @/search-filters</span>
<span class="text-zinc-100">audit-logging</span>    <span class="text-zinc-400"># @/audit-logging</span></CodeBlock>
					</div>
				</div>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:lightbulb" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Naming Best Practices</h3>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Use kebab-case for consistency across systems</li>
							<li>• Include the primary noun (user, order, payment)</li>
							<li>• Add descriptive action or purpose (management, processing, validation)</li>
							<li>• Keep names between 2-3 words for readability</li>
							<li>• Test how the alias import looks: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">import ... from '@/package-name'</code></li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Package Dependencies with Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Managing Package Dependencies</h2>

		<div class="space-y-8">
			<div class="border border-border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Inter-Package Dependencies</h3>
				<p class="text-muted-foreground mb-4">
					When packages depend on other packages, aliases provide clean import paths that remain stable during refactoring.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Package Configuration</h4>
						<CodeBlock label="package.yml"><span class="text-zinc-400"># packages/user-management/package.yml</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">user-management</span>
<span class="text-zinc-400">dependencies:</span>
  - <span class="text-zinc-100">shared-types</span>       <span class="text-zinc-400"># Local package</span>
  - <span class="text-zinc-100">data-validation</span>   <span class="text-zinc-400"># Local package</span>
  - <span class="text-zinc-100">auth-middleware</span>   <span class="text-zinc-400"># Local package</span></CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Using Dependencies with Aliases</h4>
						<CodeBlock label="UserService.ts"><span class="text-zinc-400">// packages/user-management/src/UserService.ts</span>
<span class="text-zinc-300">import</span> {"{ "}<span class="text-zinc-100">User, UserRole</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/shared-types'</span>;
<span class="text-zinc-300">import</span> {"{ "}<span class="text-zinc-100">validateEmail</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/data-validation'</span>;
<span class="text-zinc-300">import</span> {"{ "}<span class="text-zinc-100">requireAuth</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/auth-middleware'</span>;

<span class="text-zinc-300">export</span> <span class="text-zinc-300">class</span> <span class="text-zinc-100">UserService</span> {"{"}
  <span class="text-zinc-300">async</span> <span class="text-zinc-100">createUser</span>(userData: <span class="text-zinc-400">Partial</span>&lt;<span class="text-zinc-400">User</span>&gt;): <span class="text-zinc-400">Promise</span>&lt;<span class="text-zinc-400">User</span>&gt; {"{"}
    <span class="text-zinc-300">if</span> (!<span class="text-zinc-100">validateEmail</span>(userData.email)) {"{"}
      <span class="text-zinc-300">throw</span> <span class="text-zinc-300">new</span> <span class="text-zinc-400">Error</span>(<span class="text-zinc-100">'Invalid email'</span>);
    {"}"}
    <span class="text-zinc-400">// Implementation...</span>
  {"}"}
{"}"}</CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Testing Packages with Aliases -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Testing Packages with Aliases</h2>

		<div class="space-y-8">
			<div class="border border-border rounded-lg p-6">
				<h3 class="text-lg font-semibold mb-3">Test Configuration</h3>
				<p class="text-muted-foreground mb-4">
					Ensure your test environment properly resolves aliases for comprehensive package testing.
				</p>

				<div class="space-y-4">
					<div>
						<h4 class="font-medium mb-2">Jest Configuration</h4>
						<CodeBlock label="jest.config.js"><span class="text-zinc-400">// packages/user-management/jest.config.js</span>
<span class="text-zinc-300">module.exports</span> = {"{"}
  <span class="text-zinc-400">moduleNameMapping:</span> {"{"}
    <span class="text-zinc-100">'^@/(.*)$'</span>: <span class="text-zinc-100">'&lt;rootDir&gt;/../$1/src'</span>
  {"}"},
  <span class="text-zinc-400">testEnvironment:</span> <span class="text-zinc-100">'node'</span>,
  <span class="text-zinc-400">roots:</span> [<span class="text-zinc-100">'&lt;rootDir&gt;/src'</span>, <span class="text-zinc-100">'&lt;rootDir&gt;/tests'</span>]
{"}"};</CodeBlock>
					</div>

					<div>
						<h4 class="font-medium mb-2">Test Files with Aliases</h4>
						<CodeBlock label="UserService.test.ts"><span class="text-zinc-400">// packages/user-management/tests/UserService.test.ts</span>
<span class="text-zinc-300">import</span> {"{ "}<span class="text-zinc-100">UserService</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'../src/UserService'</span>;
<span class="text-zinc-300">import</span> {"{ "}<span class="text-zinc-100">User</span> {"} "}<span class="text-zinc-300">from</span> <span class="text-zinc-100">'@/shared-types'</span>; <span class="text-zinc-400">// Alias works in tests</span>

<span class="text-zinc-100">describe</span>(<span class="text-zinc-100">'UserService'</span>, () => {"{"}
  <span class="text-zinc-100">test</span>(<span class="text-zinc-100">'should create user with valid data'</span>, <span class="text-zinc-300">async</span> () => {"{"}
    <span class="text-zinc-300">const</span> userData: <span class="text-zinc-400">Partial</span>&lt;<span class="text-zinc-400">User</span>&gt; = {"{"}
      email: <span class="text-zinc-100">'test@example.com'</span>,
      name: <span class="text-zinc-100">'John Doe'</span>
    {"}"};

    <span class="text-zinc-300">const</span> userService = <span class="text-zinc-300">new</span> <span class="text-zinc-100">UserService</span>();
    <span class="text-zinc-300">const</span> result = <span class="text-zinc-300">await</span> userService.<span class="text-zinc-100">createUser</span>(userData);

    <span class="text-zinc-100">expect</span>(result.email).<span class="text-zinc-100">toBe</span>(<span class="text-zinc-100">'test@example.com'</span>);
  {"}"});
{"}"});</CodeBlock>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
			<a href="/docs/aliases" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Aliases</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Comprehensive guide to configuring and using package aliases effectively.
				</p>
			</a>

			<a href="/docs/publishing" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Publishing Guide</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Detailed guide to publishing packages to Spore and other registries.
				</p>
			</a>

			<a href="/docs/inter-package-dependencies" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:layers" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Inter-Package Dependencies</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Master dependencies between packages with comprehensive examples.
				</p>
			</a>

			<a href="/docs/package-linking" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:link" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Linking</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Understand how Spore links packages and manages dependencies.
				</p>
			</a>

			<a href="/docs/typescript" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">TypeScript Setup</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Configure TypeScript for optimal development experience.
				</p>
			</a>

			<a href="/docs/project-management" class="block border border-border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:settings" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Project Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Project-level operations and monorepo management commands.
				</p>
			</a>
		</div>
	</section>
</div>
