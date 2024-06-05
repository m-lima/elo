import javascript from '@eslint/js';
import prettier from 'eslint-plugin-prettier/recommended';
import typescript from 'typescript-eslint';

export default typescript.config(
  javascript.configs.recommended,
  {
    files: ['**/*.{ts,tsx}'],
    ignores: ['vite.config.ts'],

    extends: typescript.configs.strictTypeChecked,
    rules: {
      '@typescript-eslint/restrict-template-expressions': [
        'error',
        {
          allowNumber: true,
        },
      ],
      '@typescript-eslint/no-unused-vars': [
        'error',
        {
          args: 'all',
          argsIgnorePattern: '^_',
          caughtErrors: 'all',
          caughtErrorsIgnorePattern: '^_',
          destructuredArrayIgnorePattern: '^_',
          varsIgnorePattern: '^_',
          ignoreRestSiblings: true,
        },
      ],
    },

    languageOptions: {
      parserOptions: {
        project: true,
      },
    },
  },
  prettier,
);
