import { defineConfig } from 'vite';
import solid from 'vite-plugin-solid';
import { VitePWA } from 'vite-plugin-pwa';

export default defineConfig({
  plugins: [
    solid(),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: true
      },
      manifest: {
        short_name: 'PongElo',
        name: 'PongElo',
        start_url: '.',
        display: 'standalone',
        theme_color: '#333333',
        background_color: '#333333',
        icons: [
          {
            src: '/logo.svg',
            sizes: 'any',
            type: 'image/svg+xml',
            purpose: 'any',
          }
        ]
      }
    })
  ],
});
