import { config, createUrl } from './config';
import { ApiError } from './types';

/**
 * @typedef {Object} TaxReturn
 * @property {number} [tax_return_id]
 * @property {number} client_id
 * @property {number} tax_year
 * @property {string} filing_status
 * @property {Object[]} income_sources
 * @property {Object[]} deductions
 * @property {Object[]} credits
 * @property {number} taxes_paid
 * @property {number} tax_liability
 * @property {number} refund_or_amount_due
 * @property {string} [created_at]
 * @property {string} [updated_at]
 */

/**
 * Fetches all tax returns, optionally filtered by client ID
 * @async
 * @param {number} [clientId] - Optional client ID to filter returns
 * @returns {Promise<TaxReturn[]>} A promise that resolves to an array of tax returns
 * @throws {ApiError} If the server returns an error response
 */
export async function listReturns(clientId) {
    try {
        const url = clientId 
            ? `${createUrl('/returns')}?client_id=${clientId}`
            : `${createUrl('/returns')}`;
        const response = await fetch(url);
        if (!response.ok) {
            throw new ApiError('Failed to fetch tax returns', response.status);
        }
        return await response.json();
    } catch (error) {
        if (error instanceof ApiError) throw error;
        throw new ApiError(String(error), 500);
    }
}

/**
 * Fetches a single tax return by ID
 * @async
 * @param {number} returnId - The ID of the tax return to fetch
 * @returns {Promise<TaxReturn>} A promise that resolves to a single tax return
 * @throws {ApiError} If the tax return is not found (404) or other server errors
 */
export async function getReturn(returnId) {
    try {
        const response = await fetch(`${config.baseUrl}/returns/${returnId}`);
        if (!response.ok) {
            if (response.status === 404) {
                throw new ApiError('Tax return not found', 404);
            }
            throw new ApiError(`Failed to fetch tax return ${returnId}`, response.status);
        }
        return await response.json();
    } catch (error) {
        if (error instanceof ApiError) throw error;
        throw new ApiError(String(error), 500);
    }
}