import type { NextConfig } from "next";

const nextConfig: NextConfig = {
    webpack(config, { isServer, dev, webpack }) {

        // Since Webpack 5 doesn't enable WebAssembly by default, we should do it manually
        config.experiments = { ...config.experiments, asyncWebAssembly: true }

        // https://nextjs.org/docs/app/building-your-application/optimizing/memory-usage#disable-webpack-cache
        // This just stops building altogether:
        // if (config.cache && !dev) {
        //     config.cache = Object.freeze({
        //         type: 'memory',
        //     })
        // }

        // Deubbing (vercel/next.js/issues/27650)
        config.infrastructureLogging = { debug: /PackFileCache/ }

        return config
    },
};

export default nextConfig;
