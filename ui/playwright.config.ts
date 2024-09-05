import { type PlaywrightTestConfig, devices } from '@playwright/test';

const config: PlaywrightTestConfig = {
	webServer: {
		command: 'npm run dev',
		port: 5173,
		reuseExistingServer: !process.env.CI,
		stdout: 'pipe',
		stderr: 'pipe'
	},
	testDir: 'tests',
	testMatch: /(.+\.)?(test|spec)\.[jt]s/,
	use: {
		testIdAttribute: 'data-testid'
	},
	timeout: 30000,
	retries: 1,
	projects: [
		/* Test against desktop browsers */
		{
			name: 'chromium',
			use: { ...devices['Desktop Chrome'] }
		},
		{
			name: 'firefox',
			use: { ...devices['Desktop Firefox'] }
		}
		// Webkit tests seem to be flaky since switching
		// to server-side fetching. Re-enable them once
		// issue has been investigated further.
		// {
		// 	name: 'webkit',
		// 	use: { ...devices['Desktop Safari'] }
		// }
	]
};

export default config;
