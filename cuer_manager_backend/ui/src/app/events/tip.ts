import { Program } from "./program";
import { Cuecard } from "./cuecard";
import {TipCuecard} from "./tip-cuecard";

export class Tip {
    uuid: String;
    name: String;
    program_uuid: String;
    date_start: Date;
    date_end: Date;
    cuecards: Array<Object|Cuecard>
    tip_cuecards: Array<Object|TipCuecard>

    constructor(data: Object, program: Program) {
        Object.assign(this, data);
        this.program_uuid = program.uuid;
        this.date_start = new Date(this.date_start);
        this.date_end = new Date(this.date_end);

        for (let i in this.cuecards) {
            this.cuecards[i] = new Cuecard(this.cuecards[i]);
        }

        for (let i in this.tip_cuecards) {
            this.tip_cuecards[i] = new TipCuecard(this.tip_cuecards[i]);
        }
    }
}
