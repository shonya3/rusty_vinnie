//@ts-check

/**
 * Represents a news thread with information about the date it was posted in ISO format and its URL.
 * @typedef {{
 *  postedDateISO: string; // The date and time the thread was posted in ISO 8601 format.
 *  url: string; // The URL of the news thread.
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
		/** @returns {NewsThreadInfo} */ row => ({ url: getThreadUrl(row), postedDateISO: postedDateISO(row) })
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
