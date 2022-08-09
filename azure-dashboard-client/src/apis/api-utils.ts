import { warn } from '../components/toast'

enum Verb {
    GET = 'get',
    // PUT = 'put',
    // POST = 'post',
    // DELETE = 'delete',
    // PATCH = 'patch'
}

/**
 * A standard method for calling an action (GET, POST, etc) on an URL.
 * @param {string} accessToken The authenticated user's access token.
 * @param {string} url The relative URL.
 * @param {string} method The method (GET, POST, etc.)
 * @param {any} message (Optional) The object to go in the request body as JSON.
 * @param {string} contentType (Optional) The content type
 */
const authorizedActionAsync = async <T>(method: Verb, accessToken: string, url: string, message?: any, contentType = 'application/json'): Promise<T> => {
    // console.log(`Verb ${method} - url - ${url}`)
    return fetch(url, {
        method,
        headers: {
            Authorization: `Bearer ${accessToken}`,
            'Content-Type': `${contentType}; charset=utf-8`,
            'Accept-Language': navigator.language
        },
        cache: 'no-cache',
        body: message ? JSON.stringify(message) : null
    })
        .then(async response => {
            if (response.ok) {
                // Get the content type
                const responseContentType = response.headers.get('content-type')
                // If JSON...
                if (responseContentType && -1 !== responseContentType.indexOf('application/json')) {
                    // Return parsed JSON
                    return response.json()
                }
                // Return text
                return response.text()
            }
            const text = await response.text()
            const message = text ? `${response.status} ${response.statusText} - ${text}` : `${response.status} ${response.statusText}`
            throw new Error(message)
        })
        .then(value => {
            return value as T
        })
}

/**
 * A standard GET request.
 * @param {string} accessToken The authenticated user's access token.
 * @param {string} url The URL.
 * @returns
 */
export const authorizedGetAsync = async <T>(accessToken: string, url: string): Promise<T> => authorizedActionAsync<T>(Verb.GET, accessToken, url)

/**
 * Shows an error with a title.
 * @param title
 */
export const showError = (title: string) => {
    return (error: Error): void => {
        // Show the error
        warn(title + '\r\n\r\n' + error.toString())
        // Log the error
        console.warn(error)
    }
}
