import { defineConfig } from "vitest/config";
import react from "@vitejs/plugin-react";
import path from "path";

export default defineConfig({
	plugins: [react()],
	test: {
		environment: "jsdom",
		globals: true,
		setupFiles: ["./vitest.setup.ts"],
		include: ["**/__tests__/**/*.test.{ts,tsx}", "**/*.test.{ts,tsx}"],
		exclude: [
			"node_modules",
			".next",
			"dist",
			// These tests use bun:test native API and must be run with `bun test`, not vitest
			"lib/hooks/__tests__/timeline-reconnection.test.ts",
			"lib/hooks/__tests__/server-poll-logic.test.ts",
			"lib/hooks/__tests__/window-focus-refresh.test.ts",
			"lib/hooks/__tests__/timeline-ui-issues.test.ts",
			"lib/hooks/__tests__/server-push-old-frames.test.ts",
			"lib/hooks/__tests__/timeline-store-logic.test.ts",
			"lib/utils/meeting-state.test.ts",
			"lib/utils/redact-pii.test.ts",
			"lib/__tests__/team-api-contract.test.ts",
			"lib/__tests__/team-crypto.test.ts",
			"components/__tests__/url-detection-benchmark.test.ts",
		],
		env: {
			NODE_ENV: "test",
		},
	},
	resolve: {
		alias: {
			"@": path.resolve(__dirname, "./"),
		},
	},
});
