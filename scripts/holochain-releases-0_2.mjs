#!/usr/bin/env node

import https from 'node:https'

const URL = 'https://api.github.com/repos/holochain/holochain/releases'
const request = {
	headers: { 'User-Agent': 'scaffolding-holochain-releases' },
}

function nHoursAgo(n) {
	return new Date(new Date().getTime() - (n * 60 * 60 * 1000))
}

https.get(URL, request, (res) => {
	let data = '';

	res.on('data', (chunk) => {
		data += chunk;
	});

	res.on('end', () => {
		try {
			const result = JSON.parse(data);
			const holochain_0_2 = result
				.filter(r => r.tag_name.startsWith('holochain-0.2'))
				.filter(r => new Date(r.published_at) >= nHoursAgo(144))
				.map(r => r.tag_name)

			if (holochain_0_2.length) {
				console.log(holochain_0_2[0])
			}
		} catch (e) {
			console.error('Error processing result', e);
		}
	});

}).on('error', (e) => {
	console.error(`Got error: ${e.message}`);
});