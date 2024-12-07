/**
 * @typedef {Object} Client
 * @property {number} client_id
 * @property {string} first_name
 * @property {string} last_name
 * @property {string} social_security_number
 * @property {string} address
 * @property {string} phone_number
 * @property {string} email
 * @property {string} created_at
 * @property {string} updated_at
 */

/**
 * Custom error class for API errors
 * @extends Error
 */
export class ApiError extends Error {
    /**
     * @param {string} message
     * @param {number} status
     */
    constructor(message, status) {
        super(message);
        /** @type {string} */
        this.name = 'ApiError';
        /** @type {number} */
        this.status = status;
    }
}
