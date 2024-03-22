//@ts-check

/**
 * Represents a news thread with information about how many minutes ago it was posted and its URL.
 * @typedef {{
 *  postedMinutesAgo: number;
 *  url: string;
 * }} NewsThreadInfo
 */

/**
 * Retrieves information about threads from the first page of the subforum.
 * @returns {NewsThreadInfo[]} An array of objects containing information about news threads,
 *                              including the number of minutes ago they were posted and their URLs.
 */
function getThreadsInfo() {
	/** @type {NodeListOf<HTMLTableRowElement>} */
	const rows = document.querySelectorAll('table tbody tr');

	return Array.from(rows).map(
		/** @returns {NewsThreadInfo} */ row => ({ url: getThreadUrl(row), postedMinutesAgo: postedMinutesAgo(row) })
	);
}

/**
 * Calculates the number of minutes ago a forum thread was posted based on the provided HTML table row element.
 * @param {HTMLTableRowElement} tr - The HTML table row element representing the forum thread.
 * @returns {number} - The number of minutes ago the thread was posted.
 */
function postedMinutesAgo(tr) {
	const postDate = tr.querySelector('.post_date');
	if (!postDate || !postDate.textContent) {
		throw new Error('No post date');
	}

	const date = new Date(postDate.textContent);
	return (Date.now() - date.getTime()) / 1000 / 60;
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
