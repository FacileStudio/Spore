<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	const utilsCode = `export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD'
  }).format(amount);
}

export function slugify(text: string): string {
  return text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '');
}`;

	const appCode = `import { formatCurrency, slugify } from '#/utils';

function App() {
  const price = 29.99;
  const title = "My Awesome Product";

  return (
    <div>
      <h1>{title}</h1>
      <p>Price: {formatCurrency(price)}</p>
      <p>Slug: {slugify(title)}</p>
    </div>
  );
}`;
</script>

<svelte:head>
	<title>Quick Start Guide - Spore CLI Documentation</title>
	<meta name="description" content="Get started with Spore CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta property="og:title" content="Quick Start Guide - Spore CLI" />
	<meta property="og:description" content="Get started with Spore CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta property="og:image" content="/images/og/quick-start.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/quick-start" />
	<meta name="twitter:title" content="Quick Start Guide - Spore CLI" />
	<meta name="twitter:description" content="Get started with Spore CLI in under 5 minutes. Learn to initialize projects, create packages, and link dependencies for TypeScript monorepos." />
	<meta name="twitter:image" content="/images/og/quick-start.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/quick-start" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Quick Start Guide
		</h1>
		<p class="text-lg text-muted-foreground">
			Get your first Spore project up and running in under 5 minutes
		</p>
	</div>

	<!-- Prerequisites -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Prerequisites</h2>
		<div class="bg-muted border border-border rounded-xl p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:info" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Before we start</h3>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• Node.js 18+ installed on your system</li>
						<li>• Spore CLI installed (<a href="/docs/installation" class="underline hover:no-underline">installation guide</a>)</li>
						<li>• A text editor or IDE (VS Code recommended)</li>
						<li>• Basic knowledge of command-line tools</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 1 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-foreground rounded-full flex items-center justify-center">
				<span class="text-background font-bold text-sm">1</span>
			</div>
			<h2 class="text-2xl font-bold" style="font-family: 'Goga', sans-serif;">Create Your First Project</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Initialize a new project</h3>
				<CodeBlock label="bash" copy="spore init my-awesome-project">spore init my-awesome-project</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new directory with your project structure and configuration files.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Navigate to your project</h3>
				<CodeBlock label="bash" copy="cd my-awesome-project">cd my-awesome-project</CodeBlock>
			</div>

			<div>
				<h4 class="font-semibold mb-3">Project structure created</h4>
				<CodeBlock label="my-awesome-project"><span class="text-zinc-100">my-awesome-project/</span>
