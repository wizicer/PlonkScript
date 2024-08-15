import { defineConfig } from "vitepress";
import mathjax3 from "markdown-it-mathjax3";
import plonkscriptGrammar from "./language/plonkscript.tmLanguage.json";

// prettier-ignore
const customElements = [
  'mjx-container', 'mjx-assistive-mml', 'math', 'maction', 'maligngroup',
  'malignmark', 'menclose', 'merror', 'mfenced', 'mfrac', 'mi', 'mlongdiv',
  'mmultiscripts', 'mn', 'mo', 'mover', 'mpadded', 'mphantom', 'mroot', 'mrow',
  'ms', 'mscarries', 'mscarry', 'mscarries', 'msgroup', 'mstack', 'mlongdiv',
  'msline', 'mstack', 'mspace', 'msqrt', 'msrow', 'mstack', 'mstack', 'mstyle',
  'msub', 'msup', 'msubsup', 'mtable', 'mtd', 'mtext', 'mtr', 'munder',
  'munderover', 'semantics', 'math', 'mi', 'mn', 'mo', 'ms', 'mspace', 'mtext',
  'menclose', 'merror', 'mfenced', 'mfrac', 'mpadded', 'mphantom', 'mroot',
  'mrow', 'msqrt', 'mstyle', 'mmultiscripts', 'mover', 'mprescripts', 'msub',
  'msubsup', 'msup', 'munder', 'munderover', 'none', 'maligngroup',
  'malignmark', 'mtable', 'mtd', 'mtr', 'mlongdiv', 'mscarries', 'mscarry',
  'msgroup', 'msline', 'msrow', 'mstack', 'maction', 'semantics', 'annotation',
  'annotation-xml',
];

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "plonk.pro",
  description:
    "Plonk Script is a domain-specific scripting language designed for Plonkish arithmetic, offering enhanced readability, expressiveness, and compatibility with systems like halo2.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: "Home", link: "/" },
      { text: "Quick Start", link: "/guide/started" },
      { text: "Playground", link: "https://plonk.pro/play/index.html" },
    ],

    sidebar: [
      {
        text: "User's Guide",
        items: [
          { text: "Introduction", link: "/guide/introduction" },
          { text: "Quick Started", link: "/guide/started" },
          { text: "Online Playground", link: "/guide/playground" },
          // { text: "Benchmarks", link: "/guide/benchmarks" },
          // { text: "FAQ", link: "/guide/faq" },
          // { text: "Install", link: "/guide/install" },
          { text: "Example", link: "/guide/example" },
          { text: "Architecture", link: "/guide/architecture" },
        ],
      },
      {
        text: "Plonk Script",
        collapsed: false,
        items: [
          { text: "Basic Syntax", link: "/language/syntax" },
          { text: "Constraints", link: "/language/constraints" },
          { text: "Data Type", link: "/language/datatype" },
          { text: "Control Flow", link: "/language/controlflow" },
          // { text: "Debugging", link: "/language/debugging" },
          // { text: "Module", link: "/language/module" },
          {
            text: "Examples",
            collapsed: false,
            items: [
              { text: "Fibonacci", link: "/example/fibonacci" },
              { text: "MiMC5", link: "/example/mimc" },
              { text: "Poseidon", link: "/example/poseidon" },
            ],
          },
        ],
      },
      {
        text: "Analyzer",
        collapsed: false,
        items: [
          { text: "Halo2", link: "/analyzer/halo2" },
          { text: "Plonky2", link: "/analyzer/plonky2" },
          { text: "Plonky3", link: "/analyzer/plonky3" },
        ],
      },
      // {
      //   text: "Reference Guide",
      //   collapsed: false,
      //   items: [
      //   ],
      // },
    ],

    editLink: {
      pattern: "https://github.com/OKX-Web3/PlonkScript/edit/main/docs/content/:path",
      text: "Edit on Github",
    },

    socialLinks: [{ icon: "github", link: "https://github.com/OKX-Web3/PlonkScript" }],
  },

  markdown: {
    config: (md) => {
      md.use(mathjax3);
    },
    languages: [
      {
        id: "plonkscript",
        scopeName: "source.plonkscript",
        grammar: plonkscriptGrammar,
        aliases: ["ps", "plonk"],
      } as any,
    ],
  },
  vue: {
    template: {
      compilerOptions: {
        isCustomElement: (tag) => customElements.includes(tag),
      },
    },
  },
  vite: {
    ssr: {
      noExternal: ["vue"],
    },
  },
});
