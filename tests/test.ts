import { expect, test } from '@playwright/test';
import * as k8s from '@kubernetes/client-node';
import type { GitRepository } from '@kubernetes-models/flux-cd/source.toolkit.fluxcd.io/v1';
import { parse } from 'yaml';
import fs from 'fs';

function randomString(length: number) {
	const characters = 'abcdefghijklmnopqrstuvwxyz';
	let result = '';
	for (let i = 0; i < length; ++i) {
		result += characters[Math.floor(Math.random() * characters.length)];
	}
	return result;
}

async function setup() {
	const kc = new k8s.KubeConfig();
	kc.loadFromDefault();

	const client = kc.makeApiClient(k8s.CoreV1Api);
	const ns = 'flux-atlas-test-' + randomString(6);
	const obj = new k8s.V1Namespace();
	obj.metadata = { name: ns };
	await client.createNamespace({ body: obj });

	const fluxClient = kc.makeApiClient(k8s.CustomObjectsApi);
	return { ns, fluxClient };
}

async function teardown(ns: string) {
	const kc = new k8s.KubeConfig();
	kc.loadFromDefault();
	const client = kc.makeApiClient(k8s.CoreV1Api);
	await client.deleteNamespace({ name: ns });
}

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

test('Create and then delete GitRepo source', async ({ page }) => {
	test.slow();

	const { ns, fluxClient } = await setup();

	const file = fs.readFileSync('./tests/manifests/git-repo.yml', 'utf-8');
	const resource: GitRepository = await parse(file);
	resource.metadata!.namespace = ns;
	const [group, version] = resource.apiVersion.split('/');
	await fluxClient.createNamespacedCustomObject({
		plural: 'gitrepositories',
		group,
		version,
		namespace: ns,
		body: resource
	});

	await page.goto('/');
	// await page.getByRole('button', { name: 'Refresh' }).click({ delay: 500 });

	const label = `${ns}/${resource.metadata?.name}`;

	// Test that clicking on the resource displays a yaml modal
	const resourceItem = page.getByRole('button', { name: label });
	await expect(resourceItem).toBeVisible();
	await resourceItem.click({ delay: 500 });
	await expect(page.locator('.multiline')).toBeVisible();

	await page.reload();
	await page.waitForTimeout(3000);
	await resourceItem.getByText('Reconciling').waitFor();
	await page.waitForTimeout(3000);
	await page.reload();
	// await page.getByRole('button', { name: 'Refresh' }).click({ delay: 500 });
	// await page.getByRole('button', { name: 'Refresh' }).waitFor();
	await resourceItem.getByText('Ready').waitFor();

	await teardown(ns);

	for (let i = 0; i < 10; i++) {
		await page.waitForTimeout(3000);
		await page.getByRole('button', { name: 'Refresh' }).click({ delay: 500 });
		const visible = await page.getByText(label).isVisible();
		if (!visible) {
			break;
		}
	}

	await expect(page.getByText(label)).not.toBeVisible();
});
