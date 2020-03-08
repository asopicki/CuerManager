export class TipCuecard {
    id: number
    tip_id: number
    cuecard_id: number
    sort_order: number
    cued_at: String|null

    constructor(data: Object) {
        Object.assign(this, data);
    } 
}