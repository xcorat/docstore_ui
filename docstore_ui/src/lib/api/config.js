/**
 * @typedef {Object} ApiConfig
 * @property {string} baseUrl - The base URL for all API requests
 */

/** @type {ApiConfig} */
export const config = {
    // TODO: Get this from the URL
    baseUrl: 'http://localhost:5173',
};

/**
 * Creates a full URL for an API endpoint
 * @param {string} endpoint - The endpoint path
 * @returns {string} The full URL
 */
export function createUrl(endpoint) {
    return `${config.baseUrl}/api${endpoint}`;
}