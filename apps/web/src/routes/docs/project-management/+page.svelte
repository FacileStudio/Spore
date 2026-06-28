<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Project Management - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to manage your Spore projects, from initialization to configuration, status checks, and project operations." />
	<meta property="og:title" content="Project Management - Spore CLI" />
	<meta property="og:description" content="Learn how to manage your Spore projects, from initialization to configuration, status checks, and project operations." />
	<meta property="og:image" content="/images/og/project-management.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/project-management" />
	<meta name="twitter:title" content="Project Management - Spore CLI" />
	<meta name="twitter:description" content="Learn how to manage your Spore projects, from initialization to configuration, status checks, and project operations." />
	<meta name="twitter:image" content="/images/og/project-management.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/project-management" />
</svelte:head>

<div class="max-w-4xl mx-auto py-12 px-4 sm:px-6">
	<div class="mb-12">
		<h1 class="text-4xl font-bold tracking-tight mb-4" style="font-family: 'Goga', sans-serif;">
			Project Management
		</h1>
		<p class="text-lg text-muted-foreground">
			Efficiently manage your monorepo with Spore's comprehensive project management commands
		</p>
	</div>

	<!-- Initializing a Project -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:rocket-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Initializing a Project</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Start a new monorepo with a single command. Spore will set up the basic structure for you, including the root <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore.yml</code> configuration file and organize your workspace.
		</p>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Create a new project</h3>
				<CodeBlock label="bash" copy="spore init my-project">spore init &lt;project-name&gt;</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					This creates a new directory with your project name and sets up the initial project structure.
				</p>
			</div>

			<div>
				<h4 class="font-semibold mb-3 flex items-center space-x-2">
					<Icon icon="lucide:folder" class="w-5 h-5 text-foreground" />
					<span>Generated Project Structure</span>
				</h4>
				<CodeBlock label="my-project/">my-project/
├── <span class="text-zinc-100">spore.yml</span>          <span class="text-zinc-500"># Project configuration</span>
├── <span class="text-zinc-100">packages/</span>        <span class="text-zinc-500"># Shared packages go here</span>
├── <span class="text-zinc-100">apps/</span>             <span class="text-zinc-500"># Applications go here</span>
├── <span class="text-zinc-400">node_modules/</span>     <span class="text-zinc-500"># Dependencies</span>
├── <span class="text-zinc-100">tsconfig.json</span>    <span class="text-zinc-500"># TypeScript configuration</span>
└── <span class="text-zinc-400">README.md</span>         <span class="text-zinc-500"># Project documentation</span></CodeBlock>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Initialize in existing directory</h3>
				<CodeBlock label="bash" copy="spore init .">spore init .</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Initialize Spore in your current directory without creating a new folder.
				</p>
			</div>
		</div>
	</section>

	<!-- Project Configuration -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:settings-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Project Configuration</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			The <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore.yml</code> file is the heart of your Spore project. It defines the project's metadata, scripts, and global configurations that apply to your entire monorepo.
		</p>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Basic spore.yml structure</h3>
				<CodeBlock label="spore.yml"><span class="text-zinc-500"># spore.yml - Project configuration</span>
<span class="text-zinc-400">name:</span> <span class="text-zinc-100">"my-awesome-project"</span>
<span class="text-zinc-400">description:</span> <span class="text-zinc-100">"A modern TypeScript monorepo"</span>
<span class="text-zinc-400">version:</span> <span class="text-zinc-100">"1.0.0"</span>
<span class="text-zinc-400">author:</span> <span class="text-zinc-100">"Your Name"</span>

<span class="text-zinc-400">scripts:</span>
  <span class="text-zinc-400">test:</span> <span class="text-zinc-100">"echo 'Running tests for all packages'"</span>
  <span class="text-zinc-400">lint:</span> <span class="text-zinc-100">"eslint . --ext .ts,.tsx,.js,.jsx"</span>
  <span class="text-zinc-400">build:</span> <span class="text-zinc-100">"spore run build --all"</span>
  <span class="text-zinc-400">dev:</span> <span class="text-zinc-100">"spore run dev --parallel"</span></CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Define project metadata and global scripts that can be run from anywhere in your monorepo.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Advanced configuration options</h3>
				<CodeBlock label="spore.yml"><span class="text-zinc-500"># Advanced spore.yml configuration</span>
<span class="text-zinc-400">workspaces:</span>
  <span class="text-zinc-400">packages:</span>
    - <span class="text-zinc-100">"packages/*"</span>
    - <span class="text-zinc-100">"tools/*"</span>
  <span class="text-zinc-400">apps:</span>
    - <span class="text-zinc-100">"apps/*"</span>
    - <span class="text-zinc-100">"examples/*"</span>

<span class="text-zinc-400">typescript:</span>
  <span class="text-zinc-400">aliases:</span>
    <span class="text-zinc-400">"@utils/*":</span> <span class="text-zinc-100">"packages/utils/src/*"</span>
    <span class="text-zinc-400">"@components/*":</span> <span class="text-zinc-100">"packages/ui/src/components/*"</span>

