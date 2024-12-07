import { createUrl } from './config';
import { ApiError } from './types';

// Import the Client type for JSDoc
/** @typedef {import('./types').Client} Client */

/**
 * Fetches all clients from the backend
 * @returns {Promise<Client[]>}
 * @throws {ApiError}
 */
export async function listClients() {
    try {
        const response = await fetch(`${createUrl('/clients')}`);
        if (!response.ok) {
            throw new ApiError(`HTTP error! status: ${response.status}`, response.status);
        }
        return await response.json();
    } catch (error) {
        if (error instanceof ApiError) {
            console.error('Error fetching clients:', error);
            throw error;
        }
        console.error('Error fetching clients:', error);
        throw new ApiError('Unknown error', 500);
    }
}

/**
 * Fetches a single client by ID
 * @async
 * @param {number} clientId - The ID of the client to fetch
 * @returns {Promise<Client>} A promise that resolves to a single client
 * @throws {ApiError} If the client is not found (404) or other server errors
 */
export async function getClient(clientId) {
    try {
        const response = await fetch(`${createUrl(`/clients/${clientId}`)}`);
        if (!response.ok) {
            if (response.status === 404) {
                throw new ApiError('Client not found', 404);
            }
            throw new ApiError(`HTTP error! status: ${response.status}`, response.status);
        }
        return await response.json();
    } catch (error) {
        console.error(`Error fetching client ${clientId}:`, error);
        throw error;
    }
}

/** @typedef {{ name: string, path: string }} ClientFile */

/**
 * Fetches all files for a specific client
 * @param {string} clientId - The ID of the client whose files to fetch
 * @returns {Promise<String[]>} A promise that resolves to an array of client files
 * @throws {ApiError} If the client is not found (404) or other server errors
 */
export async function listClientFiles(clientId) {
    try {
        const response = await fetch(`${createUrl(`/clients/${clientId}/files`)}`);
        if (!response.ok) {
            if (response.status === 404) {
                throw new ApiError('Client not found', 404);
            }
            throw new ApiError(`HTTP error! status: ${response.status}`, response.status);
        }
        console.log('response', response);
        return await response.json();
    } catch (error) {
        console.error(`Error fetching files for client ${clientId}:`, error);
        if (error instanceof ApiError) {
            throw error;
        }
        throw new ApiError('Unknown error', 500);
    }
}
