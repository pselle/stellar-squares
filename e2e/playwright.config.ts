import { defineConfig, devices } from "@playwright/test";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const appDir = resolve(dirname(fileURLToPath(import.meta.url)), "../app");
const port = 5180;

export default defineConfig({
  testDir: "./tests",
  reporter: "list",
  forbidOnly: !!process.env.CI,
  // The gallery tests read from testnet, so allow for a slow or flaky RPC in CI
  retries: process.env.CI ? 2 : 0,
  use: {
    ...devices["Desktop Chrome"],
    baseURL: `http://localhost:${port}`,
  },
  webServer: {
    // Run only Vite (not `npm start`, which also starts the contract watcher).
    // Staging mode loads app/.env.staging, so the app talks to testnet; the
    // contract clients must have been generated for staging first.
    command: `npx vite --mode staging --port ${port} --strictPort`,
    cwd: appDir,
    url: `http://localhost:${port}`,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
