import { expect, test } from '@playwright/test';

test('Basic overview page functionality', async ({ page }) => {
	await page.goto('/');
	await page.getByTestId('resources').waitFor();
	const resources = [
		'GitRepository',
		'OCIRepository',
		'HelmRepository',
		'HelmChart',
		'HelmRelease',
		'Kustomization'
	];
	resources.map(async (h) => await expect(page.getByText(h)).toBeVisible());
});

test('Clicking on resource displays resource yaml', async ({ page }) => {
	await page.goto('/');
	const resourceItem = page.getByRole('button', { name: 'flux-system/podinfo' });
	await expect(resourceItem).toBeVisible();
	await resourceItem.click();
	await expect(page.locator('.multiline')).toBeVisible();
});

test('Compact toggle alters grid layout', async ({ page }) => {
	await page.goto('/');
	// Check that default compact layout has reactive column classes
	let grid = page.getByTestId('resources');
	let gridClasses = (await grid.getAttribute('class'))
		?.split(' ')
		.filter((c) => c.includes('grid-cols-'));
	expect(gridClasses?.length).toBeGreaterThan(1);
	// Disable compact view
	const toggle = page.getByText('Compact View');
	await toggle.click();
	// Check that grid is single column only
	grid = page.getByTestId('resources');
	gridClasses = (await grid.getAttribute('class'))
		?.split(' ')
		.filter((c) => c.includes('grid-cols-'));
	expect(gridClasses).toEqual(['grid-cols-1']);
});
