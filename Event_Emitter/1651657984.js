class EventEmitter {
    constructor() {
        this.events = new Map(); // Map of eventName -> array of {callback, id}
        this.idCounter = 0;      // Global subscription ID tracker
        this.subscriptions = []; // Array to map subscription index to eventName + id
    }

    /**
     * Subscribe to an event with a callback
     * @param {string} eventName
     * @param {Function} callback
     * @return {Object}
     */
    subscribe(eventName, callback) {
        if (!this.events.has(eventName)) {
            this.events.set(eventName, []);
        }

        const id = this.idCounter++;
        this.events.get(eventName).push({ callback, id });

        const unsubscribe = () => {
            const listeners = this.events.get(eventName);
            if (listeners) {
                const idx = listeners.findIndex(l => l.id === id);
                if (idx !== -1) {
                    listeners.splice(idx, 1);
                }
            }
        };

        this.subscriptions.push({ eventName, id });

        return { unsubscribe };
    }

    /**
     * Emit an event and return results of each callback
     * @param {string} eventName
     * @param {Array} args
     * @return {Array}
     */
    emit(eventName, args = []) {
        const listeners = this.events.get(eventName);
        if (!listeners) return [];

        return listeners.map(listener => listener.callback(...args));
    }
}
