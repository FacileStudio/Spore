# Spore 🍄

A modern monorepo package manager for any programming language that simplifies dependency management and builds across multiple apps and packages.

## ✨ Features

- **🔗 Smart Package Linking** - Automatic symlinking of local packages
- **☁️ Online Package Support** - Download packages from spore space (`@package`, `@team/package`)
- **📋 Template System** - Use published packages as templates for new projects
- **⚡ Language Integration** - Per-app configuration with special TypeScript support
- **🔨 Build Management** - Execute build commands across apps or individually
- **▶️ Script Runner** - Run scripts from any config file with `spore run`
- **📦 Flexible Configuration** - YAML-based configuration with multiple syntax options
- **🌍 Multi-Language Support** - Works with any programming language (Rust, Go, Python, Java, etc.)

## 🚀 Quick Install

```bash
curl -fsSL https://raw.githubusercontent.com/saravenpi/spore/main/install.sh | bash
```

### Prerequisites
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning repositories

## 📋 Commands

### Project Management
```bash
spore init <name> [--description <desc>]    # Initialize a new project
spore status                                # Show project status
```

### Package & App Management
```bash
spore init:package <name> [--team <team>] [--template <@package>]  # Create a new package
spore init:app <name> [--template <@package>]                      # Create a new app

# Examples with templates from Spore:
spore init:app my-app --template @svelte-starter
spore init:package my-lib --template @typescript-lib
```

### Linking & Building
```bash
spore link                                  # Link packages to apps
spore build                                 # Build apps (context-aware)
spore run <script>                          # Run scripts from config files
```

### Updates & Maintenance
```bash
spore update                                # Update to latest version
spore update --force                        # Force reinstall current version
```

## 🎯 Quick Start

```bash
# 1. Initialize project
spore init my-monorepo --description "My awesome monorepo"
cd my-monorepo

# 2. Create shared packages (optionally with templates)
cd packages
spore init:package utils --team myteam
spore init:package types --team myteam --template @typescript-lib
cd ..

# 3. Create applications (optionally with templates)
cd apps
spore init:app frontend --description "Web frontend" --template @react-starter
spore init:app backend --description "API server" --template @fastify-api
cd ..

# 4. Configure dependencies and scripts in spore.yml
# 5. Link packages and build
spore link
spore build

# 6. Run development scripts
spore run dev    # Run dev script
spore run test   # Run test script
```

## ⚙️ Configuration

### Project Configuration (`spore.yml`)

```yaml
name: MyProject
description: My awesome monorepo

# Global project scripts
scripts:
  setup: "npm install && echo 'Setup complete'"
  test-all: "spore run test --all"
  clean: "rm -rf */dist */build"
  deploy: "spore build && deploy.sh"

apps:
  frontend:
    tsAlias: "@"
    packages:
      - types
      - utils
      - "@jwt"
  backend:
    tsAlias: "#"
    packages:
      - types
      - "@klysium/logger"
```

### App Configuration (`app.yml`)

```yaml
name: frontend
description: Web frontend app
tsAlias: "~"                    # Optional: Override project tsAlias (TypeScript projects)
build: "npm run build"          # Build command

# App-specific scripts
scripts:
  dev: "npm run dev"
  test: "npm run test"
  lint: "npm run lint"
  serve: "npm run serve"

packages:                       # Optional: Additional packages
  - types
  - utils
  - "@jwt"
```

### Package Configuration (`package.yml`)

```yaml
name: utils
team: myteam      # Optional: Team namespace
version: 1.0.0

# Package scripts
scripts:
  build: "npm run build"
  test: "npm run test"
  docs: "npm run docs"
  benchmark: "npm run benchmark"
```

## 🔨 Build Commands

Spore supports flexible build management with the `spore build` command:

### Context-Aware Building

- **From project root**: Builds all apps that have build commands configured
- **From app directory**: Builds only the current app

### Configuration

Add build commands to your `app.yml` files:

```yaml
name: frontend
description: Web frontend
build: "npm run build"          # Simple command
# build: "pnpm build --prod"    # Alternative build tools  
# build: "./build.sh"           # Custom scripts
# build: "cargo build --release" # Rust projects
# build: "go build -o bin/app"  # Go projects
```

### Examples

```bash
# Build all apps from project root
cd my-monorepo
spore build
# Output:
# 🔨 Building all apps for project 'MyProject'...
# 📋 Found 2 app(s) with build commands
# 🔨 Building app 'frontend'...
# 🔨 Building app 'backend'...
# 🎉 All apps built successfully!

# Build single app from app directory
cd apps/frontend
spore build
# Output:
# 🔨 Building app 'frontend'...
# 📝 Running: npm run build
# ✅ Successfully built app 'frontend'
```

## ▶️ Script Runner

Spore provides a powerful script runner similar to npm scripts, with context-aware execution:

### Context-Aware Script Resolution

