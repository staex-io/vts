import js from '@eslint/js'
import pluginVue from 'eslint-plugin-vue'
export default [
  js.configs.recommended,
  ...pluginVue.configs['flat/recommended'],
  {
    rules: {
      'no-unused-vars': 'error',
    },
  },
  {
    ignores: ['src/assets/declarations/**'],
  },
]
