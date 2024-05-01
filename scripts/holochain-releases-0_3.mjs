import https from 'node:https'

const URL = 'https://api.github.com/repos/holochain/holochain/releases'
const request = {
		headers: {'User-Agent': 'scaffolding-holochain-releases'},
}

function n_hours_ago(n) {
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
					const holochain_0_3 = result
						.filter(r => r.tag_name.startsWith('holochain-0.3'))
						.filter(r => new Date(r.published_at) >= n_hours_ago(6))
						.map(r => r.tag_name)
					
					if (holochain_0_3.length) {
						console.log(holochain_0_3[0])
					}
			} catch (e) {
					console.error('Error processing result', e);
			}
	});

}).on('error', (e) => {
	console.error(`Got error: ${e.message}`);
});