// @ts-check
// `@type` JSDoc annotations allow editor autocompletion and type checking
// (when paired with `@ts-check`).
// There are various equivalent ways to declare your Docusaurus config.
// See: https://docusaurus.io/docs/api/docusaurus-config

import { themes as prismThemes } from "prism-react-renderer";

const siteUrl = "https://sumi.rs";
const siteName = "git-sumi";
const tagline = "The non-opinionated Rust-based commit message linter.";

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: "git-sumi",
  tagline: tagline,
  favicon: "img/logo.png",

  // Set the production url of your site here
  url: siteUrl,
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: "",

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: "welpo",
  projectName: "git-sumi",
  deploymentBranch: "gh-pages",

  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",

  // Even if you don't use internationalization, you can use this field to set
  // useful metadata like html lang. For example, if your site is Chinese, you
  // may want to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: "en",
    locales: ["en"],
  },

  themes: [
    [
      require.resolve("@easyops-cn/docusaurus-search-local"),
      /** @type {import("@easyops-cn/docusaurus-search-local").PluginOptions} */
      ({
        // `hashed` is recommended as long-term-cache of index file is possible.
        hashed: true,
      }),
    ],
  ],

  presets: [
    [
      "classic",
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: "./sidebars.js",
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: "https://github.com/welpo/git-sumi/tree/main/website/",
        },
        theme: {
          customCss: "./src/css/custom.css",
        },
      }),
    ],
  ],

  headTags: [
    {
      tagName: "meta",
      attributes: { "http-equiv": "X-UA-Compatible", content: "IE=edge" },
    },
    {
      tagName: "meta",
      attributes: {
        "http-equiv": "content-type",
        content: "text/html; charset=utf-8",
      },
    },

    // Open Graph meta tags.
    {
      tagName: "meta",
      attributes: { property: "og:type", content: "website" },
    },
    {
      tagName: "meta",
      attributes: { property: "og:url", content: siteUrl },
    },
    {
      tagName: "meta",
      attributes: { property: "og:title", content: siteName },
    },
    {
      tagName: "meta",
      attributes: {
        property: "og:description",
        content: tagline,
      },
    },
    {
      tagName: "meta",
      attributes: {
        name: "description",
        content: tagline,
      },
    },
    {
      tagName: "meta",
      attributes: { property: "og:image", content: "/img/social-card.jpg" },
    },
    {
      tagName: "meta",
      attributes: { property: "og:image:height", content: "1024" },
    },
    {
      tagName: "meta",
      attributes: { property: "og:image:width", content: "1792" },
    },

    // Twitter card.
    {
      tagName: "meta",
      attributes: { name: "twitter:image", content: "/img/social-card.jpg" },
    },
    {
      tagName: "meta",
      attributes: { name: "twitter:card", content: "summary_large_image" },
    },

    {
      tagName: "meta",
      attributes: {
        name: "viewport",
        content: "width=device-width, initial-scale=1.0",
      },
    },
    {
      tagName: "meta",
      attributes: { content: "light dark", name: "color-scheme" },
    },
    {
      tagName: "meta",
      attributes: {
        media: "(prefers-color-scheme: light)",
        content: "#73304F",
        name: "theme-color",
      },
    },
    {
      tagName: "meta",
      attributes: {
        media: "(prefers-color-scheme: dark)",
        content: "#272731",
        name: "theme-color",
      },
    },

    // Icons.
    {
      tagName: "link",
      attributes: { rel: "apple-touch-icon", href: "/img/logo.png" },
    },
    { tagName: "link", attributes: { rel: "icon", href: "/img/logo.png" } },
  ],

  scripts: [
    {
      src: "https://stats.sumi.rs/count.js",
      async: true,
      "data-goatcounter": "https://stats.sumi.rs/count",
    },
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      image: "img/social-card.jpg",
      navbar: {
        title: "git-sumi",
        logo: {
          alt: "git-sumi logo: a lantern held on a bamboo stick over the sea",
          src: "img/logo.png",
        },
        items: [
          {
            type: "docSidebar",
            sidebarId: "tutorialSidebar",
            position: "left",
            label: "Documentation",
          },
          {
            href: "https://github.com/welpo/git-sumi",
            "aria-label": "GitHub",
            className: "header-github-link",
            position: "right",
          },
        ],
      },
      footer: {
        style: "dark",
        copyright: `Copyright © ${new Date().getFullYear()} git-sumi`,
      },
      prism: {
        theme: prismThemes.oneLight,
        darkTheme: prismThemes.oceanicNext,
        additionalLanguages: ["toml", "bash"],
      },
      colorMode: {
        defaultMode: "light",
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
    }),
};

export default config;
