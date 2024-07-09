//@ts-check

// eslint-disable-next-line @typescript-eslint/no-var-requires
const { composePlugins, withNx } = require('@nx/next');

/**
 * @type {import('@nx/next/plugins/with-nx').WithNxOptions}
 **/
const nextConfig = {
  webpack: (config) => {
    config.externals = [
      ...(config.externals || []),
      'bigint',
      'node-gyp-build',
    ];
    return config;
  },
  nx: {
    // Set this to true if you would like to use SVGR
    // See: https://github.com/gregberge/svgr
    svgr: false,
  },
  experimental: {
    serverComponentsExternalPackages: ["@coral-xyz/anchor"],
  },
  reactStrictMode: true,
  output:'export',
  // output: 'export', // Added here as part of the nextConfig object
  // any other configurations you have
};

const plugins = [
  // Add more Next.js plugins to this list if needed.
  withNx,
];

module.exports = composePlugins(...plugins)(nextConfig);