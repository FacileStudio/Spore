<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Installation Guide - Spore CLI Documentation</title>
	<meta name="description" content="Install Spore CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta property="og:title" content="Installation Guide - Spore CLI" />
	<meta property="og:description" content="Install Spore CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta property="og:image" content="/images/og/installation.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/installation" />
	<meta name="twitter:title" content="Installation Guide - Spore CLI" />
	<meta name="twitter:description" content="Install Spore CLI with our one-line installer script. Works on macOS, Linux, and Windows. Includes built-in self-update functionality." />
	<meta name="twitter:image" content="/images/og/installation.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/installation" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6 overflow-x-hidden">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Installation
		</h1>
		<p class="text-lg text-muted-foreground">
			Get Spore CLI up and running on your system in minutes
		</p>
	</div>

	<!-- Quick Install -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Quick Install (Recommended)</h2>
		<div class="bg-muted border border-border rounded-xl p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:rocket" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">One-line installer</h3>
					<p class="text-sm text-muted-foreground mb-4">
						This script automatically detects your platform and installs the latest version of Spore CLI.
					</p>
					<CodeBlock label="bash" copy="curl -fsSL https://raw.githubusercontent.com/FacileStudio/Spore/main/install.sh | bash">curl -fsSL https://raw.githubusercontent.com/FacileStudio/Spore/main/install.sh | bash</CodeBlock>
				</div>
			</div>
		</div>
	</section>

	<!-- Update Command -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Keeping Spore Up to Date</h2>
		<div class="bg-muted border border-border rounded-xl p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:refresh-circle-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">Self-Upgrade Command</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Spore can upgrade itself to the latest version automatically. No need to reinstall!
					</p>
					<div class="space-y-4">
						<div>
							<div class="text-sm font-medium text-foreground mb-2">Upgrade to latest version</div>
							<CodeBlock label="bash" copy="spore upgrade">spore upgrade</CodeBlock>
						</div>
						<div>
							<div class="text-sm font-medium text-foreground mb-2">Force upgrade (reinstall current version)</div>
							<CodeBlock label="bash" copy="spore upgrade --force">spore upgrade --force</CodeBlock>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Verification -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Verify Installation</h2>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Check version</h3>
				<CodeBlock label="bash" copy="spore --version">spore --version</CodeBlock>
				<div class="mt-2 text-sm text-muted-foreground">
					Should output something like: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore v1.0.0</code>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Test basic functionality</h3>
				<CodeBlock label="bash" copy="spore --help">spore --help</CodeBlock>
				<div class="mt-2 text-sm text-muted-foreground">
					This should display version information and available commands.
				</div>
			</div>
		</div>
	</section>

	<!-- Troubleshooting -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Troubleshooting</h2>

		<div class="space-y-6">
			<div class="border border-border rounded-xl p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:shield-warning-bold" class="w-5 h-5 mr-2 text-foreground" />
					Command not found
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>If you get "command not found" after installation:</p>
					<ul class="space-y-1 ml-4">
						<li>• Check if <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">/usr/local/bin</code> is in your PATH</li>
						<li>• Try restarting your terminal</li>
						<li>• On macOS: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">sudo chown -R $(whoami) /usr/local/bin</code></li>
						<li>• Manually add to PATH: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">export PATH=$PATH:/usr/local/bin</code></li>
					</ul>
				</div>
			</div>

			<div class="border border-border rounded-xl p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:lock-bold" class="w-5 h-5 mr-2 text-foreground" />
					Permission denied
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>If you get permission errors:</p>
					<ul class="space-y-1 ml-4">
						<li>• Make sure the binary is executable: <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">chmod +x spore</code></li>
						<li>• On macOS, you might need to allow the app in System Preferences > Security</li>
						<li>• Use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">sudo</code> for system-wide installation</li>
					</ul>
				</div>
			</div>

			<div class="border border-border rounded-xl p-6">
				<h3 class="font-semibold mb-3 flex items-center">
					<Icon icon="lucide:bug-bold" class="w-5 h-5 mr-2 text-foreground" />
					Other issues
				</h3>
				<div class="space-y-3 text-sm text-muted-foreground">
					<p>For other installation problems:</p>
					<ul class="space-y-1 ml-4">
						<li>• Check our <a href="/docs/troubleshooting" class="text-foreground underline hover:no-underline">troubleshooting guide</a></li>
						<li>• Visit the <a href="https://github.com/FacileStudio/Spore/issues" class="text-foreground underline hover:no-underline">GitHub issues</a></li>
						<li>• Join our community for support</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Next Steps -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Next Steps</h2>
		<div class="bg-muted border border-border rounded-xl p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:check-circle" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div>
					<h3 class="font-semibold text-foreground mb-2">Installation complete!</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Now that Spore is installed, let's get you started with your first project.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:bg-foreground/90 transition-colors">
							<Icon icon="lucide:play" class="w-4 h-4 mr-2" />
							Quick Start Guide
						</a>
						<a href="/docs/authentication" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:key-bold" class="w-4 h-4 mr-2" />
							Setup Authentication
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
