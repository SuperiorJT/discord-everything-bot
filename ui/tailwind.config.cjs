const isPlainObject = require('lodash.isplainobject');
const colors = require('tailwindcss/colors');
const turbine = require('tailwindcss-turbine');

module.exports = {
  mode: 'jit',
  purge: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'media', // or 'media' or 'class'
  theme: {
    colors: {
      ...colors,
      transparent: 'transparent',
      current: 'currentColor'
    },
    caretColor: {
      ...colors,
      transparent: 'transparent',
      current: 'currentColor'
    },
    extend: {
      transitionDelay: {
        0: '0ms'
      },
      scale: {
        '-100': '-1'
      }
    }
  },
  plugins: [
    require('@tailwindcss/forms'),
    turbine,
    turbine({
      prefix: 'badge',
      baseStyles:
        'px-4 py-2 inline-flex justify-center items-center text-base rounded-full select-none',
      modifiers: {
        sm: 'px-3 py-0.5 text-sm',
        lg: 'px-5 py-3 text-lg'
      },
      colorStyles: color => `bg-${color}-500 text-white`,
      colorValidator: (_, values) => isPlainObject(values) && values[500]
    })
  ]
};
