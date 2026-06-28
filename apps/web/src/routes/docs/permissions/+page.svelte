<script lang="ts">
	import Icon from '@iconify/svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
</script>

<svelte:head>
	<title>Permissions - Spore CLI Documentation</title>
	<meta name="description" content="Learn how to manage permissions for your Spore projects and packages." />
	<meta property="og:title" content="Permissions - Spore CLI" />
	<meta property="og:description" content="Learn how to manage permissions for your Spore projects and packages." />
	<meta property="og:image" content="/images/og/permissions.png" />
	<meta property="og:url" content="https://spore.facile.studio/docs/permissions" />
	<meta name="twitter:title" content="Permissions - Spore CLI" />
	<meta name="twitter:description" content="Learn how to manage permissions for your Spore projects and packages." />
	<meta name="twitter:image" content="/images/og/permissions.png" />
	<link rel="canonical" href="https://spore.facile.studio/docs/permissions" />
</svelte:head>

<div class="max-w-4xl mx-auto py-8 px-4 sm:px-6">
	<div class="mb-8">
		<h1 class="text-3xl font-bold tracking-tight mb-4" style="font-family: 'Goga', 'Satoshi', sans-serif;">
			Permissions
		</h1>
		<p class="text-lg text-muted-foreground">
			Manage access to your projects and packages
		</p>
	</div>

	<!-- Roles -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Roles</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:user-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">User Roles</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Spore uses a role-based access control system to manage permissions. The following roles are available:
					</p>
					<ul class="text-sm text-muted-foreground space-y-1">
						<li><b>Owner:</b> Can manage all aspects of the project, including billing and team members.</li>
						<li><b>Admin:</b> Can manage packages and team members.</li>
						<li><b>Developer:</b> Can publish and manage packages.</li>
						<li><b>Guest:</b> Can view packages.</li>
					</ul>
				</div>
			</div>
		</div>
	</section>

	<!-- Managing Permissions -->
	<section class="mb-12">
		<h2 class="text-2xl font-bold mb-6">Managing Permissions</h2>
		<div class="bg-muted border border-border rounded-lg p-6">
			<div class="flex items-start space-x-3">
				<Icon icon="lucide:shield-user-bold" class="w-6 h-6 text-foreground mt-1 flex-shrink-0" />
				<div class="flex-1 min-w-0">
					<h3 class="font-semibold text-foreground mb-2">Team Management</h3>
					<p class="text-sm text-muted-foreground mb-4">
						You can manage your team members and their roles from the Spore dashboard.
					</p>
					<div class="space-y-3">
						<div>
							<div class="text-sm font-medium text-foreground mb-1">Invite a team member</div>
							<CodeBlock label="bash" copy="spore team:invite user@example.com --role developer"><span class="text-zinc-100">spore team:invite user@example.com --role developer</span></CodeBlock>
						</div>
					</div>
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
					<h3 class="font-semibold text-foreground mb-2">Ready to collaborate?</h3>
					<p class="text-sm text-muted-foreground mb-4">
						Now that you understand how to manage permissions, you can start collaborating with your team.
					</p>
					<div class="flex flex-wrap gap-3">
						<a href="/docs/teams" class="inline-flex items-center px-4 py-2 bg-foreground text-background text-sm font-medium rounded-md hover:bg-foreground/90 transition-colors">
							<Icon icon="lucide:users-group-two-rounded-bold" class="w-4 h-4 mr-2" />
							Team Management
						</a>
					</div>
				</div>
			</div>
		</div>
	</section>
</div>