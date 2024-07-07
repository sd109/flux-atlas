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

test('Compact toggle alters grid layout', async ({ page }) => {
	await page.goto('/');
	let gridClass = await page.getByTestId('resources').getAttribute('class');
	expect(gridClass?.includes('md:grid-cols-2')).toBe(true);

	await page.getByText('Compact View').click({ delay: 500 });

	gridClass = await page.getByTestId('resources').getAttribute('class');
	expect(gridClass?.includes('md:grid-cols-2')).toBeFalsy();
});
