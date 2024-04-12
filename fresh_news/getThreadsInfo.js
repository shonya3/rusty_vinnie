//@ts-check

/**
 * Represents a news thread with information about the date it was posted in ISO format and its URL.
 * @typedef {{
 *  postedDateISO: string; // The date and time the thread was posted in ISO 8601 format.
 *  url: string; // The URL of the news thread.
 *  title: string
 * }} NewsThreadInfo
 */

/**
 * Retrieves information about threads from the first page of the subforum.
 * @returns {NewsThreadInfo[]} An array of objects containing information about news threads,
 *                              including the date they were posted in ISO format and their URLs.
 */
function getThreadsInfo() {
	/** @type {NodeListOf<HTMLTableRowElement>} */
	const rows = document.querySelectorAll('table tbody tr');

	return Array.from(rows).map(
		/** @returns {NewsThreadInfo} */ row => ({
			title: getThreadTitle(row),
			url: getThreadUrl(row),
			postedDateISO: postedDateISO(row),
		})
	);
}

/**
 * Retrieves the date and time a forum thread was posted in ISO 8601 format based on the provided HTML table row element.
 * @param {HTMLTableRowElement} tr - The HTML table row element representing the forum thread.
 * @returns {string} - The date and time the forum thread was posted in ISO 8601 format.
 */
function postedDateISO(tr) {
	const postDate = tr.querySelector('.post_date');
	if (!postDate || !postDate.textContent) {
		throw new Error('No post date');
	}

	// You can't construct the Date from ru localestring, need to parse it first (example: "26 марта 2024 г., 5:10:44")
	if (document.querySelector('html')?.lang === 'ru-RU') {
		const dateString = postDate.textContent;
		// Split the date string and extract day, month, year, hour, minute, second
		const parts = dateString.match(/(\d{1,2})\s+(.*?)\s+(\d{4})\s+г\.,\s+(\d{1,2}):(\d{2}):(\d{2})/);
		if (!parts) {
			throw new Error('Could not parse RU date string');
		}

		// Extracted parts
		const day = parseInt(parts[1]);
		const monthPart = parts[2];
		if (typeof monthPart !== 'string') {
			throw new Error(`Could not parse month part: ${monthPart} in dateString: ${dateString}`);
		}
		const lowercasedMonthPart = monthPart.toLowerCase();
		const month = ['янв', 'фев', 'мар', 'апр', 'ма', 'июн', 'июл', 'авг', 'сен', 'окт', 'ноя', 'дек'].findIndex(m =>
			lowercasedMonthPart.startsWith(m)
		);
		const year = parseInt(parts[3]);
		const hour = parseInt(parts[4]);
		const minute = parseInt(parts[5]);
		const second = parseInt(parts[6]);

		try {
			return new Date(year, month, day, hour, minute, second).toISOString();
		} catch (err) {
			throw new Error(`Could not parse Ru date string ${err?.message}`);
		}
	}

	const date = new Date(postDate.textContent);
	return date.toISOString();
}

/**
 * Retrieves the URL of a forum thread based on the provided HTML table row element.
 * @param {HTMLTableRowElement} tr - The HTML table row element representing the forum thread.
 * @returns {string} - The URL of the forum thread.
 */
function getThreadUrl(tr) {
	const href = tr.querySelector('.title')?.querySelector('a')?.href;
	if (!href) {
		throw new Error('Error occurred when extracting the thread URL.');
	}
	return href;
}

/**
 * Retrieves the title of a forum thread based on the provided HTML table row element.
 * @param {HTMLTableRowElement} tr - The HTML table row element representing the forum thread.
 * @returns {string} - The URL of the forum thread.
 */
function getThreadTitle(tr) {
	return tr.querySelector('.title a')?.textContent?.trim() ?? '';
}
