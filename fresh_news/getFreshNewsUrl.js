/**
 * Extracts the URL of the latest non-sticky thread from the forum announcements.
 * @returns {Promise<string>} - URL of the latest post.
 */
async function getFreshNewsUrl() {
	const rows = document.querySelectorAll('table tbody tr');

	for (const row of rows) {
		const [td_first, td_title] = Array.from(row.querySelectorAll('td'));
		const stickyThread = Boolean(td_first.querySelector('.sticky'));
		if (!stickyThread) {
			const href = td_title.querySelector('.title a').href;
			if (!href) {
				throw new Error('Error occurred when extracting the title URL.');
			}
			return href;
		}
	}
}

/**
 * Goes to the forum announcements and extracts the URL of the latest non-sticky thread.
 * @returns {Promise<string>} - URL of the latest post.
 */
