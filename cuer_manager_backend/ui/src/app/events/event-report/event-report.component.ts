import { Component, OnInit } from '@angular/core';
import { Observable, of, EMPTY } from 'rxjs';
import { mergeMap, map } from 'rxjs/operators';
import { ActivatedRoute } from '@angular/router';

import { EventService } from '../event.service';
import { ProgramService } from '../program.service';
import { TipService } from '../tip.service';
import { Tip } from '../tip';
import { Event } from '../event';
import { Cuecard } from '../cuecard';
import { Program } from '../program';
import { TipCuecard } from '../tip-cuecard';

const urls = {
  'cuecard_data': '/cuecard/'
};

class CuedChoreo {
  constructor(public cuecard: Cuecard, public tip: Tip) {}
}

@Component({
  selector: 'app-event-report',
  templateUrl: './event-report.component.html',
  styleUrls: ['./event-report.component.scss']
})
export class EventReportComponent implements OnInit {

  event: Event;
  tips: Tip[];
  loading: boolean;

  constructor(
    private route: ActivatedRoute,
    private eventService: EventService, 
    private programService: ProgramService, 
    private tipService: TipService,
  ) { }

  ngOnInit() {
    this.tips = [];
    this.loading = true
    this.route.paramMap.subscribe(params => this._getEvent(params.get('uuid') || '').subscribe(event => {
      this.event = event;
      this.loading = false;
    }))
  }

  _getEvent(uuid: string): Observable<Event> {
    let result = null
  
    if (!uuid) {
        return EMPTY
    }

    return this.eventService.getEvent(uuid).pipe(
      mergeMap(event => {
          if (event) {
            result = event
            this.programService.getProgram(event).pipe(
                  map(program => {
                      result.setProgram(program);
                      if (program) {
                          this.tipService.getTips(program).subscribe(tips => { 
                            if (tips) {
                              result.setTips(tips);
                              this.tips = tips;
                            }
                          });
                      }
                      return result;
                  })
            ).subscribe(_ => {})
            return of(result);
          } else {
             return of(null);
          }
    }));
  }

  plusFigures(cuecard: Cuecard): String {
    return cuecard.meta['plusfigures'];
  }

  program(): Program {
    if (this.event) {
      return this.event.getProgram();
    }

    return null;
  }

  cuedChoreos(): CuedChoreo[] {
    let result: CuedChoreo[] = [];
    let previousTip: Tip;
    this.tips.forEach(tip => {
      tip.cuecards.forEach(cuecard => {
        if (this.cuedAt(tip, <Cuecard>cuecard)) {
          result.push(new CuedChoreo(<Cuecard>cuecard, tip))
        }       
      });
      previousTip = tip;
    })
  
    return result;
  }

  cuedAt(tip: Tip, cuecard: Cuecard): boolean {

    let i = tip.cuecards.findIndex(c => (<Cuecard>c).uuid === cuecard.uuid);

    if (i >= 0 && tip.tip_cuecards[i] && (<TipCuecard>tip.tip_cuecards[i]).cued_at) {
      return true
    }

    return false
  }
}