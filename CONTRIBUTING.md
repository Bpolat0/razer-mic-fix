# Contributing to Razer Mic Fix

Thank you for your interest in contributing to Razer Mic Fix! We welcome contributions from the community and are pleased to have you help make this tool better for everyone experiencing Razer microphone volume issues.

## 🎯 Ways to Contribute

### 🐛 Bug Reports
- **Search first**: Check if the issue already exists in our [Issues](https://github.com/Bpolat0/razer-mic-fix/issues)
- **Be specific**: Include system information, steps to reproduce, and expected vs actual behavior
- **Use templates**: We provide issue templates to help you include all necessary information

### 💡 Feature Requests
- **Explain the problem**: Describe what problem the feature would solve
- **Consider alternatives**: Think about different ways to solve the same problem
- **Be realistic**: Consider the scope and complexity of the request

### 💻 Code Contributions
- **Fork and branch**: Create a feature branch from `main`
- **Follow conventions**: Use existing code style and conventions
- **Test thoroughly**: Ensure your changes work across different systems
- **Write clear commits**: Use descriptive commit messages

### 🌐 Translations
Help us make the app accessible to more users:
- Copy `src-tauri/src/i18n/en.json`
- Translate all strings to your language
- Test the translation in the app
- Submit a pull request

### 📖 Documentation
- Improve README files
- Add or update comments in code
- Write user guides or tutorials
- Fix typos or formatting issues

## 🛠️ Development Setup

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) (16 or higher)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- Git

### Getting Started
```bash
# Clone your fork
git clone https://github.com/Bpolat0/razer-mic-fix.git
cd razer-mic-fix

# Install dependencies
npm install

# Start development server
npm run tauri dev

# Build for production
npm run tauri build
```

## 📝 Pull Request Process

1. **Fork the repository** and create your branch from `main`
2. **Make your changes** with clear, focused commits
3. **Test your changes** thoroughly
4. **Update documentation** if needed
5. **Submit a pull request** with a clear description

### Pull Request Guidelines
- **One feature per PR**: Keep pull requests focused on a single feature or fix
- **Clear title**: Use a descriptive title that explains what the PR does
- **Detailed description**: Explain what changes you made and why
- **Link related issues**: Reference any related issues or discussions

## 🎨 Code Style

### Rust Code
- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to catch common issues
- Write meaningful variable and function names
- Add comments for complex logic

### JavaScript Code
- Use consistent indentation (2 spaces)
- Use meaningful variable names
- Follow modern JavaScript best practices
- Keep functions small and focused

### Commit Messages
- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

## 🧪 Testing

### Before Submitting
- Test on Windows 10 and Windows 11 if possible
- Verify the app builds successfully
- Test core functionality (volume monitoring, tray operations)
- Check that your changes don't break existing features

### Manual Testing
- Start the app and verify UI loads correctly
- Test microphone device selection
- Verify volume monitoring works
- Test system tray functionality
- Check that app starts/stops properly

## 🔒 Security Considerations

This application has access to system audio devices. When contributing:
- **Never add** data collection or telemetry
- **Avoid** network requests unless absolutely necessary
- **Minimize** system permissions required
- **Document** any new audio API usage

## 🤝 Community Guidelines

### Be Respectful
- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the community

### Be Helpful
- Help newcomers get started
- Share knowledge and experience
- Provide constructive feedback
- Be patient with questions

## 📋 Issue Labels

We use labels to help organize and prioritize issues:
- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to documentation
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `question`: Further information is requested
- `translation`: Translation-related issues

## 🚀 Release Process

### Version Numbering
We follow [Semantic Versioning](https://semver.org/):
- **MAJOR**: Incompatible API changes
- **MINOR**: Backward-compatible functionality additions
- **PATCH**: Backward-compatible bug fixes

### Release Steps
1. Update version in `src-tauri/Cargo.toml` and `src-tauri/tauri.conf.json`
2. Update CHANGELOG.md with new features and fixes
3. Create and push a new tag: `git tag v1.0.0 && git push origin v1.0.0`
4. GitHub Actions will automatically build and create a release

## 💬 Getting Help

- **Discord/Discussions**: [Create a discussion](https://github.com/Bpolat0/razer-mic-fix/discussions)
- **Issues**: For bugs and feature requests
- **Email**: For security concerns or private matters

## 📄 License

By contributing to Razer Mic Fix, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Razer Mic Fix! Your help makes this tool better for everyone in the gaming community who experiences Razer microphone volume issues.

**Made with ❤️ for the gaming community**
