<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	const sporercConfig = `# ~/.sporerc
SPORE_TOKEN=your-token-here
SPORE_SPACE_URL=https://spore-space-production.up.railway.app`;

	const selfHostedConfig = `export SPORE_SPACE_URL=https://your-spore-instance.com
export SPORE_TOKEN=your-auth-token`;
</script>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Authentication
		</h1>
		<p class="text-lg text-muted-foreground">
			Set up authentication to publish and manage packages with Spore CLI
		</p>
	</div>

	<!-- Overview -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">How Authentication Works</h2>
		<div class="space-y-6">
			<p class="text-muted-foreground leading-relaxed">
				Spore uses JWT (JSON Web Token) based authentication for secure package publishing and management.
				You'll need to authenticate with Spore to publish packages, manage teams, and access private packages.
			</p>

			<div class="bg-muted border border-border rounded-xl p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:shield-check-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Secure by Default</h3>
						<p class="text-sm text-muted-foreground">
							Authentication tokens are stored securely and expire automatically.
							You can revoke access at any time from your account settings.
						</p>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Getting Your Token -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Getting Your Authentication Token</h2>

		<div class="space-y-8">
			<!-- Step 1 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-foreground rounded-full flex items-center justify-center">
						<span class="text-background font-bold text-xs">1</span>
					</div>
					<h3 class="text-lg font-semibold">Create or Sign In to Your Account</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						First, you'll need a Spore account. You can create one or sign in if you already have an account.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/register" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:bg-foreground/90 transition-colors">
							<Icon icon="lucide:user-plus-bold" class="w-4 h-4 mr-2" />
							Create Account
						</a>
						<a href="/login" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:login-3-bold" class="w-4 h-4 mr-2" />
							Sign In
						</a>
					</div>
				</div>
			</div>

			<!-- Step 2 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-foreground rounded-full flex items-center justify-center">
						<span class="text-background font-bold text-xs">2</span>
					</div>
					<h3 class="text-lg font-semibold">Navigate to Settings</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						Once logged in, go to your account settings to access your authentication token.
					</p>
					<CodeBlock label="bash" copy="https://spore-space-production.up.railway.app/settings">https://spore-space-production.up.railway.app/settings</CodeBlock>
					<p class="text-sm text-muted-foreground">
						Or click the "Settings" link in your user menu after logging in.
					</p>
				</div>
			</div>

			<!-- Step 3 -->
			<div>
				<div class="flex items-center space-x-3 mb-4">
					<div class="w-6 h-6 bg-foreground rounded-full flex items-center justify-center">
						<span class="text-background font-bold text-xs">3</span>
					</div>
					<h3 class="text-lg font-semibold">Copy Your Authentication Token</h3>
				</div>
				<div class="ml-9 space-y-3">
					<p class="text-muted-foreground">
						In the settings page, you'll find your personal authentication token.
						Click the copy button to copy it to your clipboard.
					</p>
					<div class="bg-muted border border-border rounded-lg p-4">
						<div class="flex items-start space-x-2">
							<Icon icon="lucide:shield-warning-bold" class="w-5 h-5 text-foreground mt-0.5 flex-shrink-0" />
							<div>
								<div class="text-sm font-medium text-foreground mb-1">Keep Your Token Secure</div>
								<div class="text-sm text-muted-foreground">
									Your authentication token is like a password. Don't share it publicly
									or commit it to version control. Store it securely in your environment.
								</div>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Setting Up CLI -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Configure Spore CLI</h2>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Method 1: Environment Variable (Recommended)</h3>
				<div class="space-y-3">
					<p class="text-muted-foreground">
						Set your token as an environment variable. This keeps it secure and makes it available to the CLI.
					</p>

					<div class="grid grid-cols-1 md:grid-cols-2 gap-4">
						<!-- macOS/Linux -->
						<div>
							<h4 class="font-medium mb-2">macOS / Linux</h4>
							<CodeBlock label="bash" copy="export SPORE_TOKEN=your-token-here">export SPORE_TOKEN=your-token-here</CodeBlock>
							<div class="text-xs text-muted-foreground mt-1">
								Add to <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">~/.bashrc</code> or <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">~/.zshrc</code> for persistence
							</div>
						</div>

						<!-- Windows -->
						<div>
							<h4 class="font-medium mb-2">Windows</h4>
							<CodeBlock label="cmd" copy="set SPORE_TOKEN=your-token-here">set SPORE_TOKEN=your-token-here</CodeBlock>
							<div class="text-xs text-muted-foreground mt-1">
								Or use <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">setx</code> for system-wide persistence
							</div>
						</div>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Method 2: Configuration File</h3>
				<div class="space-y-3">
					<p class="text-muted-foreground">
						Alternatively, you can store your token in a configuration file.
					</p>
					<CodeBlock label="~/.sporerc" copy={sporercConfig}><span class="text-zinc-600"># ~/.sporerc</span>
