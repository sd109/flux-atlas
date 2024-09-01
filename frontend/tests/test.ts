import { expect, test } from '@playwright/test';

test('Basic navigation functionality', async ({ page }) => {
	await page.goto('/');
	const resources = [
		'GitRepos',
		'OCIRepos',
		'HelmRepos',
		'HelmCharts',
		'HelmReleases',
		'Kustomizations'
	];
	for (const heading of resources) {
		// Check that card is visible
		const gitRepos = page.getByText(heading);
		await expect(gitRepos).toBeVisible();
		// Check that card links to details page
		await gitRepos.locator('..').click();
		const urlPattern = new RegExp(`.*${heading.toLowerCase()}`);
		await expect(page).toHaveURL(urlPattern);
		// Check that nav bar shows correct title on details page
		const navbar = page.getByTestId('navbar');
		await expect(navbar.getByText(heading)).toBeVisible();
		// Check that clicking Flux CD element links back to home
		await navbar.getByText('Flux CD').click();
		await expect(page).toHaveURL('/');
	}
});

test('Helm release to chart navigation', async ({ page }) => {
	const home = '/helmreleases';
	await page.goto(home);
	const cards = page.getByTestId('resource-details-card');
	// Check that at least 1 card is present
	const card = cards.first();
	await expect(card).toBeVisible();
	for (const card of await cards.all()) {
		let chartLink = card.getByText(/Chart:/).getByRole('link');
		let chartTitle = await chartLink.textContent();
		await chartLink.click();
		// Check that chart link works
		await expect(page).toHaveURL(`/helmcharts#${chartTitle}`);
		// Check that a chart card with expected heading exists
		const chartCardTitle = page
			.getByTestId('resource-details-card')
			.getByRole('heading')
			.getByText(chartTitle!);
		await expect(chartCardTitle).toBeVisible();
		// Heading's parent should be card with matching ID
		await expect(chartCardTitle.locator('..')).toHaveId(chartTitle!);
		await page.goto(home);
	}
});

test('Helm chart to source navigation', async ({ page }) => {
	const home = '/helmcharts';
	await page.goto(home);
	const cards = page.getByTestId('resource-details-card');
	// Check that at least 1 card is present
	const card = cards.first();
	await expect(card).toBeVisible();
	for (const card of await cards.all()) {
		let sourceLink = card.getByText(/Source:/).getByRole('link');
		let sourceTitle = await sourceLink.textContent();
		await sourceLink.click();
		// Check that source link works
		await expect(page).not.toHaveURL(home);
		await expect(page).toHaveURL(new RegExp(`#${sourceTitle}`));
		// Check that a chart card with expected heading exists
		const sourceCardTitle = page
			.getByTestId('resource-details-card')
			.getByRole('heading')
			.getByText(sourceTitle!);
		await expect(sourceCardTitle).toBeVisible();
		// Heading's parent should be card with matching ID
		await expect(sourceCardTitle.locator('..')).toHaveId(sourceTitle!);
		await page.goto(home);
	}
});

test('Kustomization to source navigation', async ({ page }) => {
	// TODO: Write a test here
	const home = '/kustomizations';
	await page.goto(home);
	const cards = page.getByTestId('resource-details-card');
	// Check that at least 1 card is present
	const card = cards.first();
	await expect(card).toBeVisible();
	for (const card of await cards.all()) {
		let sourceLink = card.getByText(/Source:/).getByRole('link');
		let sourceTitle = await sourceLink.textContent();
		await sourceLink.click();
		// Check that source link works
		await expect(page).not.toHaveURL(home);
		await expect(page).toHaveURL(new RegExp(`#${sourceTitle}`));
		// Check that a chart card with expected heading exists
		const sourceCardTitle = page
			.getByTestId('resource-details-card')
			.getByRole('heading')
			.getByText(sourceTitle!);
		await expect(sourceCardTitle).toBeVisible();
		// Heading's parent should be card with matching ID
		await expect(sourceCardTitle.locator('..')).toHaveId(sourceTitle!);
		await page.goto(home);
	}
});
