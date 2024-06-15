/** @type {import('tailwindcss').Config} */
const { nextui } = require("@nextui-org/react");
module.exports = {
  // darkMode: "selector",
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
    "./node_modules/@nextui-org/theme/dist/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        primary: "var(--color-primary)",
        secondary: "var(--color-secondary)",
        accent: "var(--color-accent)",
        'button-primary': "var(--color-button-primary)",
        'button-secondary': "var(--color-button-secondary)",
        'icon': "var(--color-icon-primary)",
        'brand-primary': "var(--color-brand-primary)",
        'brand-secondary': "var(--color-brand-secondary)",
        'brand-disabled': "var(--color-brand-disabled)",
        'brand-accent': "var(--color-brand-accent)",
        'hover-primary': "var(--color-hover-primary)",
        'input-primary': "var(--color-input-primary)",
        'red-primary': "var(--color-red-primary)",
        'red-secondary': "var(--color-red-secondary)",
        'red-accent': "var(--color-red-accent)",
        'red-highlight': "var(--color-red-highlight)",
        'skeleton': "var(--color-skeleton)",
        'green-primary': "var(--color-green-primary)",
        'green-secondary': "var(--color-green-secondary)",
        'green-accent': "var(--color-green-accent)",
        'blue-primary': "var(--color-blue-primary)",
      },
      textColor: {
        primary: "var(--color-text-primary)",
        secondary: "var(--color-text-secondary)",
        accent: "var(--color-text-accent)",
        'icon': "var(--color-icon-primary)",
        'brand': "var(--color-brand-text)",
        'red': "var(--color-red-text)",
        'green': "var(--color-green-text)",
        'blue': "var(--color-blue-text)",
      },
      borderColor: {
        primary: "var(--color-border-primary)",
        secondary: "var(--color-border-secondary)",
        accent: "var(--color-border-accent)",
      },
      ringColor: {
        primary: "var(--color-ring-primary)",
        secondary: "var(--color-ring-secondary)",
        accent: "var(--color-ring-accent)",
      },
    },
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('flowbite/plugin'),
    nextui({
      layout: {},

      themes: {
        dark: {
          colors: {
            primary: {
              foreground: '#9333ea',
              DEFAULT: '#9333ea'
            }
          }
        },

        light: {
          colors: {
            primary: {
              foreground: '#9333ea',
              DEFAULT: '#9333ea'
            }
          }
        }
      }
    })
  ]
}