<span class="text-zinc-400">SPORE_TOKEN</span>=<span class="text-zinc-100">your-token-here</span>
<span class="text-zinc-400">SPORE_SPACE_URL</span>=<span class="text-zinc-100">https://spore-space-production.up.railway.app</span></CodeBlock>
					<div class="text-sm text-muted-foreground">
						The CLI will automatically read from <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">~/.sporerc</code> if it exists.
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Verification -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Verify Authentication</h2>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Test your authentication</h3>
				<CodeBlock label="bash" copy="spore auth">spore auth</CodeBlock>
				<div class="mt-2 space-y-2">
					<div class="text-sm text-muted-foreground">
						<strong>Success:</strong> You should see your username and authentication status.
					</div>
					<div class="bg-muted border border-border rounded p-3 text-sm">
						<code class="text-foreground">✓ Authenticated as username</code>
					</div>
				</div>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Common issues</h3>
				<div class="space-y-4">
					<div class="border border-border rounded-lg p-4">
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:danger-triangle-bold" class="w-4 h-4 mr-2 text-foreground" />
							"Authentication failed"
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1 ml-6">
							<li>• Check that your token is correct and not expired</li>
							<li>• Ensure there are no extra spaces in the token</li>
							<li>• Verify the environment variable is set correctly</li>
						</ul>
					</div>

					<div class="border border-border rounded-lg p-4">
						<h4 class="font-medium mb-2 flex items-center">
							<Icon icon="lucide:wifi-router-bold" class="w-4 h-4 mr-2 text-foreground" />
							"Connection failed"
						</h4>
						<ul class="text-sm text-muted-foreground space-y-1 ml-6">
							<li>• Check your internet connection</li>
							<li>• Verify the SPORE_SPACE_URL is correct</li>
							<li>• Check if you're behind a corporate firewall</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Self-Hosted -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Self-Hosted Instances</h2>

		<div class="space-y-6">
			<p class="text-muted-foreground">
				If you're using a self-hosted Spore instance, you'll need to configure the CLI
				to point to your instance URL.
			</p>

			<div>
				<h3 class="text-lg font-semibold mb-3">Configure custom URL</h3>
				<CodeBlock label="bash" copy={selfHostedConfig}>export SPORE_SPACE_URL=https://your-spore-instance.com
export SPORE_TOKEN=your-auth-token</CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Verify connection</h3>
				<CodeBlock label="bash" copy="spore auth">spore auth</CodeBlock>
				<div class="mt-2 text-sm text-muted-foreground">
					This should connect to your self-hosted instance and verify authentication.
				</div>
			</div>
		</div>
	</section>

	<!-- Security -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6" style="font-family: 'Goga', sans-serif;">Security Best Practices</h2>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:eye-closed-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Keep Tokens Private</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Never commit tokens to version control</li>
					<li>• Don't share tokens in chat or email</li>
					<li>• Use environment variables or secure config files</li>
					<li>• Rotate tokens regularly</li>
				</ul>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:shield-network-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Team Access</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Each team member should have their own token</li>
					<li>• Use team permissions to control access</li>
					<li>• Revoke access when team members leave</li>
					<li>• Monitor authentication logs</li>
				</ul>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:refresh-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Token Management</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Tokens automatically expire for security</li>
					<li>• Generate new tokens from your settings</li>
					<li>• Old tokens are invalidated when new ones are created</li>
					<li>• Check token status with <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore auth</code></li>
				</ul>
			</div>

			<div class="border border-border rounded-xl p-6">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:server-path-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">CI/CD Integration</h3>
				</div>
				<ul class="text-sm text-muted-foreground space-y-1">
					<li>• Store tokens as encrypted secrets</li>
					<li>• Use dedicated service accounts for automation</li>
					<li>• Limit permissions for automated publishing</li>
					<li>• Monitor and audit automated access</li>
				</ul>
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
					<h3 class="font-semibold text-foreground mb-2">Authentication Complete!</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Now that you're authenticated, you can start publishing packages and collaborating with teams.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/quick-start" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:bg-foreground/90 transition-colors">
							<Icon icon="lucide:play" class="w-4 h-4 mr-2" />
							Quick Start Guide
						</a>
						<a href="/docs/publishing" class="inline-flex items-center px-4 py-2 border border-border text-foreground text-sm font-medium rounded-md hover:bg-muted transition-colors">
							<Icon icon="lucide:upload-bold" class="w-4 h-4 mr-2" />
							Publishing Packages
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>
