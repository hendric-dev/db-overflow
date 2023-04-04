import {defineConfig} from 'vitepress'

export default defineConfig({
  title: 'DB Overflow',
  base: '/db-overflow/',
  lang: 'en-US',
  srcExclude: ['README.md'],
  themeConfig: {
    sidebar: [
      {text: 'Get Started', link: '/get-started'},
      {text: 'Customization', link: '/customization'},
    ],
  },
})