├── <span class="text-zinc-300">spore.yml</span>          <span class="text-zinc-600"># Project configuration</span>
├── <span class="text-zinc-300">packages/</span>        <span class="text-zinc-600"># Your packages will go here</span>
├── <span class="text-zinc-300">apps/</span>             <span class="text-zinc-600"># Your applications will go here</span>
└── <span class="text-zinc-300">README.md</span>         <span class="text-zinc-600"># Getting started guide</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Step 2 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-foreground rounded-full flex items-center justify-center">
				<span class="text-background font-bold text-sm">2</span>
			</div>
			<h2 class="text-2xl font-bold" style="font-family: 'Goga', sans-serif;">Create Your First Package</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate a utility package</h3>
				<CodeBlock label="bash" copy="spore init:package utils">spore init:package utils</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new package in the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/utils/</code> directory.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Add some utility functions</h3>
				<p class="text-muted-foreground mb-3">
					Edit <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">packages/utils/src/index.ts</code> and add your first utility:
				</p>
				<CodeBlock label="index.ts" copy={utilsCode}>{@html `<span class="text-zinc-400">export</span> <span class="text-zinc-300">function</span> <span class="text-zinc-100">formatCurrency</span>(<span class="text-zinc-100">amount</span>: <span class="text-zinc-400">number</span>): <span class="text-zinc-400">string</span> {
  <span class="text-zinc-300">return</span> <span class="text-zinc-100">new</span> <span class="text-zinc-100">Intl</span>.<span class="text-zinc-100">NumberFormat</span>(<span class="text-zinc-100">'en-US'</span>, {
    <span class="text-zinc-400">style</span>: <span class="text-zinc-100">'currency'</span>,
    <span class="text-zinc-400">currency</span>: <span class="text-zinc-100">'USD'</span>
  }).<span class="text-zinc-100">format</span>(<span class="text-zinc-100">amount</span>);
}

<span class="text-zinc-400">export</span> <span class="text-zinc-300">function</span> <span class="text-zinc-100">slugify</span>(<span class="text-zinc-100">text</span>: <span class="text-zinc-400">string</span>): <span class="text-zinc-400">string</span> {
  <span class="text-zinc-300">return</span> <span class="text-zinc-100">text</span>
    .<span class="text-zinc-100">toLowerCase</span>()
    .<span class="text-zinc-100">replace</span>(<span class="text-zinc-100">/[^a-z0-9]+/g</span>, <span class="text-zinc-100">'-'</span>)
    .<span class="text-zinc-100">replace</span>(<span class="text-zinc-100">/^-+|-+$/g</span>, <span class="text-zinc-100">''</span>);
}`}</CodeBlock>
			</div>
		</div>
	</section>

	<!-- Step 3 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-foreground rounded-full flex items-center justify-center">
				<span class="text-background font-bold text-sm">3</span>
			</div>
			<h2 class="text-2xl font-bold" style="font-family: 'Goga', sans-serif;">Create Your First App</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Generate a React app</h3>
				<CodeBlock label="bash" copy="spore init:app frontend">spore init:app frontend</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new app in the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">apps/frontend/</code> directory.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Install the package in your app</h3>
				<p class="text-muted-foreground mb-3">
					Navigate to your app directory and install the utils package:
				</p>
				<CodeBlock label="bash" copy="cd apps/frontend && spore install utils">cd apps/frontend && spore install utils</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This adds the utils package to your app and links it automatically.
				</p>
			</div>
		</div>
	</section>

	<!-- Step 4 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-foreground rounded-full flex items-center justify-center">
				<span class="text-background font-bold text-sm">4</span>
			</div>
			<h2 class="text-2xl font-bold" style="font-family: 'Goga', sans-serif;">Link Packages and Setup TypeScript</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Link your packages</h3>
				<CodeBlock label="bash" copy="spore link">spore link</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This copies your packages to your apps and sets up TypeScript path mapping automatically.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Use your package in the app</h3>
				<p class="text-muted-foreground mb-3">
					Edit <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">apps/frontend/src/App.tsx</code> to use your utilities:
				</p>
				<CodeBlock label="App.tsx" copy={appCode}>{@html `<span class="text-zinc-400">import</span> { <span class="text-zinc-100">formatCurrency</span>, <span class="text-zinc-100">slugify</span> } <span class="text-zinc-400">from</span> <span class="text-zinc-100">'#/utils'</span>;

<span class="text-zinc-400">function</span> <span class="text-zinc-100">App</span>() {
  <span class="text-zinc-300">const</span> <span class="text-zinc-100">price</span> = <span class="text-zinc-100">29.99</span>;
  <span class="text-zinc-300">const</span> <span class="text-zinc-100">title</span> = <span class="text-zinc-100">"My Awesome Product"</span>;

  <span class="text-zinc-300">return</span> (
    &lt;<span class="text-zinc-400">div</span>&gt;
      &lt;<span class="text-zinc-400">h1</span>&gt;{<span class="text-zinc-100">title</span>}&lt;/<span class="text-zinc-400">h1</span>&gt;
      &lt;<span class="text-zinc-400">p</span>&gt;Price: {<span class="text-zinc-100">formatCurrency</span>(<span class="text-zinc-100">price</span>)}&lt;/<span class="text-zinc-400">p</span>&gt;
      &lt;<span class="text-zinc-400">p</span>&gt;Slug: {<span class="text-zinc-100">slugify</span>(<span class="text-zinc-100">title</span>)}&lt;/<span class="text-zinc-400">p</span>&gt;
    &lt;/<span class="text-zinc-400">div</span>&gt;
  );
}`}</CodeBlock>
			</div>
		</div>
	</section>

	<!-- Step 5 -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-foreground rounded-full flex items-center justify-center">
				<span class="text-background font-bold text-sm">5</span>
			</div>
			<h2 class="text-2xl font-bold" style="font-family: 'Goga', sans-serif;">Build and Run</h2>
		</div>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Install dependencies</h3>
				<CodeBlock label="bash" copy="cd apps/frontend && npm install">cd apps/frontend && npm install</CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Start development server</h3>
				<CodeBlock label="bash" copy="spore run dev">spore run dev</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This runs the dev script from your app configuration. Your app should be available at <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">http://localhost:3000</code>
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Build for production</h3>
				<CodeBlock label="bash" copy="spore run build">spore run build</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This builds all your apps for production deployment.
				</p>
			</div>
		</div>
	</section>

	<!-- Success -->
	<section class="mb-12">
		<div class="bg-muted border border-border rounded-xl p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Congratulations! 🎉</h3>
					<p class="text-sm text-muted-foreground mb-4">
						You've successfully created your first Spore monorepo with a shared package and a React app that uses it.
						Your utilities are automatically typed and available with clean imports.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/package-linking" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:bg-foreground/90 transition-colors">
							<Icon icon="lucide:link" class="w-4 h-4 mr-2" />
							Learn Package Linking
						</a>
						<a href="/docs/publishing" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:upload-bold" class="w-4 h-4 mr-2" />
							Publish Your Package
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Next Steps</h2>
		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:settings-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Advanced Configuration</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Learn about advanced configuration options, custom TypeScript aliases, and build optimization.
				</p>
				<a href="/docs/configuration" class="text-sm text-foreground underline hover:no-underline">
					Configuration Guide →
				</a>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:users-group-two-rounded-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Team Collaboration</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Set up teams, manage permissions, and collaborate on packages with your team members.
				</p>
				<a href="/docs/teams" class="text-sm text-foreground underline hover:no-underline">
					Team Management →
				</a>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:upload-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Publishing</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Share your packages with the world by publishing them to Spore registry.
				</p>
				<a href="/docs/publishing" class="text-sm text-foreground underline hover:no-underline">
					Publishing Guide →
				</a>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:server-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Self-Hosting</h3>
				</div>
				<p class="text-sm text-muted-foreground mb-4">
					Host your own private Spore instance for complete control over your package registry.
				</p>
				<a href="/docs/self-hosting" class="text-sm text-foreground underline hover:no-underline">
					Self-Hosting Guide →
				</a>
			</div>
		</div>
	</section>
</div>
