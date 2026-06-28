
<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Self-Hosting - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to self-host your own Spore instance for complete control over your package registry." />
	<meta property="og:title" content="Self-Hosting - Spore CLI" />
	<meta property="og:description" content="Learn how to self-host your own Spore instance for complete control over your package registry." />
	<meta property="og:image" content="/images/og/self-hosting.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/self-hosting" />
	<meta name="twitter:title" content="Self-Hosting - Spore CLI" />
	<meta name="twitter:description" content="Learn how to self-host your own Spore instance for complete control over your package registry." />
	<meta name="twitter:image" content="/images/og/self-hosting.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/self-hosting" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Self-Hosting
		</h1>
		<p class="text-lg text-muted-foreground">
			Host your own private Spore instance
		</p>
	</div>

	<!-- Prerequisites -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Prerequisites</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:info" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Before you start</h3>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li>• A server with Docker installed.</li>
						<li>• A domain name pointing to your server's IP address.</li>
						<li>• A running PostgreSQL database.</li>
						<li>• Basic knowledge of the command line and Docker.</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Step 1: Clone the repository -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
				<span class="text-foreground font-bold text-sm">1</span>
			</div>
			<h2 class="text-2xl font-bold">Clone the Repository</h2>
		</div>
		<CodeBlock label="bash" copy="git clone https://github.com/FacileStudio/Spore.git">
			<code class="whitespace-nowrap block">{@html `<span class="text-zinc-100">git</span> <span class="text-zinc-400">clone</span> <span class="text-zinc-300">https://github.com/FacileStudio/Spore.git</span>`}</code>
		</CodeBlock>
	</section>

	<!-- Step 2: Configure your environment -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
				<span class="text-foreground font-bold text-sm">2</span>
			</div>
			<h2 class="text-2xl font-bold">Configure Your Environment</h2>
		</div>
		<p class="text-muted-foreground mb-3">
			Navigate to the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">apps/spore-space</code> directory and create a <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">.env</code> file from the example.
		</p>
		<CodeBlock label="bash" copy="cp .env.example .env">
			<code>{@html `<span class="text-zinc-100">cp</span> <span class="text-zinc-300">.env.example</span> <span class="text-zinc-300">.env</span>`}</code>
		</CodeBlock>
		<p class="text-muted-foreground mt-3">
			Edit the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">.env</code> file and set the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">DATABASE_URL</code> to your PostgreSQL database, and <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">JWT_SECRET</code> to a secret of your choice.
		</p>
	</section>

	<!-- Step 3: Build and Run the Docker Image -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<div class="w-8 h-8 bg-muted rounded-full flex items-center justify-center">
				<span class="text-foreground font-bold text-sm">3</span>
			</div>
			<h2 class="text-2xl font-bold">Build and Run the Docker Image</h2>
		</div>
		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Build the image</h3>
				<CodeBlock label="bash" copy="docker build -t spore-space .">
					<code>{@html `<span class="text-zinc-100">docker</span> <span class="text-zinc-400">build</span> <span class="text-zinc-100">-t</span> <span class="text-zinc-300">spore-space</span> <span class="text-zinc-100">.</span>`}</code>
				</CodeBlock>
			</div>
			<div>
				<h3 class="text-lg font-semibold mb-3">Run the container</h3>
				<CodeBlock label="bash" copy="docker run -d -p 3000:3000 --env-file .env spore-space">
					<code>{@html `<span class="text-zinc-100">docker</span> <span class="text-zinc-400">run</span> <span class="text-zinc-100">-d</span> <span class="text-zinc-100">-p</span> <span class="text-zinc-300">3000:3000</span> <span class="text-zinc-100">--env-file</span> <span class="text-zinc-300">.env</span> <span class="text-zinc-300">spore-space</span>`}</code>
				</CodeBlock>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Next Steps</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Your Spore is ready!</h3>
					<p class="text-sm text-muted-foreground mb-4">
						You can now access your private Spore instance at your domain.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/production" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:opacity-90 transition-opacity">
							<Icon icon="lucide:box-bold" class="w-4 h-4 mr-2" />
							Production Deployment
						</a>
						<a href="/docs/permissions" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:shield-user-bold" class="w-4 h-4 mr-2" />
							Permissions Guide
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
