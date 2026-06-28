<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Production Deployment - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to deploy your Spore projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta property="og:title" content="Production Deployment - Spore CLI" />
	<meta property="og:description" content="Learn how to deploy your Spore projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta property="og:image" content="/images/og/production.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/production" />
	<meta name="twitter:title" content="Production Deployment - Spore CLI" />
	<meta name="twitter:description" content="Learn how to deploy your Spore projects to production. Covers building for production, containerizing with Docker, and deploying to common platforms." />
	<meta name="twitter:image" content="/images/og/production.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/production" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Production Deployment
		</h1>
		<p class="text-lg text-muted-foreground">
			Deploy your Spore projects to production with confidence
		</p>
	</div>

	<!-- Building for Production -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Building for Production</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:box-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">Create a production-ready build</h3>
					<p class="text-sm text-muted-foreground mb-4">
						The <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore build</code> command will create optimized builds for all your apps.
					</p>
					<CodeBlock label="bash" copy="spore build"><code class="text-zinc-100">spore build</code></CodeBlock>
				</div>
			</div>
		</div>
	</section>

	<!-- Containerizing with Docker -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Containerizing with Docker</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:server-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">Dockerize your apps</h3>
					<p class="text-sm text-muted-foreground mb-4">
						A <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">Dockerfile</code> is provided in the <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore-space</code> app to containerize your application.
					</p>
					<CodeBlock label="bash" copy="docker build -t my-app ./apps/spore-space"><code class="text-zinc-100">docker build -t my-app ./apps/spore-space</code></CodeBlock>
				</div>
			</div>
		</div>
	</section>

	<!-- Deployment Platforms -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Deployment Platforms</h2>
		<div class="space-y-6">
			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="simple-icons:vercel" class="w-5 h-5 mr-2" />
					Deploying to Vercel
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>
						Vercel is a great platform for deploying frontend applications.
					</p>
					<ul class="space-y-1 ml-4">
						<li>• Connect your Git repository to Vercel.</li>
						<li>• Configure the build command to be <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore build</code>.</li>
						<li>• Set the output directory to <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">apps/spore/build</code>.</li>
					</ul>
				</div>
			</div>

			<div class="border rounded-lg p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="simple-icons:amazonaws" class="w-5 h-5 mr-2" />
					Deploying to AWS
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>
						Use Docker to deploy your apps to AWS services like ECS or EKS.
					</p>
					<ul class="space-y-1 ml-4">
						<li>• Build your Docker image and push it to ECR.</li>
						<li>• Create a new task definition in ECS.</li>
						<li>• Launch a new service with your task definition.</li>
					</ul>
				</div>
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
					<h3 class="font-semibold text-foreground mb-2">Ready for production!</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Now that you know how to deploy your project, let's explore more advanced topics.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/self-hosting" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:opacity-90 transition-opacity">
							<Icon icon="lucide:server-bold" class="w-4 h-4 mr-2" />
							Self-Hosting Guide
						</a>
						<a href="/docs/workflows" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:settings-bold" class="w-4 h-4 mr-2" />
							Workflow Best Practices
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
