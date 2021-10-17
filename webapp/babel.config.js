const { NODE_ENV } = process.env;

module.exports = (api) => {
  api.cache.using(() => process.env.NODE_ENV);

  return {
    presets: [
      "@babel/preset-env",
      "@babel/preset-react",
      "@babel/preset-typescript",
    ],
    plugins: [
      "@babel/plugin-transform-runtime",
      "@babel/plugin-proposal-optional-chaining",
      [
        "react-css-modules",
        {
          context: "./src",
          handleMissingStyleName: "warn",
          generateScopedName: api.env("development")
            ? "[name]-[local]__[hash:base64:5]"
            : "[hash:base64:5]",
          webpackHotModuleReloading: true,
          autoResolveMultipleImports: true,
        },
      ],
    ],
  };
};
