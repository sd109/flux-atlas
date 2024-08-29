import { expect, test } from '@playwright/test';

test('Basic overview page functionality', async ({ page }) => {
	await page.goto('/');
	await page.getByTestId('resources').waitFor();
	const resources = [
		'GitRepos',
		'OCIRepos',
		'HelmRepos',
		'HelmChart',
		'HelmRelease',
		'Kustomization'
	];
	await Promise.all(resources.map(async (h) => await expect(page.getByText(h)).toBeVisible()));
});
