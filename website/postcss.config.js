// Why does a CSS processor tool need postcss? Well, put simply, it's not ready yet.
const postcssPresetEnv = require("postcss-preset-env");
const postcssImport = require("postcss-import");
const cssnano = require("cssnano");
const combineSelectors = require("postcss-combine-duplicated-selectors");
const postcssJitProps = require("postcss-jit-props");
const OpenProps = require("open-props");
module.exports = {
  plugins: [
    // postcssJitProps({
    //   files: ["./node_modules/open-props/open-props.min.css", "./node_modules/open-props/media.min.css"],
    // }),
    postcssImport(),
    postcssPresetEnv({
      stage: 0,
      autoprefixer: true,
      features: {
        "logical-properties-and-values": false,
        "prefers-color-scheme-query": false,
        "gap-properties": false,
        "custom-properties": false,
        "place-properties": false,
        "not-pseudo-class": false,
        "focus-visible-pseudo-class": false,
        "focus-within-pseudo-class": false,
        "color-functional-notation": false,
        "custom-media-queries": {
          preserve: false,
        },
      },
    }),
    combineSelectors(),
    cssnano({
      preset: "default",
    }),
  ],
};
