/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./app/src/**/*.rs"],
    },
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("@tailwindcss/aspect-ratio")],
};