Scripts are resolved in priority order:
1. **App directory** (`app.yml`) - Highest priority
2. **Package directory** (`package.yml`) - Medium priority
3. **Project root** (`spore.yml`) - Lowest priority

### Usage Examples

```bash
# Run scripts from different contexts
cd apps/frontend && spore run dev     # Runs frontend dev script
cd packages/utils && spore run test   # Runs utils test script
cd project-root && spore run setup    # Runs project setup script

# Script not found shows available options
spore run invalid-script
# Output:
# ❌ Script 'invalid-script' not found
# 💡 Available scripts:
#   📱 App scripts (frontend)
#     • dev - npm run dev
#     • test - npm run test
#     • lint - npm run lint
#   🏗️ Project scripts (MyProject)
#     • setup - npm install && echo 'Setup complete'
#     • clean - rm -rf */dist */build
```

### Script Features

- **Environment variables** - Access to full shell environment
- **Working directory** - Scripts run in their config file's directory
- **Command chaining** - Use `&&` and `||` for complex workflows
- **Cross-platform** - Works on Windows, macOS, and Linux

### Common Script Patterns

```yaml
# Development workflow
scripts:
  dev: "concurrently 'spore run dev-server' 'spore run dev-client'"
  dev-server: "npm run start:dev"
  dev-client: "npm run dev"

# Testing pipeline
scripts:
  test: "npm run test"
  test-watch: "npm run test:watch"
  test-e2e: "npm run test:e2e"
  test-all: "spore run test && spore run test-e2e"

# Build pipeline
scripts:
  prebuild: "spore run lint && spore run test"
  build: "npm run build"
  postbuild: "cp README.md dist/"
```

## 📦 Package Types

### Local Packages
- Located in `packages/` directory
- Referenced by name: `types`, `utils`
- Symlinked to `spore_packages/` in apps

### Online Packages
- **Public packages**: `@jwt`, `@axios`
- **Team packages**: `@team/package-name`
- Downloaded to `spore_packages/` in apps

## 🎨 TypeScript Integration (Optional)

Spore works with any programming language, but provides special integration features for TypeScript projects.

### Per-App Aliases
Configure TypeScript path aliases per app:

```yaml
# spore.yml
apps:
  frontend:
    tsAlias: "@"      # Creates @/* alias
  backend:
    tsAlias: "#"      # Creates #/* alias
```

### Priority Order
1. `app.yml` `tsAlias` (highest priority)
2. `spore.yml` app-specific `tsAlias`
3. `spore.yml` global `tsAlias`

### Generated TypeScript Configuration
Spore automatically updates `tsconfig.json` files:

```json
{
  "compilerOptions": {
    "paths": {
      "@/*": ["./spore_packages/*"],
      "src/*": ["./src/*"]
    }
  }
}
```

## 📁 Project Structure

```
my-monorepo/
├── spore.yml                 # Project configuration
├── packages/               # Local packages
│   ├── types/
│   │   ├── package.yml
│   │   └── src/           # Source files (any language)
│   └── utils/
│       ├── package.yml
│       └── src/           # Source files (any language)
└── apps/                   # Applications
    ├── frontend/
    │   ├── app.yml         # App configuration
    │   ├── tsconfig.json   # Auto-updated (TypeScript projects only)
    │   ├── spore_packages/  # Linked packages
    │   │   ├── types -> ../../packages/types
    │   │   ├── utils -> ../../packages/utils
    │   │   └── @jwt/       # Downloaded package
    │   └── src/
    └── backend/
        ├── app.yml
        ├── tsconfig.json       # (TypeScript projects only)
        ├── spore_packages/
        └── src/
```

## 🛠️ Manual Installation

```bash
git clone https://github.com/saravenpi/spore.git
cd spore/apps/cli
cargo build --release
sudo cp target/release/spore /usr/local/bin/
```

## 📖 Advanced Usage

### Multiple Package Sources

```yaml
# spore.yml
apps:
  frontend:
    packages:
      - types              # Local package
      - utils              # Local package
      - "@jwt"            # Public online package
      - "@myteam/logger"  # Team online package
```

### Custom Build Scripts

```yaml
# app.yml
name: frontend
build: "./scripts/build.sh"    # Custom build script
```

```bash
#!/bin/bash
# scripts/build.sh
echo "🔨 Building with custom script..."
npm run lint                    # Or: cargo clippy, go fmt, etc.
npm run test                    # Or: cargo test, go test, etc.  
npm run build:prod              # Or: cargo build --release, go build, etc.
echo "✅ Build complete!"
```

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Repository**: [https://github.com/saravenpi/spore](https://github.com/saravenpi/spore)
- **Issues**: [https://github.com/saravenpi/spore/issues](https://github.com/saravenpi/spore/issues)
- **Documentation**: [https://github.com/saravenpi/spore](https://github.com/saravenpi/spore)
