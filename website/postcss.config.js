// Why does a CSS processor tool need postcss? Well, put simply, it's not ready yet.
import postcssPresetEnv from "postcss-preset-env";
import postcssImport from "postcss-import";
import cssnano from "cssnano";
import combineSelectors from "postcss-combine-duplicated-selectors";

export default {
  plugins: [
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
