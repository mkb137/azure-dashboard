import * as lodash from 'lodash'
import { writable } from 'svelte/store'

export enum ToastType {
    Default,
    Warning
}
// The data for a toast pop-up.
export type ToastItem = {
    // The ID
    id: string
    // The toast message
    message: string
    // The toast type
    toastType: ToastType
}

// The array of our toast items.
const toastItems = new Array<ToastItem>()

// THe writable store of our toast items.
export const toasts = writable<ToastItem[]>(toastItems)

// Adds a toast item.
export const toast = (message: string): void => {
    // console.log(`toast - message = ${message}`)
    toastItems.push({
        id: lodash.uniqueId('toast'),
        toastType: ToastType.Default,
        message
    })
    toasts.set(toastItems)
}

// Adds an warning toast item
export const warn = (message: string): void => {
    // console.log(`toast - message = ${message}`)
    toastItems.push({
        id: lodash.uniqueId('toast'),
        toastType: ToastType.Warning,
        message
    })
    toasts.set(toastItems)
}

// Removes a toast item.
export const removeToast = (item: ToastItem): void => {
    const index = toastItems.indexOf(item)
    if (0 <= index) {
        // console.log(` - removing toast at ${index}`)
        toastItems.splice(index, 1)
        // console.log(` - we now have ${toastItems.length} items`)
        toasts.set(toastItems)
    }
}
