import {Program} from './program';
import { Tip } from './tip';
import { DateTime } from 'luxon';

export class Event {
    id: number;
    uuid: String;
    date_start: Date;
    date_end: Date;
    name: String;
    program: Program | null;
    tips: Tip[];

    constructor(data: any) {
        Object.assign(this, data);
        this.date_start = DateTime.fromISO(data.date_start).toJSDate();
        this.date_end = DateTime.fromISO(data.date_end).toJSDate();
        this.tips = [];
    }

    public getId(): number {
        return this.id;
    }

    public getProgram(): Program | null {
        return this.program
    }

    public setProgram(program: Program|null) {
        this.program = program
    }

    public getTips(): Tip[] {
        return this.tips
    }

    public setTips(tips: Tip[]) {
        this.tips = tips
    }
}
