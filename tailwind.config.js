import daisyui from "daisyui";
import { addDynamicIconSelectors } from "@iconify/tailwind";
import typography from "@tailwindcss/typography";
import defaultTheme from "tailwindcss/defaultTheme";

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./app/src/**/*.rs"],
    },
    daisyui: {
        themes: ["business", "cupcake"],
    },
    theme: {
        extend: {
            fontFamily: {
                sans: ["Geist", ...defaultTheme.fontFamily.sans],
                mono: ["Geist Mono", ...defaultTheme.fontFamily.mono],
            },
        },
    },
    plugins: [typography, addDynamicIconSelectors(), daisyui],
};
