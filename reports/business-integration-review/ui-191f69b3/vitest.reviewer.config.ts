import config from "./vitest.config"

// Reviewer cache belongs to this archive, never the reused installed packages.
export default { ...config, cacheDir: ".reviewer-vite" }
