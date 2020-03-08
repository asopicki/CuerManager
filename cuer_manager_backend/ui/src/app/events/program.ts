import {Event} from './event';
import { DateTime } from 'luxon';

export class Program {
    id: number;
    uuid: String;
    notes: String;
    event: Event;
    date_modified: String | Date

    constructor(data: any, event: Event) {
        Object.assign(this, data);
        this.event = event;
        this.date_modified = DateTime.fromISO(data.date_modified).toJSDate();
    }
}

export type OptionalProgram = Program | null;