<span class="text-zinc-400">build:</span>
  <span class="text-zinc-400">parallel:</span> <span class="text-zinc-100">true</span>
  <span class="text-zinc-400">watch:</span> <span class="text-zinc-100">true</span>
  <span class="text-zinc-400">target:</span> <span class="text-zinc-100">"es2020"</span></CodeBlock>
			</div>
		</div>
	</section>

	<!-- Project Status -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:chart-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Checking Project Status</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Get comprehensive insights into your project's health, dependencies, and workspace organization with Spore's status commands.
		</p>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Project overview</h3>
				<CodeBlock label="bash" copy="spore status">spore status</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Shows a comprehensive overview of all packages, apps, dependencies, and their status.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Detailed workspace information</h3>
				<CodeBlock label="bash" copy="spore status --detailed">spore status --detailed</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Includes version information, build status, and dependency analysis for each workspace.
				</p>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:chart-2-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h4 class="font-semibold text-foreground mb-2">Status Information Includes</h4>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• <strong>Workspace overview</strong>: All packages and apps in your project</li>
							<li>• <strong>Dependency graph</strong>: Internal dependencies between workspaces</li>
							<li>• <strong>Build status</strong>: Which packages are built and up-to-date</li>
							<li>• <strong>Version information</strong>: Current versions of all workspaces</li>
							<li>• <strong>Link status</strong>: Which packages are properly linked</li>
						</ul>
					</div>
				</div>
			</div>
		</div>
	</section>

	<!-- Project Scripts -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:play" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Running Project Scripts</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Execute scripts across your entire monorepo or target specific workspaces with Spore's powerful script execution system.
		</p>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Run project-wide scripts</h3>
				<CodeBlock label="bash" copy="spore run build">spore run &lt;script-name&gt;</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Executes the specified script defined in your <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore.yml</code> file.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Run scripts in parallel</h3>
				<CodeBlock label="bash" copy="spore run build --parallel">spore run build --parallel</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Execute scripts across multiple workspaces simultaneously for faster builds.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Target specific workspaces</h3>
				<CodeBlock label="bash" copy={'spore run test --filter="packages/*"'}>spore run test --filter="packages/*"</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Run scripts only in workspaces that match the specified pattern.
				</p>
			</div>
		</div>
	</section>

	<!-- Project Maintenance -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:settings-minimalistic-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Project Maintenance</h2>
		</div>
		<p class="text-muted-foreground mb-6 leading-relaxed">
			Keep your monorepo healthy and optimized with maintenance commands for cleaning, updating, and reorganizing your project.
		</p>

		<div class="space-y-6">
			<div>
				<h3 class="text-lg font-semibold mb-3">Clean build artifacts</h3>
				<CodeBlock label="bash" copy="spore clean">spore clean</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Removes build outputs, temporary files, and cached data from all workspaces.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Update dependencies</h3>
				<CodeBlock label="bash" copy="spore update">spore update</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Updates dependencies across all workspaces and refreshes package links.
				</p>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-3">Validate project structure</h3>
				<CodeBlock label="bash" copy="spore validate">spore validate</CodeBlock>
				<p class="text-sm text-muted-foreground mt-2">
					Checks for common issues in project structure, dependencies, and configuration.
				</p>
			</div>
		</div>
	</section>

	<!-- Best Practices -->
	<section class="mb-12">
		<div class="flex items-center space-x-3 mb-6">
			<Icon icon="lucide:star-bold" class="w-8 h-8 text-foreground" />
			<h2 class="text-2xl font-bold">Best Practices</h2>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:check-circle" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Project Organization</h3>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Keep packages focused and single-purpose</li>
							<li>• Use consistent naming conventions</li>
							<li>• Document your project structure in README</li>
							<li>• Regularly run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore status</code> to monitor health</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:settings-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Configuration Tips</h3>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Version your <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore.yml</code> configuration</li>
							<li>• Use environment-specific scripts</li>
							<li>• Leverage TypeScript aliases for clean imports</li>
							<li>• Set up parallel builds for performance</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:shield-check-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Maintenance</h3>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Run <code class="rounded bg-muted px-1.5 py-0.5 font-mono text-sm">spore clean</code> before major builds</li>
							<li>• Validate project structure regularly</li>
							<li>• Keep dependencies up to date</li>
							<li>• Monitor build performance metrics</li>
						</ul>
					</div>
				</div>
			</div>

			<div class="bg-muted border border-border rounded-lg p-6">
				<div class="flex items-start space-x-3">
					<Icon icon="lucide:users-group-two-rounded-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
					<div>
						<h3 class="font-semibold text-foreground mb-2">Team Collaboration</h3>
						<ul class="text-sm text-muted-foreground space-y-1">
							<li>• Share consistent development scripts</li>
							<li>• Document workspace dependencies</li>
							<li>• Use project-level linting and formatting</li>
							<li>• Establish clear contribution guidelines</li>
						</ul>
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
					<Icon icon="lucide:box-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Package Development</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Learn how to create, build, and manage packages within your monorepo.
				</p>
			</a>

			<a href="/docs/configuration" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:terminal" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">CLI Commands</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Complete reference for all Spore CLI commands and their options.
				</p>
			</a>

			<a href="/docs/workflows" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:cpu-bolt-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Workflows</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Set up CI/CD workflows and automation for your Spore projects.
				</p>
			</a>

			<a href="/docs/teams" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:users-group-two-rounded-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Team Management</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Collaborate with your team and manage permissions effectively.
				</p>
			</a>

			<a href="/docs/typescript" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:code" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">TypeScript Setup</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Configure TypeScript for optimal development experience.
				</p>
			</a>

			<a href="/docs/troubleshooting" class="block border rounded-lg p-6 hover:bg-accent transition-colors">
				<div class="flex items-center space-x-3 mb-3">
					<Icon icon="lucide:bug-bold" class="w-6 h-6 text-foreground" />
					<h3 class="font-semibold">Troubleshooting</h3>
				</div>
				<p class="text-sm text-muted-foreground">
					Common issues and solutions for Spore project management.
				</p>
			</a>
		</div>
	</section>
</div>
