
const path = require('path');

module.exports = {
  webpack: {
    configure: (webpackConfig) => {
      // Remove the ModuleScopePlugin to allow imports outside of src/
      const scopePluginIndex = webpackConfig.resolve.plugins.findIndex(
        ({ constructor }) => constructor && constructor.name === 'ModuleScopePlugin'
      );

      // If ModuleScopePlugin is found, remove it
      if (scopePluginIndex !== -1) {
        webpackConfig.resolve.plugins.splice(scopePluginIndex, 1);
      }

      // Add custom resolve path for dist directory (or wherever your external files are)
      webpackConfig.resolve.modules = [
        ...(webpackConfig.resolve.modules || []),
        path.resolve(__dirname, 'dist'), // Adjust this path based on your project
      ];

      return webpackConfig;
    }
  }
};
