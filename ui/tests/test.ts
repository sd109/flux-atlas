import { expect, test } from '@playwright/test';
import { parse } from 'yaml';

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

test('Yaml details modal', async ({ page }) => {
	for (const resource of [
		'gitrepos',
		'ocirepos',
		'helmrepos',
		'helmcharts',
		'helmreleases',
		'kustomizations'
	]) {
		await page.goto('/' + resource);
		const cards = await page.getByTestId('resource-details-card').all();
		// Make sure we have at least 1 resource to test
		expect(cards.length).toBeGreaterThan(0);
		for (const card of cards) {
			// Trigger modal
			await card.getByRole('button').getByText('Details').click();
			// Check modal content
			const modal = page.getByTestId('details-modal');
			await expect(modal.getByText('Resource Details')).toBeVisible();
			let text = await modal.getByText('apiVersion').textContent();
			expect(text).not.toBeNull();
			const yaml = parse(text!);
			expect(yaml.kind.toLowerCase()).toContain(resource.slice(0, resource.length - 1));
			expect(yaml.apiVersion).toContain('fluxcd.io');
			// Check that unwanted fields have been scrubbed
			expect(yaml.metadata.managedFields).toBeUndefined();
			// Close modal
			// await page.getByLabel('Close modal').click();
			await page.keyboard.press('Escape');
			await expect(modal).not.toBeVisible();
		}
	}
});

test('Flux controller logs modal', async ({ page }) => {
	// Should match id var in LogsModal component
	const modalId = '#logs-lines-ul-element';

	await page.goto('/');
	await page.getByText('Flux Logs').click();
	await expect(page.getByText('No logs')).not.toBeVisible();

	// Check that we get a non-zero line count
	let logCountText = page.getByText('matched lines');
	await expect(logCountText).toBeVisible();
	let count = (await logCountText.textContent())?.match(/^\d+/)?.join('');
	expect(parseInt(count!)).toBeGreaterThan(0);

	// Check for duplicate log lines which could indicate
	// errors in lifecycle logic of server-side event sources
	let logsElement = page.locator(modalId);
	await expect(logsElement).toBeVisible();
	let logLines = await logsElement.getByRole('listitem').allTextContents();
	let uniqueLogLines = [...new Set(logLines)];
	expect(uniqueLogLines.length).toEqual(logLines.length);

	// Check that closing modal works
	// await page.getByLabel('Close modal').click();
	await page.keyboard.press('Escape');
	await expect(logsElement).not.toBeVisible();
});
