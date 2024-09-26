module.exports = require('@neon-rs/load').proxy({
  platforms: {
    // 'win32-x64-msvc': () => require('../platforms/win32-x64-msvc/index.node'), // windows is not working
    'darwin-x64': () => require('../platforms/darwin-x64/index.node'),
    'darwin-arm64': () => require('../platforms/darwin-arm64/index.node'),
    'linux-x64-gnu': () => require('../platforms/linux-x64-gnu/index.node'),
    'linux-arm64-gnu': () => require('../platforms/linux-arm64-gnu/index.node')
  },
  // debug: () => require('../index.node')
});

