import { defineConfig } from "vitest/config"

export default defineConfig({
  test: {
    environment: "node",
    include: ["integrations/pi-desk/**/*.test.ts"],
    testTimeout: 15000,
    hookTimeout: 15000,
  },
})
