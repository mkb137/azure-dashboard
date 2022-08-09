
module.exports = {
    parser: '@typescript-eslint/parser',
    extends: [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended',
        'plugin:@typescript-eslint/recommended-requiring-type-checking'
    ],
    parserOptions: {
        ecmaVersion: 2021,
        sourceType: 'module',
        tsconfigRootDir: __dirname,
        project: ['./tsconfig.json'],
        extraFileExtensions: ['.svelte']
    },
    env: {
        es6: true,
        browser: true
    },
    overrides: [
        {
            files: ['*.svelte'],
            processor: 'svelte3/svelte3'
        }
    ],
    settings: {
        'svelte3/typescript': () => require('typescript'),
        // ignore style tags in Svelte because of Tailwind CSS
        // See https://github.com/sveltejs/eslint-plugin-svelte3/issues/70
        'svelte3/ignore-styles': () => true
    },
    plugins: ['simple-import-sort', 'svelte3', '@typescript-eslint'],
    ignorePatterns: ['node_modules'],
    rules: {
        // note you must disable the base rule as it can report incorrect errors
        "brace-style": "off",
        "linebreak-style": [ "error", "unix" ],
        "no-use-before-define": "off",
        "indent": [
            "error",
            4,
            { 'SwitchCase': 1 }
        ],
        "jsx-quotes": [
            "error",
            "prefer-double"
        ],
        // "sort-imports": [
        //     "error", {
        //         "ignoreCase": false,
        //         "ignoreDeclarationSort": false,
        //         "ignoreMemberSort": false,
        //         "memberSyntaxSortOrder": ["none", "all", "multiple", "single"],
        //         "allowSeparatedGroups": false
        //     }
        // ],
        "simple-import-sort/imports": "error",
        "simple-import-sort/exports": "error",
        "space-before-function-paren": [
            "error",
            "never"
        ],
        "space-in-parens": [
            "error",
            "never"
        ],
        "yoda" : [
            "error",
            "always",
            { "exceptRange": true }
        ],
        "@typescript-eslint/brace-style": [
            "error",
            "1tbs"
        ],
        "@typescript-eslint/naming-convention": [
            "error",
            {
                "selector": "typeLike",
                "format": [
                    "PascalCase"
                ]
            },
            {
                "selector": "typeProperty",
                "format": [
                    "camelCase",
                    "UPPER_CASE"
                ]
            }
        ],
        "@typescript-eslint/no-explicit-any": "off",
        "@typescript-eslint/no-unused-vars": [
            "error",
            {
                "varsIgnorePattern": "^_",
                "argsIgnorePattern": "^_"
            }
        ],
        "@typescript-eslint/no-use-before-define": "off",
        "@typescript-eslint/restrict-template-expressions": "off",
        // These conflict with svelte, especially the "$store" syntax.
        "@typescript-eslint/no-unsafe-assignment": "off",
        "@typescript-eslint/no-unsafe-call": "off",
        "@typescript-eslint/no-unsafe-member-access": "off",
        "non-top-level-reactive-declaration": "off",
        // Allow "any" in arguments since Svelte's #each items seem to be treated as "any"
        "@typescript-eslint/no-unsafe-argument": "off",
        // Allow ts-ignore
        "@typescript-eslint/ban-ts-comment": "off"
    }
}
